use crate::common::res::{PageRes, Res};
use crate::server::kb::request::{
    KbAddReq, KnowledgeBaseImportRecordListReq, KnowledgeBaseUpdateReq,
};
use crate::server::kb::response::{
    KnowledgeBaseDetailRes, KnowledgeBaseImportRecordListRes, KnowledgeBaseListRes,
};
use crate::server::kb::service;

#[tauri::command]
pub(crate) fn hello(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub(crate) async fn add_kb(req: KbAddReq) -> Res<()> {
    match service::add_kb(req).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn list_all_kb() -> Res<Vec<KnowledgeBaseListRes>> {
    match service::list_all_kb().await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn delete_kb(id: i64) -> Res<()> {
    match service::delete_kb(id).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn update_kb(req: KnowledgeBaseUpdateReq) -> Res<()> {
    match service::update_kb(req).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn kb_detail(id: i64) -> Res<KnowledgeBaseDetailRes> {
    match service::kb_detail(id).await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn add_kb_file(kb_id: i64, files: Vec<String>) -> Res<()> {
    match service::add_kb_file(kb_id, files).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn kb_import_record_list(
    req: KnowledgeBaseImportRecordListReq,
) -> Res<PageRes<KnowledgeBaseImportRecordListRes>> {
    match service::kb_import_record_list(req).await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn delete_kb_import_record(id: i64) -> Res<()> {
    match service::delete_kb_import_record(id).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}
