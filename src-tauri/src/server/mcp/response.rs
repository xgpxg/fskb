use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct McpServerDefineListRes {
    // 服务名称
    pub(crate) name: String,
    // 服务简介
    pub(crate) summary: String,
    // 描述
    pub(crate) description: Option<String>,
    // 版本
    pub(crate) version: Option<String>,
    // 是否需要升级
    pub(crate) need_upgrade: bool,
    // 是否已安装
    pub(crate) installed: bool,
    // 已安装版本
    pub(crate) installed_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct McpServerListRes {
    pub(crate) id: i64,
    // 服务名称
    pub(crate) name: String,
    // 服务简介
    pub(crate) summary: String,
    // 描述
    pub(crate) description: Option<String>,
    // 是否需要升级
    pub(crate) need_upgrade: bool,
    // 已安装版本
    pub(crate) installed_version: Option<String>,
    // 最新版本
    pub(crate) latest_version: Option<String>,
    // 服务配置
    pub(crate) config: Option<String>,
    // 状态：0未启用 1已启用 2异常 3安装中 4启动中 5停止中 6升级中
    pub(crate) status: Option<i8>,
    // 状态信息
    pub(crate) status_msg: Option<String>,
}
