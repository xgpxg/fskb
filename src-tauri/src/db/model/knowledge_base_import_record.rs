use crate::common::result::AppError;
use crate::server::kb::request::KnowledgeBaseImportRecordListReq;
use anyhow::anyhow;
use derive_builder::Builder;
use rbatis::rbdc::DateTime;
use rbatis::{crud, htmlsql_select_page};
use serde::{Deserialize, Deserializer, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
pub struct KnowledgeBaseImportRecord {
    pub id: Option<i64>,
    /// 知识库ID
    pub knowledge_base_id: Option<i64>,
    /// 来源类型：1本地文件 2网页 3自定义文本
    pub source: Option<i8>,
    /// 标题
    pub title: Option<String>,
    /// 原始文件名
    pub original_file_name: Option<String>,
    /// 原始文件路径
    pub original_file_path: Option<String>,
    /// 文件名
    pub file_name: Option<String>,
    /// 文件大小
    pub file_size: Option<u64>,
    /// 文件路径
    pub file_path: Option<String>,
    /// 文件内容类型：1文档 2数据表
    pub file_content_type: Option<i8>,
    /// 文件内容提取方式型
    pub file_content_extract_type: Option<KnowledgeBaseImportFileContentExtractType>,
    /// 网页地址地址
    pub url: Option<String>,
    /// 导入记录的自然语言描述
    pub nld: Option<String>,
    /// 状态：0待解析 1成功 2导入中 3失败
    pub status: Option<i8>,
    /// 状态信息
    #[serde(deserialize_with = "crate::common::deserialize_to_string")]
    pub status_msg: Option<String>,
    /// 开始时间
    pub start_time: Option<DateTime>,
    /// 结束时间
    pub end_time: Option<DateTime>,
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

pub enum KnowledgeBaseImportSource {
    LocalFile = 1,
    /// 网页
    Url = 2,
    /// 自定义文本
    CustomText = 3,
}

impl TryFrom<i8> for KnowledgeBaseImportSource {
    type Error = AppError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(KnowledgeBaseImportSource::LocalFile),
            2 => Ok(KnowledgeBaseImportSource::Url),
            3 => Ok(KnowledgeBaseImportSource::CustomText),
            _ => Err(AppError::MessageError("未知来源类型".to_string())),
        }
    }
}

pub enum KnowledgeBaseImportStatus {
    /// 待解析
    Waiting = 0,
    /// 成功
    Success = 1,
    /// 导入中(解析中)
    Importing = 2,
    /// 失败
    Failed = 3,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum KnowledgeBaseImportDataType {
    /// 文档
    Document = 1,
    /// 表格
    Table = 2,
}
impl KnowledgeBaseImportDataType {
    pub fn from_file(file: Option<String>) -> Self {
        if file.is_none() {
            return KnowledgeBaseImportDataType::Document;
        }
        let file = PathBuf::from(file.unwrap());
        let ext = file.extension().unwrap().to_str().unwrap();
        match ext {
            "txt" | "md" | "doc" | "docx" | "pdf" => KnowledgeBaseImportDataType::Document,
            "xls" | "xlsx" | "csv" | "tsv" => KnowledgeBaseImportDataType::Table,
            _ => KnowledgeBaseImportDataType::Document,
        }
    }
    pub fn is_csv(file: Option<String>) -> bool {
        if file.is_none() {
            return false;
        }
        let file = PathBuf::from(file.unwrap());
        let ext = file.extension().unwrap().to_str().unwrap();
        match ext {
            "csv" => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum KnowledgeBaseImportFileContentType {
    /// 文档
    Document = 1,
    /// 表格
    Table = 2,
}

impl TryFrom<i8> for KnowledgeBaseImportFileContentType {
    type Error = anyhow::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(KnowledgeBaseImportFileContentType::Document),
            2 => Ok(KnowledgeBaseImportFileContentType::Table),
            _ => Err(anyhow!("未知内容类型")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    rename_all = "snake_case",
    rename_all_fields = "snake_case",
    tag = "type"
)]
pub enum KnowledgeBaseImportFileContentExtractType {
    /// 仅文本
    Text,
    /// OCR
    Ocr,
    /// 视觉模型
    VisionModel { model_id: i64 },
}

impl TryFrom<i8> for KnowledgeBaseImportFileContentExtractType {
    type Error = anyhow::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(KnowledgeBaseImportFileContentExtractType::Text),
            2 => Ok(KnowledgeBaseImportFileContentExtractType::Ocr),
            // 3 => Ok(KnowledgeBaseImportFileContentExtractType::VisionModel),
            _ => Err(anyhow!("未知内容提取类型")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct KnowledgeBaseImportFileContentExtractModelConfig {
    pub(crate) model_id: i64,
}

crud!(KnowledgeBaseImportRecord {});
htmlsql_select_page!(list_page(param: &KnowledgeBaseImportRecordListReq) -> KnowledgeBaseImportRecord => "src/db/mapper/knowledge_base_import_record.html");
