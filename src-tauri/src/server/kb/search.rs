use crate::constant;
use crate::db::model::knowledge_base::KnowledgeBase;
use embedding::{Embedding, EmbeddingInput, Embeddings, Reranker};
use engine::Engine;

pub(crate) async fn search(kb: &KnowledgeBase, content: &String) -> Option<String> {
    let vector = Embeddings::embedding(EmbeddingInput::Text(format!(
        "{}{}",
        constant::TEXT_SEARCH_INSTRUCTION,
        content
    )))
    .unwrap();
    // 知识库配置
    let kb_config = kb.get_config();

    log::debug!("Knowledge base config: {:?}", kb_config);

    match Engine::simple_search(
        kb.table_name.clone().unwrap().as_str(),
        vector,
        kb_config.search_extend_size,
        kb_config.search_min_score,
        kb_config.search_limit,
    )
    .await
    {
        Ok(list) => {
            if list.is_empty() {
                return None;
            }
            log::info!(
                "Retrieved {} similar entries from knowledge base",
                list.len()
            );
            // 检索出的文本列表
            let contents = list
                .into_iter()
                .map(|item| item.content)
                .collect::<Vec<_>>();

            let list = if kb_config.is_rerank {
                // 重排结果
                let rerank_results = match Reranker::rerank(content.to_string(), contents.clone()) {
                    Ok(results) => results
                        .into_iter()
                        .take(kb_config.rerank_limit)
                        .map(|(index, _)| contents[index].clone())
                        .collect::<Vec<_>>(),
                    Err(e) => {
                        log::error!("Rerank error: {}", e);
                        // 重排失败，使用原始顺序
                        contents
                            .into_iter()
                            .take(kb_config.rerank_limit)
                            .collect::<Vec<_>>()
                    }
                };
                log::info!("Rerank {} similar entries", rerank_results.len());
                rerank_results
            } else {
                contents
            };

            let context = list.join("\n");
            Some(context)
        }
        Err(e) => {
            log::error!("知识库检索失败：{}", e);
            None
        }
    }
}
