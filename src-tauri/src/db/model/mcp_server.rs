use derive_builder::Builder;
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, htmlsql};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
pub struct McpServer {
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
    /// 来源：1内置 2自定义
    pub source: Option<i8>,
    /// 是否可配置：0不可配置 1可配置，如果可配置，则需要提供一个配置页面，目前仅平台内置的server可配置
    pub configurable: Option<i8>,
    /// 状态：0未启用 1已启用 2异常 3安装中 4启动中 5停止中 6升级中
    pub status: Option<i8>,
    /// 状态信息
    pub status_msg: Option<String>,
    /// 已安装版本
    pub installed_version: Option<String>,
    /// 最新版本
    pub latest_version: Option<String>,
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

pub enum McpServerSource {
    /// 平台内置
    Platform = 1,
    /// 自定义
    Custom = 2,
}

pub enum McpServerStatus {
    /// 未运行
    NotRun = 0,
    /// 正常
    Ok = 1,
    /// 异常
    #[allow(unused)]
    Error = 2,
    /// 安装中
    Installing = 3,
    /// 启动中
    Starting = 4,
    /// 停止中
    Stopping = 5,
    /// 升级中
    Upgrading = 6,
}

crud!(McpServer {});
