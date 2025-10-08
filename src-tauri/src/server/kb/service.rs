use crate::common::id;
use crate::common::req::Pagination;
use crate::common::res::{IntoPageRes, PageRes};
use crate::db::model::knowledge_base::{KnowledgeBase, KnowledgeBaseBuilder, KnowledgeBaseSource};
use crate::db::model::knowledge_base_import_record;
use crate::db::model::knowledge_base_import_record::{
    KnowledgeBaseImportFileContentExtractType, KnowledgeBaseImportFileContentType,
    KnowledgeBaseImportRecord, KnowledgeBaseImportRecordBuilder, KnowledgeBaseImportSource,
    KnowledgeBaseImportStatus,
};
use crate::db::{tools, Pool};
use crate::server::kb::parse;
use crate::server::kb::request::{
    KbAddReq, KnowledgeBaseImportRecordListReq, KnowledgeBaseUpdateReq,
};
use crate::server::kb::response::{
    KnowledgeBaseDetailRes, KnowledgeBaseImportRecordListRes, KnowledgeBaseListRes,
};
use crate::utils::file_util::make_save_file;
use crate::{business_error, db, db_error, message_error};
use anyhow::{bail, Context};
use engine::{Engine, TableEngine};
use rbatis::executor::RBatisTxExecutor;
use rbs::value;
use std::fs;
use std::path::Path;

pub(crate) async fn add_kb(req: KbAddReq) -> anyhow::Result<()> {
    db::tx(|tx| {
        let req = req.clone();
        async move {
            let table_name = uuid::Uuid::new_v4().to_string();
            let kb = KnowledgeBaseBuilder::default()
                .id(Some(id::next()))
                .name(Some(req.name))
                .description(req.description)
                .icon(req.icon)
                .source(Some(KnowledgeBaseSource::Custom as i8))
                .table_name(Some(table_name.clone()))
                .file_content_extract_type(Some(KnowledgeBaseImportFileContentExtractType::Ocr))
                .build()
                .unwrap();
            KnowledgeBase::insert(Pool::get()?, &kb)
                .await
                .map_err(|e| db_error!(e))?;

            // 创建知识库文档表
            // 一个知识库只有1个文档表，用于相似性搜索，但可以有多个数据表，数据表使用sql精确查询
            Engine::new_table(&table_name).await.map_err(|e| {
                log::error!("[创建知识库表]失败，原因：{}", e);
                message_error!("创建知识库表失败")
            })?;

            // 重建nld
            rebuild_nld(&tx, kb.id.unwrap())
                .await
                .map_err(|e| message_error!(e))?;

            Ok(())
        }
    })
    .await?;
    Ok(())
}

// 构建知识库的自然语言描述
// 这些描述信息会传给LLM，由LLM进行相关决策
async fn rebuild_nld(tx: &RBatisTxExecutor, id: i64) -> anyhow::Result<()> {
    //获取知识库
    let knowledge_base = KnowledgeBase::select_by_map(tx, value! {"id": id}).await?;
    let knowledge_base = knowledge_base.first().cloned().unwrap();
    //获取知识库导入记录
    // let import_records = KnowledgeBaseImportRecord::select_by_map(
    //     tx,
    //     value! {"knowledge_base_id":  knowledge_base.id},
    // )
    // .await?;

    let mut nld = vec![];
    nld.push(format!(
        "知识库名称：{}",
        knowledge_base.name.unwrap_or("未知".to_string())
    ));
    nld.push(format!(
        "数据库编码：{}",
        knowledge_base
            .table_name
            .clone()
            .unwrap_or("未知".to_string())
    ));
    nld.push(format!("类型：{}", "文档".to_string()));
    nld.push(format!(
        "描述：{}",
        knowledge_base.description.clone().unwrap_or("".to_string())
    ));

    // import_records
    //     .iter()
    //     .for_each(|item| nld.push(item.nld.clone().unwrap_or("未知".to_string())));

    let nld = nld.join("；");
    KnowledgeBase::update_by_map(
        tx,
        &KnowledgeBaseBuilder::default()
            .id(Some(id))
            .nld(Some(nld.clone()))
            .build()
            .unwrap(),
        value! {"id": id},
    )
    .await?;

    Ok(())
}

