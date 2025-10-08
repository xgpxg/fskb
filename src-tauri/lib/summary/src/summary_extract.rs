use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
    ChatMessageImageContentPart, ChatMessageTextContentPart, ImageUrlType,
};
use std::time::Duration;

pub async fn extra(
    content: &str,
    base_url: &str,
    model_name: &str,
    api_key: &str,
) -> anyhow::Result<Option<String>> {
    let mut client = Client::new(api_key.to_string());
    client.set_base_url(base_url);
    client.http_client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .build()?;
    let parameters = ChatCompletionParametersBuilder::default()
        // 模型名称
        .model(model_name)
        // 消息
        .messages(build_messages(content).await?)
        // 返回格式
        .response_format(ChatCompletionResponseFormat::Text)
        // 不使用流式调用
        .stream(false)
        .build()?;

    let response = client.chat().create(parameters).await?;
    log::debug!("[summary]model response: {:?}", response);

    let choice = response.choices.get(0).unwrap().clone();
    let text = match choice.message {
        ChatMessage::Assistant { content, .. } => match content {
            None => None,
            Some(content) => match content {
                ChatMessageContent::Text(text) => {
                    let text = text.replace("```json", "").replace("```", "");
                    Some(text)
                }
                _ => return Err(anyhow::anyhow!("返回格式错误")),
            },
        },
        _ => return Err(anyhow::anyhow!("返回格式错误")),
    };
    Ok(text)
}

async fn build_messages(content: &str) -> anyhow::Result<Vec<ChatMessage>> {
    let messages = vec![ChatMessage::User {
        content: ChatMessageContent::Text(format!(
            "【提取规则】\n{}\n\n【用户消息】\n{}",
            get_extract_rule(),
            content,
        )),
        name: None,
    }];
    Ok(messages)
}

fn get_extract_rule() -> String {
    let rule = "
        1. 从用户消息中提取标题和摘要
        2. 以JSON格式返回。格式为：{\"title\",\"\",\"summary\":\"\"}
        3. 仅返回完整的JSON格式内容，不要添加任何解释或额外文本
    ";
    rule.to_string()
}
