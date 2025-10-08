use derive_builder::Builder;
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, htmlsql};
use serde::{Deserialize, Serialize};

/// 用户
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
pub struct UserProfile {
    pub id: Option<i64>,
    /// 是否启用画像记忆
    pub enable_profile_memory: Option<i8>,
    /// 记忆提取使用的模型ID
    pub profile_memory_model_id: Option<i64>,
    /// 创建人ID
    pub create_user_id: Option<i64>,
    /// 修改人ID
    pub update_user_id: Option<i64>,
    /// 创建时间
    pub create_time: Option<DateTime>,
    /// 更新时间
    pub update_time: Option<DateTime>,
    /// 备注
    pub remark: Option<String>,
    /// 用户ID
    pub user_id: Option<i64>,
    ///  租户ID
    pub tenant_id: Option<i64>,
    /// 是否删除
    pub is_delete: Option<i8>,
}

crud!(UserProfile {});