pub(crate) async fn list_all_kb() -> anyhow::Result<Vec<KnowledgeBaseListRes>> {
    let mut list = KnowledgeBase::select_all(Pool::get()?).await?;
    list.sort_by(|a, b| a.id.cmp(&b.id).reverse());
    let list = list
        .into_iter()
        .map(|kb| KnowledgeBaseListRes { inner: kb })
        .collect::<Vec<_>>();

    Ok(list)
}

pub(crate) async fn delete_kb(id: i64) -> anyhow::Result<()> {
    KnowledgeBase::delete_by_map(Pool::get()?, value! {"id": id}).await?;
    Ok(())
}

pub(crate) async fn update_kb(req: KnowledgeBaseUpdateReq) -> anyhow::Result<()> {
    db::tx(|tx| {
        let req = req.clone();
        async move {
            let kb = KnowledgeBaseBuilder::default()
                .id(Some(req.id))
                .name(req.name)
                .description(req.description)
                .icon(req.icon)
                .mcp_server_ids(req.mcp_server_ids)
                .model_id(req.model_id)
                .file_content_extract_type(req.file_content_extract_type)
                .update_time(Some(tools::now()))
                .build()
                .unwrap();
            KnowledgeBase::update_by_map(Pool::get()?, &kb, value! {"id": req.id})
                .await
                .map_err(|e| db_error!(e))?;

            // 重建nld
            rebuild_nld(&tx, kb.id.unwrap())
                .await
                .map_err(|e| message_error!(e))?;
            Ok(())
        }
    })
    .await?;

    Ok(())
}

pub(crate) async fn kb_detail(id: i64) -> anyhow::Result<KnowledgeBaseDetailRes> {
    let kb = KnowledgeBase::select_by_map(Pool::get()?, value! {"id": id}).await?;
    if kb.is_empty() {
        bail!("知识库不存在");
    }
    let kb = kb.first().unwrap();
    Ok(KnowledgeBaseDetailRes { inner: kb.clone() })
}

pub(crate) async fn add_kb_file(kb_id: i64, files: Vec<String>) -> anyhow::Result<()> {
    log::info!("add_kb_file: {:?}", files);
    let mut records = vec![];
    let kb = KnowledgeBase::select_by_map(Pool::get()?, value! {"id": kb_id}).await?;
    if kb.is_empty() {
        bail!("知识库不存在");
    }
    let kb = kb.first().unwrap();
    for file in files {
        let file = Path::new(&file);
        let original_file_name = file.file_name().unwrap().to_str().unwrap();
        let original_file_path = file.to_str().unwrap();
        let file_size = file.metadata()?.len();

        let (file_name, file_path) = make_save_file(original_file_name)?;

        // 复制文件
        fs::copy(original_file_path, &file_path)?;

        // 导入记录的nld
        let nld = format!("文件名：{}，路径：{}", file_name, file_path);
        // 生成导入记录，状态为待解析
        let record = KnowledgeBaseImportRecordBuilder::default()
            .id(Some(id::next()))
            .knowledge_base_id(Some(kb_id))
            .title(Some(original_file_name.to_string()))
            .original_file_name(Some(original_file_name.to_string()))
            .original_file_path(Some(original_file_path.to_string()))
            .file_name(Some(file_name.to_string()))
            .file_path(Some(file_path.to_string()))
            .file_size(Some(file_size))
            .file_content_type(Some(KnowledgeBaseImportFileContentType::Document as i8))
            .file_content_extract_type(kb.file_content_extract_type.clone())
            .source(Some(KnowledgeBaseImportSource::LocalFile as i8))
            .status(Some(KnowledgeBaseImportStatus::Importing as i8))
            .nld(Some(nld))
            .build()?;
        records.push(record);
    }

    KnowledgeBaseImportRecord::insert_batch(Pool::get()?, &records, 10).await?;

    records.into_iter().for_each(|mut record| {
        tokio::spawn(async move {
            // 解析文件
            match record.parse().await {
                Ok(_) => {
                    log::info!("parse file success: {}", record.file_name.as_ref().unwrap());
                    record.status = Some(KnowledgeBaseImportStatus::Success as i8);
                }
                Err(e) => {
                    log::error!("parse file error: {}", e);
                    record.status = Some(KnowledgeBaseImportStatus::Failed as i8);
                    record.status_msg = Some(e.to_string());
                }
            }
            // 更新导入记录，状态为完成
            KnowledgeBaseImportRecord::update_by_map(
                Pool::get()?,
                &record,
                value! {"id": record.id},
            )
            .await
            .context("更新导入记录失败")?;

            Ok::<(), anyhow::Error>(())
        });
    });
    Ok(())
}

