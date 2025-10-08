use crate::TextGenModelConfig;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
    ChatMessageContentPart, ChatMessageImageContentPart, ChatMessageTextContentPart, ImageUrlType,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ShareCardConfig {
    pub content: String,
    pub title: Option<String>,
    pub style: Option<Vec<String>>,
    pub layout: Option<String>,
    pub prompt: Option<String>,
    pub model: TextGenModelConfig,
}
pub async fn generate(config: ShareCardConfig) -> anyhow::Result<Option<String>> {
    let mut client = Client::new(config.model.api_key.clone().unwrap_or_default());
    client.set_base_url(&config.model.base_url);
    client.http_client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .build()?;
    let parameters = ChatCompletionParametersBuilder::default()
        // 模型名称
        .model(&config.model.model_name)
        // 消息
        .messages(build_messages(&config).await?)
        // 返回格式
        .response_format(ChatCompletionResponseFormat::Text)
        // 不使用流式调用
        .stream(false)
        .build()?;

    let response = client.chat().create(parameters).await?;
    log::debug!("[text-gen]model response: {:?}", response);

    let choice = response.choices.get(0).unwrap().clone();
    let text = match choice.message {
        ChatMessage::Assistant { content, .. } => match content {
            None => None,
            Some(content) => match content {
                ChatMessageContent::Text(text) => {
                    let text = text.replace("```html", "").replace("```", "");
                    Some(text)
                }
                _ => return Err(anyhow::anyhow!("返回格式错误")),
            },
        },
        _ => return Err(anyhow::anyhow!("返回格式错误")),
    };
    Ok(text)
}

async fn build_messages(config: &ShareCardConfig) -> anyhow::Result<Vec<ChatMessage>> {
    let messages = vec![ChatMessage::User {
        content: ChatMessageContent::Text(format!(
            "【生成规则】\n{}\n\n【卡片内容】\n{}",
            get_rule(),
            get_content(config),
        )),
        name: None,
    }];
    Ok(messages)
}
fn get_content(config: &ShareCardConfig) -> String {
    let mut content = Vec::new();

    let mut rules = vec![];
    // 风格
    if let Some(style) = &config.style {
        rules.push(style.join(","));
    }
    // 版式
    if let Some(layout) = &config.layout {
        rules.push(layout.to_string());
    }
    if let Some(prompt) = &config.prompt {
        rules.push(prompt.to_string());
    }
    content.push(format!("卡片风格要求：{}", rules.join(",")));

    // 标题
    if let Some(title) = &config.title {
        content.push(format!("卡片标题：{}", title));
    }
    // 内容
    content.push(format!("卡片内容：{}", config.content));

    content.join("\n")
}
fn get_rule() -> String {
    let rule = "
        1. 请理卡片内容，并按照要求然后生成对应的分享卡片。
        2. 以HTML代码返回，仅使用html和css，禁止使用js。
        3. 禁止设置高度和最大高度，确保内容完整显示。
        4. 仅返回完整的HTML格式内容，不要添加任何解释或额外文本
    ";
    rule.to_string()
}

#[tokio::test]
async fn test_gen() -> anyhow::Result<()> {
    const BASE_URL: &str = "https://open.bigmodel.cn/api/paas/v4";
    const API_KEY: &str = "xxx";
    const MODEL_NAME: &str = "glm-4-flash-250414";

    let mut result = generate(ShareCardConfig {
        content: "乍暖还寒的夜里，父亲躺在床上，脑海中不断浮现出一个模糊身影，像《诗经·周南·关雎》所写的：“求之不得，寤寐思服，悠哉悠哉，辗转反侧。”他在昏黄煤油灯映照下，掏出钢笔，写下假条，父亲请假时，领导拍了拍他的肩膀说：“速去速回，早点把喜事定下来。”".to_string(),
        title:  None,
        style: Some(vec!["阳光".to_string(),"春天".to_string()]),
        layout: Some("竖版".to_string()),
        prompt: None,
        model: TextGenModelConfig {
            api_key: Some(API_KEY.to_string()),
            base_url: BASE_URL.to_string(),
            model_name: MODEL_NAME.to_string(),
        },
    })
    .await?;
    println!("{:?}", result);
    let result = result.unwrap().replace("```html", "").replace("```", "");
    fs::write("share-card.html", result)?;
    Ok(())
}
