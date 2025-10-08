use crate::common::res::Res;
use crate::server::components::share_card;

// 生成分享卡片
#[tauri::command]
pub(crate) async fn gen_share_card(
    style: Option<Vec<String>>,
    layout: Option<String>,
    content: String,
    title: Option<String>,
    prompt: Option<String>,
    model_name: String,
) -> Res<String> {
    match share_card::gen_share_card(style, layout, content, title,prompt, model_name).await {
        Ok(result) => Res::success(result),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}
