use crate::db::model::knowledge_base::KnowledgeBase;
use crate::db::model::knowledge_base_import_record::KnowledgeBaseImportRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KnowledgeBaseListRes {
    #[serde(flatten)]
    pub(crate) inner: KnowledgeBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KnowledgeBaseDetailRes {
    #[serde(flatten)]
    pub(crate) inner: KnowledgeBase,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KnowledgeBaseImportRecordListRes {
    pub(crate) id: Option<i64>,
    /// 知识库ID
    pub(crate) knowledge_base_id: Option<i64>,
    /// 标题
    pub(crate) title: Option<String>,
    /// 原始文件名
    pub(crate) original_file_name: Option<String>,
    /// 原始文件路径
    pub(crate) original_file_path: Option<String>,
    /// 文件名
    pub(crate) file_name: Option<String>,
    /// 文件路径
    pub(crate) file_path: Option<String>,
    /// 文件名
    pub(crate) file_size: Option<u64>,
    /// 网页地址
    pub(crate) url: Option<String>,
    /// 来源：1文件 2网页 3自定义文本
    pub(crate) source: Option<i8>,
    /// 状态：0待解析 1导入成功 2导入中 3导入失败
    pub(crate) status: Option<i8>,
    /// 状态信息
    pub(crate) status_msg: Option<String>,
    /// 耗时
    pub(crate) use_time: Option<usize>,
}
