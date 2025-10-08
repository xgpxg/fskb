use crate::common::req::PageReq;
use crate::db::model::knowledge_base_import_record::{
    KnowledgeBaseImportFileContentExtractModelConfig, KnowledgeBaseImportFileContentExtractType,
};
use crate::impl_pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KbAddReq {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeBaseImportRecordListReq {
    pub kb_id: i64,
    pub page: PageReq,
}
impl_pagination!(KnowledgeBaseImportRecordListReq);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeBaseUpdateReq {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub mcp_server_ids: Option<Vec<i64>>,
    pub model_id: Option<i64>,
    /// 取值：[`crate::db::model::knowledge_base_import_record::KnowledgeBaseImportFileContentExtractType`]
    pub file_content_extract_type: Option<KnowledgeBaseImportFileContentExtractType>,
    pub file_content_extract_model_config: Option<KnowledgeBaseImportFileContentExtractModelConfig>,
}
