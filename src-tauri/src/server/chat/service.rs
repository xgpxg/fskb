use crate::common::id;
use crate::db::model::chat_message;
use crate::db::model::chat_message::{
    ChatMessage, ChatMessageBuilder, ChatMessageRole, ChatMessageStatus,
};
use crate::db::model::knowledge_base::KnowledgeBase;
use crate::db::model::mcp_server::McpServer;
use crate::db::model::model::{Model, ModelTaskType};
use crate::db::{tools, Pool};
use crate::server::chat::chat_helper::image_path_to_base64;
use crate::server::chat::request::UserMessageContent;
use crate::server::chat::{chat_model, command, ChatEvent};
use crate::server::mcp::default::kb_mcp::KbMcp;
use crate::server::mcp::default::{kb_mcp, DefaultMcpServer};
use crate::server::user;
use crate::server::user::User;
use crate::utils::file_util;
use crate::{constant, server};
use anyhow::{bail, Context};
use common::data_dir;
use mcp::mcp_manager;
use memory::UserProfile;
use openai_dive::v1::resources::chat;
use openai_dive::v1::resources::chat::{
    ChatCompletionFunction, ChatCompletionTool, ChatCompletionToolType, ChatMessageContent,
    ChatMessageContentPart, ChatMessageImageContentPart, ChatMessageTextContentPart, ImageUrlType,
};
use rbs::value;
use serde_json::Value;
use tauri::ipc::Channel;
use tauri::Emitter;

pub(crate) async fn chat(
    kb_id: i64,
    content: UserMessageContent,
    channel: Channel<ChatEvent>,
) -> anyhow::Result<()> {
    // 生成用户消息和助手消息
    let (user_message, assistant_message) = before_reply(kb_id, &content).await?;
    ChatEvent::add_channel(&assistant_message.gen_channel_key(), channel);
    // 推送开始事件
    ChatEvent::Start(assistant_message.clone()).send().await?;
    // 初始化缓存的消息（必须）
    ChatEvent::append_cache_message(&assistant_message);

    // 处理指令消息
    if command::is_command_message(&content) {
        log::info!("Handle command message");
        let mut temp = assistant_message.clone();
        tokio::spawn(async move {
            match command::run_command(&content).await {
                Ok(result) => {
                    temp.content = Some(result);
                    temp.status = Some(ChatMessageStatus::Finished.to_string());
                    // 追加全量消息
                    ChatEvent::append_cache_message(&temp);
                    // 发送事件到前端
                    ChatEvent::Message(temp.clone()).send().await.unwrap();
                    done(user_message, temp).await.unwrap();
                    log::info!("Command message finished");
                }
                Err(e) => {
                    done_with_error(user_message, temp, e.to_string()).await;
                }
            };
        });
        return Ok(());
    }

    // 知识库
    let kb = get_kb(kb_id).await?;
    // 检查模型是否配置
    if kb.model_id.is_none() {
        done_with_error(
            user_message,
            assistant_message,
            "Knowledge base is not configured with a language model, unable to chat".to_string(),
        )
        .await;
        return Ok(());
    };
    // 模型
    let model = match get_model(kb.model_id.unwrap()).await {
        Ok(model) => model,
        Err(e) => {
            done_with_error(user_message, assistant_message, e.to_string()).await;
            return Ok(());
        }
    };
    // 构建消息
    let messages = build_chat_messages(&kb, content, &user_message, &assistant_message).await;
    // 可用工具
    let tools = match get_tools(&kb).await {
        Ok(tools) => tools,
        Err(e) => {
            done_with_error(
                user_message,
                assistant_message,
                format!("Call MCP error: {}", e),
            )
            .await;
            return Ok(());
        }
    };
    // 发起对话
    tokio::spawn(async move {
        let res = chat_model::chat(
            &model,
            messages,
            tools,
            |content| {
                Box::pin({
                    let mut temp = assistant_message.clone();
                    //let channel = channel.clone();
                    async move {
                        //log::info!("receive message: {:?}", temp);
                        // 复制一份，更新消息内容
                        temp.content = Some(content);
                        // 追加全量消息
                        ChatEvent::append_cache_message(&temp);
                        // 发送事件到前端
                        ChatEvent::Message(temp).send().await.unwrap();
                    }
                })
            },
            |full_content| {
                Box::pin({
                    let mut fm = assistant_message.clone();
                    let um = user_message.clone();
                    //let channel = channel.clone();
                    async move {
                        // 完整的模型回复的消息
                        fm.content = Some(full_content);
                        // 状态更新为已完成
                        fm.status = Some(ChatMessageStatus::Finished.to_string());
                        if let Err(e) = done(um, fm.clone()).await {
                            log::error!(
                                "Handel DONE failed, kb id: {:?}, reason: {}",
                                &fm.knowledge_base_id,
                                e
                            );
                        };
                        log::info!("Chat completed, kb id: {:?}", &fm.knowledge_base_id);
                    }
                })
            },
        )
        .await;
        // 对话失败处理
        if let Err(e) = res {
            done_with_error(
                user_message.clone(),
                assistant_message.clone(),
                e.to_string(),
            )
            .await;
        }
        Ok::<(), anyhow::Error>(())
    });

    Ok(())
}

