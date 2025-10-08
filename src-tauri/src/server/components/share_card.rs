use crate::server::model::get_model_by_name;
use anyhow::bail;
use textgen::share_card_generate::ShareCardConfig;
use textgen::{share_card_generate, TextGenModelConfig};

pub(crate) async fn gen_share_card(
    style: Option<Vec<String>>,
    layout: Option<String>,
    content: String,
    title: Option<String>,
    prompt: Option<String>,
    model_name: String,
) -> anyhow::Result<String> {
    let model = get_model_by_name(&model_name).await?;
    let config = ShareCardConfig {
        content,
        title,
        style,
        layout,
        prompt,
        model: TextGenModelConfig {
            base_url: model.base_url.unwrap(),
            model_name,
            api_key: model.api_key,
        },
    };
    let result = share_card_generate::generate(config).await?;
    if result.is_none() {
        bail!("生成失败");
    }
    Ok(result.unwrap())
}
