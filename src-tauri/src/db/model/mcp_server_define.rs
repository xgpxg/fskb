use derive_builder::Builder;
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, htmlsql};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
pub struct McpServerDefine {
    pub id: Option<i64>,
    /// 服务名称，全局唯一
    pub name: Option<String>,
    /// 服务简介
    pub summary: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 头像
    #[serde(deserialize_with = "crate::common::deserialize_to_string")]
    pub config: Option<String>,
    /// 下载地址
    /// 或者SSE地址？？
    pub url: Option<String>,
    /// 是否可配置：0不可配置 1可配置，如果可配置，则需要提供一个配置页面，目前仅平台内置的server可配置
    pub configurable: Option<i8>,
    /// 版本
    pub version: Option<String>,
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
    /// 是否删除
    pub is_delete: Option<i8>,
}

crud!(McpServerDefine {});