pub(crate) async fn resume(
    kb_id: i64,
    message_id: i64,
    channel: Channel<ChatEvent>,
) -> anyhow::Result<()> {
    let ck = &ChatEvent::build_channel_key(kb_id, message_id);
    // 已缓存的消息
    let fm = ChatEvent::get_cache_message(ck);

    // 获取不到缓存的消息，并且库里的消息状为pending，则更新消息状态为失败
    // 这种情况可能会出现在程序强制关闭时，消息正在回复中，还没有入库
    if fm.is_none() {
        let message = ChatMessage::select_by_map(
            Pool::get()?,
            value! {"knowledge_base_id":kb_id,"message_id": message_id},
        )
        .await?;
        if message.is_empty() {
            return Ok(());
        }
        if message.first().unwrap().status != Some(ChatMessageStatus::Pending.to_string()) {
            return Ok(());
        }
        let message = ChatMessageBuilder::default()
            .id(message.first().unwrap().id)
            .status(Some(ChatMessageStatus::Error.to_string()))
            .build()?;
        ChatMessage::update_by_map(Pool::get()?, &message, value! {"id": message.id}).await?;
        return Ok(());
    }

    if let Some(fm) = ChatEvent::get_cache_message(ck) {
        ChatEvent::new_resume_channel(ck, channel, ChatEvent::Message(fm.clone()))?;
    }

    Ok(())
}

async fn before_reply(
    kb_id: i64,
    content: &UserMessageContent,
) -> anyhow::Result<(ChatMessage, ChatMessage)> {
    // 最后一条消息ID
    let last_message_id = chat_message::last_message_id(Pool::get()?, kb_id)
        .await?
        .unwrap_or(0);
    let user_message = ChatMessageBuilder::default()
        .id(Some(id::next()))
        .knowledge_base_id(Some(kb_id))
        .message_id(Some(last_message_id + 1))
        .parent_message_id(Some(last_message_id))
        .content(Some(serde_json::to_string(&content)?))
        .role(Some(ChatMessageRole::User.to_string()))
        .status(Some(ChatMessageStatus::Finished.to_string()))
        .create_time(Some(tools::now()))
        .build()?;

    let assistant_message = ChatMessageBuilder::default()
        .id(Some(id::next()))
        .knowledge_base_id(Some(kb_id))
        .message_id(Some(last_message_id + 2))
        .parent_message_id(user_message.message_id)
        .content(Some("".to_string()))
        .role(Some(ChatMessageRole::Assistant.to_string()))
        .status(Some(ChatMessageStatus::Pending.to_string()))
        .create_time(Some(tools::now()))
        .build()?;

    if let Err(e) = ChatMessage::insert_batch(
        Pool::get()?,
        &[user_message.clone(), assistant_message.clone()],
        2,
    )
    .await
    {
        log::error!("消息保存失败：{}", e);
    }

    Ok((user_message.clone(), assistant_message.clone()))
}

async fn build_chat_messages(
    kb: &KnowledgeBase,
    content: UserMessageContent,
    user_message: &ChatMessage,
    assistant_message: &ChatMessage,
) -> Vec<chat::ChatMessage> {
    let mut messages = vec![];

    let mut content = content;
    let mut rules = vec![];

    if let Some(images) = &content.images {
        if !images.is_empty() {
            rules.push("请调用图片工具分析图片".to_string());
        }
    }
    if content.audios.is_some() {
        unimplemented!("audio not support yet");
    }
    if content.videos.is_some() {
        unimplemented!("video not support yet");
    }
    if let Some(files) = &content.files {
        if !files.is_empty() {
            rules.push("请调用文件工具分析文件".to_string());
        }
    }

    content.rules = Some(rules);

    // 历史消息
    messages.extend(get_history_messages(kb, user_message, assistant_message).await);
    // 当前用户消息
    messages.push(chat::ChatMessage::User {
        content: ChatMessageContent::Text(serde_json::to_string(&content).unwrap()),
        name: None,
    });

    messages.insert(
        0,
        chat::ChatMessage::System {
            content: ChatMessageContent::Text(parse_role_prompt(kb).await),
            name: None,
        },
    );

    messages
}

