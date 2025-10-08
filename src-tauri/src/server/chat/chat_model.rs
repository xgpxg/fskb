use crate::constant;
use crate::db::model::knowledge_base::KnowledgeBase;
use crate::db::model::model::Model;
use crate::server::mcp::default::kb_mcp::KbMcp;
use crate::server::mcp::default::{kb_mcp, DefaultMcpServer};
use anyhow::bail;
use futures_util::StreamExt;
use mcp::mcp_manager;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionFunction, ChatCompletionParametersBuilder, ChatCompletionResponseFormat,
    ChatCompletionTool, ChatCompletionToolChoice, ChatCompletionToolType, ChatMessage,
    ChatMessageContent, DeltaChatMessage, Function, ToolCall,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::LazyLock;
use std::time::Duration;

/// 向模型发起对话
/// - messages：多条消息，含可选的历史记录
/// - handler：流式消息处理器
/// - done：会话结束处理，仅成功时调用，失败时返回Error由调用方处理
pub async fn chat<F, D>(
    model: &Model,
    messages: Vec<ChatMessage>,
    tools: Vec<ChatCompletionTool>,
    handler: F,
    done: D,
) -> anyhow::Result<()>
where
    F: Fn(String) -> Pin<Box<dyn Future<Output = ()> + Send>>,
    D: Fn(String) -> Pin<Box<dyn Future<Output = ()> + Send>>,
{
    let base_url = &model.base_url.clone().unwrap();
    let api_key = model.api_key.clone().unwrap_or_default();
    let model_name = model.name.clone().unwrap();
    let mut client = Client::new(api_key);
    client.set_base_url(base_url);
    client.http_client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .build()?;

    // 构建消息，包含历史消息和最新的用户消息
    let mut messages = messages;

    // 工具调用次数，用于防止无限制的工具调用
    let mut tool_call_times = 0;

    // 需要入库的全量消息，不包含工具调用过程中的消息
    let mut full_message = Vec::new();

    loop {
        // 工具调用次数累加
        tool_call_times = tool_call_times + 1;

        // 构建参数模型请求参数
        let mut parameters = ChatCompletionParametersBuilder::default()
            // 模型名称
            .model(&model_name)
            // 最大token：8K
            //.max_tokens(8192u32)
            // 消息
            .messages(messages.clone())
            // 注意tool不能传空数组，部分模型会报400错误，比如deepseek
            // 如果没有工具则传None，但是这个builder不支持传入option，所以在构建完成后取得可变引用，对tools单独赋值
            //.tools(get_tools(&assistant))
            // 工具选择：默认使用Auto，如果调用次数超过最大值，则使用None，结束工具调用
            //注释掉，因为这个参数序列化后是null，可能会有问题
            // .tool_choice(if tool_call_times > constant::MAX_MCP_TOOL_INVOKE_DEPTH {
            //     ChatCompletionToolChoice::None
            // } else {
            //     ChatCompletionToolChoice::Auto
            // })
            // 返回格式
            .response_format(ChatCompletionResponseFormat::Text)
            // 流式调用
            .stream(true)
            .build()?;

        // 可用工具
        parameters.tools = if tools.is_empty() {
            None
        } else {
            Some(tools.clone())
        };

        log::debug!("模型调用参数：{:?}", serde_json::to_string(&parameters)?);

        // 返回流
        let mut stream = client.chat().create_stream(parameters).await?;

        // 本次需要调用的工具，可能为空，如果为空则结束循环
        // 定义为tuple，用于存储工具调用的ID、工具名称和调用参数，其中，工具名称格式为：服务名+分隔符+工具名
        let mut need_call_tools = Vec::<(String, String, String)>::new();

        // Deepseek专用：标记是否存在深度思考的过程
        let mut has_reasoning_content = false;
        // 解析SSE流
        while let Some(item) = stream.next().await {
            match item {
                Ok(item) => {
                    log::debug!(
                        "Model ChatCompletionChunkResponse: {}",
                        serde_json::to_string(&item)?
                    );
                    let msg = &item.choices[0].delta;
                    // 有些模型返回消息在Untagged，有些在Assistant，需要兼容
                    match msg {
                        DeltaChatMessage::Untagged {
                            content,
                            tool_calls,
                            reasoning_content,
                            ..
                        }
                        | DeltaChatMessage::Assistant {
                            content,
                            tool_calls,
                            reasoning_content,
                            ..
                        } => {
                            // 如果有工具返回，提取调用的工具。否则为普通问答
                            if let Some(tool_calls) = tool_calls {
                                for tool_call in tool_calls {
                                    // 工具索引
                                    let index = tool_call.index.unwrap_or(0) as usize;
                                    // 工具调用ID
                                    let tool_call_id = tool_call.id.clone().unwrap_or_default();
                                    // 使用的工具函数
                                    let function = &tool_call.function;
                                    // 函数名
                                    let function_name = function.name.clone().unwrap_or_default();
                                    // 函数参数
                                    let function_arguments =
                                        function.arguments.clone().unwrap_or_default();
                                    // 模型返回的需要调用的工具
                                    let tool = need_call_tools.get_mut(index);
                                    // 未初始化时新增一条
                                    if tool.is_none() {
                                        need_call_tools.insert(
                                            index,
                                            (
                                                tool_call_id,
                                                function_name.clone(),
                                                function_arguments,
                                            ),
                                        );
                                        /*// 推送到前端
                                        handler(format!(
                                            "<div data-tool>🔨 调用工具：{}</div>",
                                            function_name
                                        )).await;*/
                                    } else {
                                        // 存在时，追加参数值
                                        tool.unwrap().2 += &function_arguments;
                                    }
                                }
                                // 需要调用的工具的消息，提取工具后即可，无需处理消息内容
                                continue;
                            }

                            // 模型回复的消息内容
                            match content {
                                None => {}
                                Some(content) => match content {
                                    ChatMessageContent::Text(text) => {
                                        // 适配Deepseek的深度思考
                                        if has_reasoning_content {
                                            has_reasoning_content = false;
                                            full_message.push("</think>".to_string());
                                            handler("</think>".to_string()).await;
                                        }
                                        full_message.push(text.clone());
                                        handler(text.clone()).await;
                                    }
                                    ChatMessageContent::ContentPart(_) => {}
                                    ChatMessageContent::None => {}
                                },
                            }

                            // 适配Deepseek的深度思考
                            match reasoning_content {
                                None => {}
                                Some(text) => {
                                    if !has_reasoning_content {
                                        has_reasoning_content = true;
                                        full_message.push("<think>".to_string());
                                        handler("<think>".to_string()).await;
                                    }
                                    full_message.push(text.clone());
                                    handler(text.clone()).await;
                                }
                            }
                        }
                        _ => {
                            log::info!("未处理的消息: {:?}", serde_json::to_string(msg));
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Model invocation failed, model name: {}, reason: {:?}",
                        model_name,
                        e
                    );
                    let err = format!("模型调用失败，原因: {:?}", e);
                    bail!(err);
                }
            }
        }

        if !need_call_tools.is_empty() {
            log::info!(
                "Tools hit, executing round {} tool call, tool name: {:?}",
                tool_call_times,
                need_call_tools.iter().map(|x| &x.1).collect::<Vec<_>>()
            );
        }

        // 工具消息：调用工具后，组装调用结果为工具消息，即ChatMessage:Tool

        let tools = TypedTools::from_tools(need_call_tools.clone());
        let mut tool_messages = tools.call().await;

        // 在向模型返回工具调用结果消息前，需要将模型要求调用的工具消息传回给模型，主要参数是tool_call_id
        let mut tool_calls = Vec::<ToolCall>::new();
        // 组装完整的tool_calls
        for (tool_call_id, name, parameters) in need_call_tools.clone() {
            tool_calls.push(ToolCall {
                id: tool_call_id,
                r#type: "function".to_string(),
                function: Function {
                    name,
                    arguments: parameters,
                },
            })
        }
        // 组装助手消息，回传tool_calls
        let assistant_tool_message = ChatMessage::Assistant {
            content: None,
            reasoning_content: None,
            refusal: None,
            name: None,
            audio: None,
            tool_calls: Some(tool_calls),
        };

        // 拼接tool_calls
        messages.push(assistant_tool_message);

        // 拼接工具消息
        messages.extend(tool_messages.clone());

        // 清空工具调用列表，后续复用
        need_call_tools.clear();

        // 没有工具调用，结束本次对话
        if tool_messages.is_empty() {
            done(full_message.join("")).await;
            log::info!("Model invocation completed");
            break;
        }
    }

    Ok(())
}

/// 工具分类
struct TypedTools {
    /// 内置默认工具
    default: Vec<(String, String, String)>,
    /// 其他工具
    other: Vec<(String, String, String)>,
}
impl TypedTools {
    /// 从需要调用的工具构建
    ///
    /// - tools: 需要调用的工具，格式为(tool_call_id, name, parameters)
    ///     - tool_call_id: 工具调用的id，用于后续返回结果
    ///     - name: 工具的名称，格式为：mcp_server_name+分隔符+tool_name，工作流的mcp_server_name固定为workflow，tool_name为工作流ID
    ///     - parameters: 工具的参数，格式为json字符串
    fn from_tools(tools: Vec<(String, String, String)>) -> Self {
        let mut default = vec![];
        let mut other = vec![];
        for (tool_call_id, name, parameters) in tools {
            if DefaultMcpServer::is_default_mcp(&name) {
                default.push((tool_call_id, name, parameters));
            } else {
                other.push((tool_call_id, name, parameters));
            }
        }
        Self { default, other }
    }

    /// 调用工具
    async fn call(&self) -> Vec<ChatMessage> {
        let mut tool_messages = vec![];
        // 默认工具调用
        let default_tools_messages = self.invoke_default_tools().await;
        // 其他工具调用
        let other_tools_messages = self.invoke_other_tools().await;
        tool_messages.extend(default_tools_messages);
        tool_messages.extend(other_tools_messages);

        tool_messages
    }

    /// 调用知识库工具
    async fn invoke_default_tools(&self) -> Vec<ChatMessage> {
        if self.default.is_empty() {
            return vec![];
        }

        let mut tool_messages = Vec::new();

        for (tool_call_id, name, parameters) in &self.default {
            log::info!("Call tool: {}", name);
            let result = DefaultMcpServer::new(&name)
                .unwrap()
                .call(&name, &parameters)
                .await
                .unwrap_or_else(|err| {
                    log::error!("Default MCP tool call fail：{}", err);
                    "调用失败".to_string()
                });
            tool_messages.push(ChatMessage::Tool {
                content: result,
                tool_call_id: tool_call_id.clone(),
            });
        }
        tool_messages
    }

    async fn invoke_other_tools(&self) -> Vec<ChatMessage> {
        if self.other.is_empty() {
            return vec![];
        }

        let mut tool_messages = Vec::new();
        for (tool_call_id, name, parameters) in &self.other {
            if name.is_empty() {
                log::warn!("工具名称为空，无法调用");
                continue;
            }

            let split = name
                .split(constant::MCP_SERVER_TOOL_NAME_SEPARATOR)
                .collect::<Vec<_>>();
            if split.len() != 2 {
                log::warn!("工具名称格式错误，无法调用，错误的名称：{}", name);
                tool_messages.push(ChatMessage::Tool {
                    content: "调用失败".to_string(),
                    tool_call_id: tool_call_id.clone(),
                });
                continue;
            }
            // MCP Server名
            let server_name = split[0];
            // 工具名
            let tool_name = split[1];
            // 调用工具
            let tool_result = match mcp_manager::call_tool(
                server_name,
                tool_name,
                Some(parameters.clone()),
            )
            .await
            {
                Ok(result) => {
                    //log::info!("工具调用结果：{:?}", result);
                    if result.content.is_empty() {
                        log::warn!("工具调用成功，但结果为空，请检查工具是否正确返回结果");
                        "".to_string()
                    } else {
                        let content = result.content.first().unwrap();
                        content.as_text().cloned().unwrap().text
                    }
                }
                Err(e) => {
                    log::error!("工具调用失败，工具名称：{}，原因: {:?}", name, e);
                    "工具调用失败".to_string()
                }
            };

            // 工具消息
            tool_messages.push(ChatMessage::Tool {
                content: tool_result,
                tool_call_id: tool_call_id.clone(),
            })
        }
        tool_messages
    }
}

/// 标准消息类型
#[derive(Debug, Clone)]
pub enum StandardChatMessage {
    /// 系统消息，用于角色提示
    System(String),
    /// 用户消息
    User(String),
    /// 助手消息，用于上下文保持
    Assistant(String),
}

impl Into<ChatMessage> for StandardChatMessage {
    fn into(self) -> ChatMessage {
        match self {
            StandardChatMessage::User(content) => ChatMessage::User {
                content: ChatMessageContent::Text(content),
                name: None,
            },
            StandardChatMessage::Assistant(content) => ChatMessage::Assistant {
                content: Some(ChatMessageContent::Text(content)),
                reasoning_content: None,
                refusal: None,
                name: None,
                audio: None,
                tool_calls: None,
            },
            StandardChatMessage::System(content) => ChatMessage::System {
                content: ChatMessageContent::Text(content),
                name: None,
            },
        }
    }
}

/// 从DeepseekMessage构建ChatMessage
struct MessageBuilder {
    messages: Vec<StandardChatMessage>,
}

impl MessageBuilder {
    fn new(messages: Vec<StandardChatMessage>) -> Self {
        Self { messages }
    }
    pub fn build(self) -> Vec<ChatMessage> {
        // 转化消息类型
        let messages = self
            .messages
            .into_iter()
            .map(|msg| msg.into())
            .collect::<Vec<ChatMessage>>();
        messages
    }
}
