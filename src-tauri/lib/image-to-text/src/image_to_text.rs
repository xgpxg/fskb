use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
    ChatMessageContentPart, ChatMessageImageContentPart, ChatMessageTextContentPart, ImageUrlType,
};
use reqwest::Url;
use std::fs;
use std::io::Read;
use std::time::Duration;

/// 提取图片中的文字
pub async fn extra(
    image_url: &str,
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
        .messages(build_messages(image_url).await?)
        // 返回格式
        .response_format(ChatCompletionResponseFormat::Text)
        // 不使用流式调用
        .stream(false)
        .build()?;

    let response = client.chat().create(parameters).await?;
    log::debug!("[image-to-text]model response: {:?}", response);

    let choice = response.choices.get(0).unwrap().clone();
    let text = match choice.message {
        ChatMessage::Assistant { content, .. } => match content {
            None => None,
            Some(content) => match content {
                ChatMessageContent::Text(text) => Some(text),
                _ => return Err(anyhow::anyhow!("返回格式错误")),
            },
        },
        _ => return Err(anyhow::anyhow!("返回格式错误")),
    };

    Ok(text)
}

async fn build_messages(image_url: &str) -> anyhow::Result<Vec<ChatMessage>> {
    let messages = vec![ChatMessage::User {
        content: ChatMessageContent::ContentPart(vec![
            ChatMessageContentPart::Image(ChatMessageImageContentPart {
                image_url: ImageUrlType {
                    url: image_url_to_base64(image_url).await?,
                    detail: None,
                },
                r#type: "image_url".to_string(),
            }),
            ChatMessageContentPart::Text(ChatMessageTextContentPart {
                text: "请分析图片并提取所有可见文本内容，按从左到右、从上到下的布局，返回纯文本，表格使用markdown格式；涉及到公式时请使用Katex语法，行内公式用单个$包裹，块级公式用$$包裹，公式首尾不要有空格;不要添加额外文字。"
                    .to_string(),
                r#type: "text".to_string(),
            }),
        ]),
        name: None,
    }];
    Ok(messages)
}

async fn image_url_to_base64(image_url: &str) -> anyhow::Result<String> {
    let data = if is_valid_url(image_url) {
        // 处理网络URL
        let client = reqwest::Client::new();
        let response = client.get(image_url).send().await?.bytes().await?;
        response.to_vec()
    } else {
        // 处理本地文件路径
        let mut file = fs::File::open(image_url)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        buffer
    };

    // 转换为 Base64
    let base64 = BASE64_STANDARD.encode(&data);
    Ok(format!("data:image/jpeg;base64,{}", base64)) // 添加正确的data URL前缀
}

fn is_valid_url(url: &str) -> bool {
    // 检查是否为有效的网络URL
    if let Ok(parsed_url) = Url::parse(url) {
        // 只有http/https协议才认为是网络URL
        return parsed_url.scheme() == "http" || parsed_url.scheme() == "https";
    }
    false
}

#[tokio::test]
async fn test_image_to_text() -> anyhow::Result<()> {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;

    const BASE_URL: &str = "https://open.bigmodel.cn/api/paas/v4";
    const API_KEY: &str = "xxx";
    const MODEL_NAME: &str = "glm-4v-flash";

    let img = fs::read("/mnt/d/download/dfee8d28bf30432499c7725e0e4c5b3a.png")?;
    let image = STANDARD.encode(img);
    let text = extra(&image, BASE_URL, MODEL_NAME, API_KEY).await?;
    println!("{:?}", text);
    Ok(())
}
