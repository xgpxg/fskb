use crate::constant;
use crate::db::model::knowledge_base::KnowledgeBase;
use crate::db::model::knowledge_base_import_record::{
    KnowledgeBaseImportRecord, KnowledgeBaseImportRecordBuilder,
};
use crate::db::Pool;
use crate::server::search::request::SearchReq;
use crate::server::search::response::{KbSearchItemRes, KbSearchRes, SearchRes};
use embedding::{Embedding, EmbeddingInput, Embeddings};
use engine::{Engine, SearchRequestBuilder};
use rbs::value;
use std::collections::HashMap;

pub(crate) async fn search(req: &SearchReq) -> anyhow::Result<KbSearchRes> {
    let kw = req.kw.as_str();
    if kw.is_empty() {
        return Ok(KbSearchRes::default());
    }

    let vector = Embeddings::embedding(EmbeddingInput::Text(format!(
        "{}{}",
        constant::TEXT_SEARCH_INSTRUCTION,
        kw
    )))
    .unwrap();

    // 获取所有知识库表名
    let kbs = KnowledgeBase::select_all(Pool::get()?).await?;
    let table_names = kbs
        .iter()
        .map(|item| item.table_name.clone().unwrap_or("未知".to_string()))
        .collect::<Vec<_>>();

    // 将kbs按table_name转为map
    let kb_map = kbs
        .iter()
        .map(|item| {
            (
                item.table_name.clone().unwrap_or("未知".to_string()),
                item.clone(),
            )
        })
        .collect::<HashMap<_, _>>();
    if table_names.is_empty() {
        return Ok(KbSearchRes::default());
    }

    let mut items = Vec::new();

    for table_name in table_names.iter() {
        let list = Engine::search_data(
            SearchRequestBuilder::default()
                .table_name(table_name.clone())
                .vector(Some(vector.clone()))
                .limit(Some(20))
                .build()
                .unwrap(),
        )
        .await?;
        let list = list
            .into_iter()
            .map(|item| KbSearchItemRes {
                content: item.content,
                score: item.score,
                ref_kb: kb_map.get(table_name).unwrap().clone(),
                ref_import_record: KnowledgeBaseImportRecordBuilder::default()
                    .id(Some(item.batch_id.parse::<i64>().unwrap()))
                    .build()
                    .unwrap(),
            })
            .collect::<Vec<_>>();
        items.extend(list);
    }

    // 查询出处
    let references = KnowledgeBaseImportRecord::select_by_map(
        Pool::get()?,
        value! {
            "id": &items.iter().map(|item| &item.ref_import_record.id).collect::<Vec<_>>(),
        },
    )
    .await?
    .into_iter()
    .map(|item| (item.id.unwrap(), item))
    .collect::<HashMap<i64, KnowledgeBaseImportRecord>>();

    let mut items = items
        .into_iter()
        .map(|mut item| {
            item.ref_import_record = references
                .get(&item.ref_import_record.id.unwrap())
                .unwrap()
                .clone();
            item
        })
        .collect::<Vec<_>>();

    // 排序
    items.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    Ok(KbSearchRes { items })
}
