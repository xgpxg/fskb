use crate::common::req::{PageReq, Pagination};
use crate::impl_pagination;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
#[serde(rename_all = "camelCase")]
pub struct ModelListReq {
    pub page: PageReq,
    pub filter: Option<String>,
}

impl_pagination!(ModelListReq);

#[derive(Debug, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
#[serde(rename_all = "camelCase")]
pub struct ModelAddReq {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub base_url: String,
    pub api_key: Option<String>,
    pub max_token: Option<i32>,
    /// 适用的任务类型：1文本生成 2视觉问答
    pub task_type: i8,
}

#[derive(Debug, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
#[serde(rename_all = "camelCase")]
pub struct ModelUpdateReq {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub max_token: Option<i32>,
    pub status: Option<i8>,
    /// 适用的任务类型：1文本生成 2视觉问答
    pub task_type: Option<i8>,
}
