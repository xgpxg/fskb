use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileUpdateReq {
    /// 是否启用画像记忆
    pub enable_profile_memory: Option<i8>,
    /// 记忆提取使用的模型ID
    pub profile_memory_model_id: Option<i64>,
}
