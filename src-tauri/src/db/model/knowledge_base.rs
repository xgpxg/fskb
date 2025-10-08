use crate::db::model::knowledge_base_import_record::{
    KnowledgeBaseImportFileContentExtractModelConfig, KnowledgeBaseImportFileContentExtractType,
};
use anyhow::anyhow;
use derive_builder::Builder;
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, htmlsql};
use serde::{Deserialize, Deserializer, Serialize};

/// 知识库
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
pub struct KnowledgeBase {
    pub id: Option<i64>,
    /// 知识库名称
    pub name: Option<String>,
    /// 知识库描述
    pub description: Option<String>,
    /// 图标
    pub icon: Option<String>,
    /// 知识库来源
    pub source: Option<i8>,
    /// 对应的向量数据库的表名
    pub table_name: Option<String>,
    /// 知识库的自然语言描述（Natural language description）
    /// 对知识库的增删改操作都需要维护这个字段
    pub nld: Option<String>,
    /// 配置，json格式
    #[serde(deserialize_with = "deserialize_config")]
    pub config: Option<KnowledgeBaseConfig>,
    /// MCP工具
    pub mcp_server_ids: Option<Vec<i64>>,
    /// 语言模型ID
    pub model_id: Option<i64>,
    /// 文件内容提取方式
    pub file_content_extract_type: Option<KnowledgeBaseImportFileContentExtractType>,
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

/// 知识库来源
#[allow(unused)]
pub enum KnowledgeBaseSource {
    /// 平台内置
    Platform = 1,
    /// 自定义
    Custom = 2,
}

pub enum KnowledgeVisibility {
    OnlySelf = 1,
    PublicRead = 2,
    PublicReadWrite = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeBaseConfig {
    /// 相似度阈值
    pub search_min_score: f32,
    /// 搜索结果数量限制
    pub search_limit: usize,
    /// 搜索扩展分段数量
    pub search_extend_size: usize,
    /// rerank开关
    pub is_rerank: bool,
    /// rerank结果数量限制
    pub rerank_limit: usize,
}
impl Default for KnowledgeBaseConfig {
    fn default() -> Self {
        KnowledgeBaseConfig {
            search_min_score: 0.7,
            search_limit: 10,
            search_extend_size: 1,
            is_rerank: false,
            rerank_limit: 3,
        }
    }
}

impl KnowledgeBase {
    pub fn get_config(&self) -> KnowledgeBaseConfig {
        if let Some(config) = &self.config {
            config.clone()
        } else {
            KnowledgeBaseConfig::default()
        }
    }
}

fn deserialize_config<'de, D>(deserializer: D) -> Result<Option<KnowledgeBaseConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(Some(KnowledgeBaseConfig::default())),
        _ => Ok(Some(
            serde_json::from_str(&value.to_string()).unwrap_or_else(|e| {
                log::error!("Deserialize knowledge config error: {}", e);
                KnowledgeBaseConfig::default()
            }),
        )),
    }
}

crud!(KnowledgeBase {});
htmlsql!(remove_mcp_server_id(rb: &dyn Executor, mcp_server_id: i64) -> Option<u32> => "src/db/mapper/knowledge_base.html");
