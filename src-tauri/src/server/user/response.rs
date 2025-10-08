use crate::db::model::user_profile::UserProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileRes {
    /// 是否启用画像记忆
    pub enable_profile_memory: Option<i8>,
    /// 记忆提取使用的模型ID
    pub profile_memory_model_id: Option<i64>,
}
impl Default for UserProfileRes {
    fn default() -> Self {
        let default = UserProfile::default();
        UserProfileRes {
            enable_profile_memory: default.enable_profile_memory,
            profile_memory_model_id: default.profile_memory_model_id,
        }
    }
}
