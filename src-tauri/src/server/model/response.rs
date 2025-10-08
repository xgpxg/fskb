use crate::db;
use derive_builder::Builder;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(default)]
pub struct ModelListRes {
    pub id: Option<i64>,
    /// 模型名称
    pub name: Option<String>,
    /// 模型描述
    pub description: Option<String>,
    /// 来源：1内置 2自建
    pub source: Option<i8>,
    /// 图标
    pub icon: Option<String>,
    /// 状态：0未启用 1已启用
    pub status: Option<i8>,
    /// 状态信息
    pub status_msg: Option<String>,
    /// 接口地址
    pub base_url: Option<String>,
    /// api key
    pub api_key: Option<String>,
    /// 最大token
    pub max_token: Option<i32>,
    /// 适用的任务类型：1文本生成 2视觉问答
    pub task_type: Option<i8>,
    /// 创建时间
    pub create_time: Option<DateTime>,
    /// 更新时间
    pub update_time: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(default)]
pub struct ModelSimpleListRes {
    /// 模型id
    pub id: Option<i64>,
    /// 模型名称
    pub name: Option<String>,
    /// 模型描述
    pub description: Option<String>,
    /// 图标
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(default)]
pub struct OfflineModelListRes {
    /// 模型名称
    pub name: String,
    /// 模型标题
    pub summary: String,
    /// 模型描述
    pub description: String,
    /// 状态
    /// [`db::model::model::ModelStatus`]
    pub status: i8,
    /// 任务类型：1文本生成 2视觉问答
    pub task_type: i8,
}
