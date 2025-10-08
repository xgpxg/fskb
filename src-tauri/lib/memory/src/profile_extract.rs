use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
    ChatMessageContentPart, ChatMessageImageContentPart, ChatMessageTextContentPart, ImageUrlType,
};
use std::fs;
use std::io::Read;
use std::time::Duration;

pub async fn extra(
    user_message: &str,
    assistant_message: &str,
    user_profile: &str,
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
        .messages(build_messages(user_message, assistant_message, user_profile).await?)
        // 返回格式
        .response_format(ChatCompletionResponseFormat::Text)
        // 不使用流式调用
        .stream(false)
        .build()?;

    let response = client.chat().create(parameters).await?;
    log::debug!("[memory]model response: {:?}", response);

    let choice = response.choices.get(0).unwrap().clone();
    let text = match choice.message {
        ChatMessage::Assistant { content, .. } => match content {
            None => None,
            Some(content) => match content {
                ChatMessageContent::Text(text) => {
                    let text = text.replace("```yaml", "").replace("```", "");
                    Some(text)
                }
                _ => return Err(anyhow::anyhow!("返回格式错误")),
            },
        },
        _ => return Err(anyhow::anyhow!("返回格式错误")),
    };
    Ok(text)
}

async fn build_messages(
    user_message: &str,
    assistant_message: &str,
    user_profile: &str,
) -> anyhow::Result<Vec<ChatMessage>> {
    let messages = vec![ChatMessage::User {
        content: ChatMessageContent::Text(format!(
            "【提取规则】\n{}\n\n【用户消息】\n{}\n\n【助手消息】\n{}\n\n【已有的用户画像】\n{}",
            get_extract_rule(),
            user_message,
            assistant_message,
            user_profile
        )),
        name: None,
    }];
    Ok(messages)
}

fn get_extract_rule() -> String {
    let rule = "
        1. 分析【用户消息】和【助手消息】中的关键信息
        2. 将新信息补充到现有用户画像的YAML格式中
        3. 保留所有现有内容，仅更新或添加新信息
        4. 仅返回完整的YAML格式内容，不要添加任何解释或额外文本
        5. 确保YAML格式正确且结构清晰
    ";
    rule.to_string()
}

#[tokio::test]
async fn test_extract() -> anyhow::Result<()> {
    const BASE_URL: &str = "https://open.bigmodel.cn/api/paas/v4";
    const API_KEY: &str = "xxx";
    const MODEL_NAME: &str = "glm-4-flash-250414";

    let mut result = extra(
        "回答简单点，不要太罗嗦",
        r#"好的，孩子5岁上一年级，说明他已经开始正式学习了。这个阶段最重要的是：

培养学习兴趣：多鼓励，少批评
建立好习惯：按时写作业、整理书包
多阅读：每天陪孩子读点书
练表达：让孩子多说、多讲
保持耐心：孩子会进步，只是时间问题
如果你需要，我可以提供简单的识字、数学练习题或阅读建议。想要吗？😊"#,
        "",
        BASE_URL,
        MODEL_NAME,
        API_KEY,
    )
    .await?;
    println!("{:?}", result);
    let result = result.unwrap().replace("```yaml", "").replace("```", "");
    fs::write("user_profile.yml", result)?;
    Ok(())
}
