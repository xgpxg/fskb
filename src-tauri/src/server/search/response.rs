use crate::db::model::knowledge_base::KnowledgeBase;
use crate::db::model::knowledge_base_import_record::KnowledgeBaseImportRecord;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(default)]
pub(crate) struct SearchRes {
    /// 模型id
    pub(crate) kb: KbSearchRes,
    /// 模型名称
    pub(crate) local: LocalSearchRes,
    /// 模型描述
    pub(crate) web: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(default)]
pub(crate) struct KbSearchRes {
    pub(crate) items: Vec<KbSearchItemRes>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(default)]
pub(crate) struct KbSearchItemRes {
    pub(crate) content: String,
    pub(crate) score: Option<f32>,
    pub(crate) ref_kb: KnowledgeBase,
    pub(crate) ref_import_record: KnowledgeBaseImportRecord,
}
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(default)]
pub(crate) struct LocalSearchRes {
    pub(crate) items: Vec<LocalSearchItemRes>,
    pub(crate) has_next: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(default)]
pub(crate) struct LocalSearchItemRes {
    pub(crate) filename: String,
    pub(crate) filepath: String,
}

#[derive(Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub(crate) enum SearchResultEvent {
    Kb(KbSearchRes),
    Local(LocalSearchRes),
    Done,
}
