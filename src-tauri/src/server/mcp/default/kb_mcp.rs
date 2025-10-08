use crate::db::model::knowledge_base::KnowledgeBase;
use crate::db::model::knowledge_base_import_record::{
    KnowledgeBaseImportRecord, KnowledgeBaseImportStatus,
};
use crate::db::Pool;
use crate::server::kb;
use crate::server::mcp::default;
use crate::{constant, server};
use anyhow::bail;
use engine::TableEngine;
use openai_dive::v1::resources::chat::{
    ChatCompletionFunction, ChatCompletionTool, ChatCompletionToolType,
};
use rbs::value;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KbDocSearchReq {
    knowledge_base_db_names: String,
    search_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KbDataSearchReq {
    knowledge_base_db_names: String,
    query_sql: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KbListItemReq {
    knowledge_base_db_name: String,
}
/// 检索文档型知识库
pub async fn kb_doc_search(parameters: &str) -> anyhow::Result<String> {
    let parameters = serde_json::from_str::<KbDocSearchReq>(parameters)?;
    let knowledge_base_db_names = parameters.knowledge_base_db_names;
    let table_names = knowledge_base_db_names.split(",").collect::<Vec<&str>>();
    let search_text = parameters.search_text;

    log::info!(
        "Knowledge base search: database name: {}, search text: {}",
        knowledge_base_db_names,
        search_text
    );

    let kbs =
        KnowledgeBase::select_by_map(Pool::get()?, value! {"table_name": table_names}).await?;

    let mut results = vec![];
    for kb in kbs {
        if let Some(knowledge_base_content) = kb::search(&kb, &search_text).await {
            results.push(format!(
                "数据库编号为【{}】的查询结果：{}",
                kb.table_name.unwrap(),
                knowledge_base_content
            ));
        }
    }

    Ok(results.join("\n\n"))
}

/// 检索数据型知识库
pub async fn kb_data_search(parameters: &str) -> anyhow::Result<String> {
    let parameters = serde_json::from_str::<KbDataSearchReq>(parameters)?;
    let knowledge_base_db_names = parameters.knowledge_base_db_names;
    let db_names = knowledge_base_db_names.split(",").collect::<Vec<&str>>();
    let mut results = vec![];
    for db_name in db_names {
        let query_sql = parameters.query_sql.clone();
        log::info!(
            "知识库搜索：知识库库名：{}，检索sql：{}",
            db_name,
            query_sql
        );

        let rows = TableEngine::query(db_name, &query_sql).await?;

        let result = rows
            .iter()
            .map(|row| row.join("\t"))
            .collect::<Vec<_>>()
            .join("\n");

        results.push(format!("数据库编号为【{}】的查询结果：{}", db_name, result));
    }

    log::info!("知识库搜索结果：{}", results.join("\n\n"));
    Ok(results.join("\n\n"))
}

pub(crate) async fn list_kb_items(parameters: &str) -> anyhow::Result<String> {
    let parameters = serde_json::from_str::<KbListItemReq>(parameters)?;
    let table_name = parameters.knowledge_base_db_name;
    let kb = KnowledgeBase::select_by_map(Pool::get()?, value! {"table_name": table_name}).await?;
    if kb.is_empty() {
        bail!("知识库不存在");
    }
    let kb = kb.first().unwrap();
    let list = KnowledgeBaseImportRecord::select_by_map(
        Pool::get()?,
        value! {"knowledge_base_id": kb.id.unwrap(),"status": KnowledgeBaseImportStatus::Success as i8},
    )
    .await?;
    let mut results = vec![];
    for item in list {
        results.push(item.nld.unwrap_or_default());
    }
    Ok(results.join("\n"))
}

pub(crate) struct KbMcp;
impl KbMcp {
    pub(crate) async fn call(&self, tool_name: &str, parameters: &str) -> anyhow::Result<String> {
        match tool_name {
            // 知识库的文档检索
            default::KB_DOC_SEARCH_TOOL => kb_doc_search(parameters).await,
            default::KB_TABLE_SEARCH_TOOL => kb_data_search(parameters).await,
            default::KB_LIST_ITEM_TOOL => list_kb_items(parameters).await,
            _ => {
                bail!("未知的工具名称：{}", tool_name);
            }
        }
    }
}

/// 知识库的默认工具，这些工具会在每次调用模型时传入
pub fn default_tools() -> Vec<ChatCompletionTool> {
    vec![
        ChatCompletionTool {
            r#type: ChatCompletionToolType::Function,
            function: ChatCompletionFunction {
                name: default::KB_DOC_SEARCH_TOOL.to_string(),
                description: Some(
                    "从知识库中检索【文档】类型的数据，当知识库类型为【文档】时，使用该工具"
                        .to_string(),
                ),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "knowledge_base_db_names":{
                            "type": "string",
                            "description": "知识库的数据库编码，多个用英文逗号分隔"
                        },
                        "search_text": {
                            "type": "string",
                            "description": "要检索的关键字文本"
                        }
                    },
                    "required":["knowledge_base_db_names","search_text"]
                }),
            },
        },
        ChatCompletionTool {
            r#type: ChatCompletionToolType::Function,
            function: ChatCompletionFunction {
                name: default::KB_TABLE_SEARCH_TOOL.to_string(),
                description: Some(
                    "从知识库中检索【表格】类型的数据，当知识库类型为【表格】时，使用该工具"
                        .to_string(),
                ),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "knowledge_base_db_names":{
                            "type": "string",
                            "description": "知识库的数据库编码，多个用英文逗号分隔"
                        },
                        "query_sql": {
                            "type": "string",
                            "description": "符合sqlite规范的完整的且正确的数据查询sql，所有表名、字段名需要用双引号包裹，所有结果列需要转为字符串类型"
                        }
                    },
                    "required":["knowledge_base_db_names","query_sql"]
                }),
            },
        },
        ChatCompletionTool {
            r#type: ChatCompletionToolType::Function,
            function: ChatCompletionFunction {
                name: default::KB_LIST_ITEM_TOOL.to_string(),
                description: Some("查询知识库中的内容列表".to_string()),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "knowledge_base_db_name":{
                            "type": "string",
                            "description": "知识库的数据库编码"
                        },
                    },
                    "required":["knowledge_base_db_name"]
                }),
            },
        },
    ]
}