async fn get_history_messages(
    kb: &KnowledgeBase,
    user_message: &ChatMessage,
    assistant_message: &ChatMessage,
) -> Vec<chat::ChatMessage> {
    let mut messages = vec![];
    ChatMessage::select_by_map(
        Pool::get().unwrap(),
        value! {
            "knowledge_base_id" : kb.id,
        },
    )
    .await
    .unwrap();
    let mut history_messages =
        chat_message::list_history_messages_limit(Pool::get().unwrap(), kb.id.unwrap(), Some(50))
            .await
            .unwrap();
    history_messages.sort_by(|a, b| a.message_id.cmp(&b.message_id));
    for message in history_messages {
        // 剔除掉本次对话刚生成的用户消息和助手消息，这两条不作为历史消息
        if message.id == user_message.id || message.id == assistant_message.id {
            continue;
        }
        let role = message.role.unwrap();
        match role.as_str() {
            "user" => {
                messages.push(chat::ChatMessage::User {
                    content: ChatMessageContent::Text(message.content.unwrap()),
                    name: None,
                });
            }
            "assistant" => {
                messages.push(chat::ChatMessage::Assistant {
                    content: Some(ChatMessageContent::Text(
                        message.content.unwrap_or_default(),
                    )),
                    reasoning_content: None,
                    refusal: None,
                    name: None,
                    audio: None,
                    tool_calls: None,
                });
            }
            _ => {}
        }
    }

    messages
}

pub(crate) async fn parse_role_prompt(kb: &KnowledgeBase) -> String {
    let mut role_prompt = String::new();

    // 核心行为指导
    role_prompt.push_str("## 角色与行为准则\n");
    role_prompt.push_str("你是专业的智能助手，需要严格遵循以下规则：\n\n");

    // 用户消息格式说明
    role_prompt.push_str("### 消息格式说明\n");
    role_prompt.push_str("用户消息将以JSON格式发送，包含以下可能的字段：\n");
    role_prompt.push_str("- `text`: 用户的主要问题或指令\n");
    role_prompt.push_str("- `images`: 图片文件路径列表\n");
    role_prompt.push_str("- `audios`: 音频文件路径列表\n");
    role_prompt.push_str("- `videos`: 视频文件路径列表\n");
    role_prompt.push_str("- `files`: 其他文件路径列表\n");
    role_prompt.push_str("- `rules`: 用户要求的规则\n\n");

    // 工具使用规则
    role_prompt.push_str("### 工具使用规则\n");
    role_prompt.push_str("1. 对于用户提供的每张图片，必须调用图像分析工具进行处理\n");
    role_prompt.push_str("2. 根据内容类型选择合适的工具处理音频、视频或文件\n");
    role_prompt.push_str("3. 只在必要时调用工具，避免无意义的工具调用\n\n");

    // 输出格式规范
    role_prompt.push_str("### 输出格式规范\n");
    role_prompt.push_str("1. 回答内容应结构清晰、逻辑严谨\n");
    role_prompt.push_str("2. 包含图片的回复必须使用markdown格式：`![描述](图片路径)`\n");
    role_prompt.push_str("3. 数学公式使用LaTeX语法：行内公式 `$公式$`，独立行公式 `$$公式$$，公式与$符号之间不能有空格(重要)`\n");
    role_prompt.push_str("4. 代码片段使用代码块格式\n\n");

    // 上下文管理
    role_prompt.push_str("### 上下文处理\n");
    role_prompt.push_str("1. 基于对话历史理解用户意图\n");
    role_prompt.push_str("2. 避免重复已经明确的信息\n");
    role_prompt.push_str("3. 当用户改变话题时，适时忽略之前的上下文\n\n");

    // 系统信息
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    role_prompt = format!("{}\n## 系统信息\n当前时间：{}\n", role_prompt, now,);

    // 知识库信息
    role_prompt = format!(
        "{}\n## 知识库信息\n{}",
        role_prompt,
        kb.nld.clone().unwrap_or_default()
    );

    if User::profile().await.enable_profile_memory == Some(1) {
        // 用户画像信息
        role_prompt = format!(
            "{}\n## 用户画像信息\n{}",
            role_prompt,
            user::memory::load().get()
        );
    }

    role_prompt
}

async fn get_kb(kb_id: i64) -> anyhow::Result<KnowledgeBase> {
    let kb = KnowledgeBase::select_by_map(Pool::get()?, value! {"id": kb_id}).await?;
    if kb.is_empty() {
        bail!("知识库不存在");
    }
    Ok(kb.first().unwrap().clone())
}