pub(crate) async fn kb_import_record_list(
    req: KnowledgeBaseImportRecordListReq,
) -> anyhow::Result<PageRes<KnowledgeBaseImportRecordListRes>> {
    let page =
        knowledge_base_import_record::list_page(Pool::get()?, &req.to_rb_page(), &req).await?;
    let res = page.convert_to_page_res(|list| {
        list.into_iter()
            .map(|item| KnowledgeBaseImportRecordListRes {
                id: item.id,
                knowledge_base_id: item.knowledge_base_id,
                title: item.title,
                original_file_name: item.original_file_path,
                original_file_path: item.original_file_name,
                file_name: item.file_name,
                file_path: item.file_path,
                file_size: item.file_size,
                url: item.url,
                source: item.source,
                status: item.status,
                status_msg: item.status_msg,
                use_time: if let Some(start_time) = item.start_time {
                    // 如果是导入中，则使用当前时间减去开始时间
                    let end_time =
                        if item.status == Some(KnowledgeBaseImportStatus::Importing as i8) {
                            tools::now()
                        } else {
                            item.end_time.unwrap_or(tools::now())
                        };
                    Some((end_time - start_time).as_millis() as usize)
                } else {
                    None
                },
            })
            .collect::<Vec<_>>()
    });
    Ok(res)
}

pub(crate) async fn remove_mcp_server_id(mcp_server_id: i64) -> anyhow::Result<()> {
    KnowledgeBase::select_by_map(Pool::get()?, value! {"id": mcp_server_id}).await?;
    Ok(())
}

pub(crate) async fn delete_kb_import_record(id: i64) -> anyhow::Result<()> {
    db::tx(|tx| async move {
        let record = KnowledgeBaseImportRecord::select_by_map(&tx, value! {"id": id})
            .await
            .map_err(|e| db_error!(e))?;
        if record.is_empty() {
            return Err(message_error!("记录不存在"));
        }
        let record = record.first().unwrap();
        if let Some(file_path) = &record.file_path {
            if let Err(e) = fs::remove_file(file_path) {
                log::error!("[kb] File deletion failed, reason: {}", e);
            }
        }

        let table_name = parse::get_table_name(record.knowledge_base_id.unwrap())
            .await
            .map_err(|e| business_error!(e))?;

        // 按batch_id从向量数据库中删除
        Engine::delete_data(&table_name, vec![record.id.unwrap().to_string()])
            .await
            .map_err(|e| {
                log::error!(
                    "[kb] Failed to delete data from vector database, reason: {}",
                    e
                );
                message_error!("删除记录失败")
            })?;

        // TODO 清理ref文件

        TableEngine::drop_table(&table_name, &record.title.clone().unwrap())
            .await
            .map_err(|e| {
                log::error!("[kb] Failed to delete data table, reason: {}", e);
                message_error!("删除记录失败")
            })?;

        KnowledgeBaseImportRecord::delete_by_map(&tx, value! {"id": id})
            .await
            .map_err(|e| db_error!(e))?;

        // 重建nld
        rebuild_nld(&tx, record.knowledge_base_id.unwrap())
            .await
            .map_err(|e| message_error!(e))?;

        Ok(())
    })
    .await?;
    Ok(())
}
