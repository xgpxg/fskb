use crate::db::model::model::Model;
use crate::db::Pool;
use crate::server::user::{service, User};
use anyhow::bail;
use common::data_dir;
use rbs::value;
use std::fs;

/// 提取用户画像
pub(crate) async fn extract(user_message: &str, assistant_message: &str) -> anyhow::Result<()> {
    let path = data_dir!("user", "profile", "profile.memory");

    // 获取用户配置
    let profile = User::profile().await;
    // 是否启用了画像记忆
    if profile.enable_profile_memory != Some(1) {
        return Ok(());
    }
    // 模型ID
    let model_id = profile.profile_memory_model_id;
    if model_id.is_none() {
        bail!("Profile memory extract model not set, can not extract profile")
    }
    let model_id = model_id.unwrap();

    let model = Model::select_by_map(
        Pool::get()?,
        value! {
            "id":model_id,
        },
    )
    .await?;
    if model.is_empty() {
        bail!("Profile memory extract model not found")
    }
    let model = model.first().unwrap();

    // 加载已有的用户画像
    let mut user_profile = memory::UserProfile::load(&path)?;
    // 提取用户画像
    user_profile
        .extract(
            user_message,
            assistant_message,
            &model.base_url.clone().unwrap_or_default(),
            &model.name.clone().unwrap_or_default(),
            &model.api_key.clone().unwrap_or_default(),
        )
        .await?;

    let path = data_dir!("user", "profile", "main.profile");
    fs::create_dir_all(path.parent().unwrap())?;
    user_profile.save(&path)?;
    Ok(())
}

/// 加载用户画像
pub(crate) fn load() -> memory::UserProfile {
    let path = data_dir!("user", "profile", "main.profile");
    let user_profile = memory::UserProfile::load(&path).unwrap_or_else(|e| {
        log::error!("Load user profile failed: {}", e);
        memory::UserProfile::default()
    });
    user_profile
}