// 获取MCP工具
async fn get_tools(kb: &KnowledgeBase) -> anyhow::Result<Vec<ChatCompletionTool>> {
    let mut tools = vec![];
    // 内置的默认工具
    tools.extend(DefaultMcpServer::all_tools());

    let mcp_servers_ids = kb.mcp_server_ids.clone();
    if mcp_servers_ids.is_none() {
        return Ok(tools);
    }
    let mcp_server_ids = mcp_servers_ids.unwrap();
    let mcp_servers =
        McpServer::select_by_map(Pool::get()?, value! {"id": &mcp_server_ids}).await?;
    for mcp_server in mcp_servers {
        let sever_name = mcp_server.name.clone().unwrap();
        let ts = mcp_manager::list_all_tools(&sever_name).await?;
        for t in ts {
            tools.push(ChatCompletionTool {
                r#type: ChatCompletionToolType::Function,
                function: ChatCompletionFunction {
                    name: format!(
                        "{}{}{}",
                        sever_name,
                        constant::MCP_SERVER_TOOL_NAME_SEPARATOR,
                        t.name
                    ),
                    description: Some(t.description.unwrap_or_default().to_string()),
                    parameters: serde_json::from_str(&serde_json::to_string(&t.input_schema)?)?,
                },
            });
        }
    }
    Ok(tools)
}

/// 会话结束
pub async fn done(user_message: ChatMessage, assistant_message: ChatMessage) -> anyhow::Result<()> {
    let mut done = assistant_message.clone();
    done.content = Some("[DONE]".to_string());
    ChatEvent::Done(done).send().await?;

    let mut assistant_message = assistant_message;
    // 更新时间
    assistant_message.update_time = Some(tools::now());
    // 保存助手消息到数据库
    if let Err(e) = ChatMessage::update_by_map(
        Pool::get()?,
        &assistant_message,
        value! {"id": assistant_message.id},
    )
    .await
    {
        log::error!("模型回复完成，但保存会话消息失败：{}", e);
    }

    ChatEvent::close_channel(&assistant_message.gen_channel_key());

    // 提取用户画像
    tokio::spawn(async move {
        if let Err(e) = user::memory::extract(
            &user_message.content.unwrap_or_default(),
            &assistant_message.content.unwrap_or_default(),
        )
        .await
        {
            log::error!("User profile update fail: {}", e)
        }
    });

    Ok(())
}

async fn done_with_error(user_message: ChatMessage, assistant_message: ChatMessage, err: String) {
    log::error!(
        "Chat failed, knowledge_base_id: {:?}, reason: {}",
        assistant_message.knowledge_base_id,
        err
    );

    // 推送错误原因
    let mut eam = assistant_message.clone();
    // 消息内容为错误原因
    eam.content = Some(err);
    // 状态设置为error
    eam.status = Some(ChatMessageStatus::Error.to_string());
    // 推送
    ChatEvent::Message(eam.clone()).send().await.unwrap();

    if let Err(e) = done(user_message, eam.clone()).await {
        log::error!("Handel DONE failed when model invoke failed, reason: {}", e);
    }
}
// 获取模型信息
async fn get_model(model_id: i64) -> anyhow::Result<Model> {
    let model = Model::select_by_map(Pool::get()?, value! {"id": model_id}).await?;
    if model.is_empty() {
        bail!("模型不可用，可能已被删除或停用，请重新选择模型");
    }
    Ok(model.first().unwrap().clone())
}

// 所有历史消息
#[deprecated]
#[allow(unused)]
pub(crate) async fn list_all_history_messages(kb_id: i64) -> anyhow::Result<Vec<ChatMessage>> {
    let mut list =
        ChatMessage::select_by_map(Pool::get()?, value! {"knowledge_base_id": kb_id}).await?;
    list.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(list)
}

pub(crate) async fn list_history_messages(
    kb_id: i64,
    last_message_id: Option<i64>,
) -> anyhow::Result<Vec<ChatMessage>> {
    let mut list =
        chat_message::list_history_messages(Pool::get()?, kb_id, last_message_id).await?;
    list.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(list)
}

pub(crate) async fn clear_message(kb_id: i64) -> anyhow::Result<()> {
    ChatMessage::delete_by_map(Pool::get()?, value! {"knowledge_base_id": kb_id}).await?;

    let ref_dir = data_dir!("chat", "kb", kb_id.to_string());
    if ref_dir.exists() {
        std::fs::remove_dir_all(ref_dir)?;
    }
    Ok(())
}

pub(crate) async fn delete_message(id: i64) -> anyhow::Result<()> {
    ChatMessage::delete_by_map(Pool::get()?, value! {"id": id}).await?;
    Ok(())
}
