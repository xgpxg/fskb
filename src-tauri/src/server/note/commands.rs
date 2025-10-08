use crate::common::res::Res;
use crate::server::note::request::{NoteAddReq, NoteUpdateReq};
use crate::server::note::response::NoteListRes;
use crate::server::note::service;

#[tauri::command]
pub(crate) async fn add_note(req: NoteAddReq) -> Res<i64> {
    match service::add(req).await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(&e.to_string()),
    }
}
#[tauri::command]
pub(crate) async fn list_all_notes(
    kb_id: i64,
    filter_text: Option<String>,
) -> Res<Vec<NoteListRes>> {
    match service::list_all_notes(kb_id, filter_text).await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(&e.to_string()),
    }
}
#[tauri::command]
pub(crate) async fn delete_note(id: i64) -> Res<()> {
    match service::delete_note(id).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

#[tauri::command]
pub(crate) async fn update_note(req: NoteUpdateReq) -> Res<()> {
    match service::update_note(req).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

#[tauri::command]
pub(crate) async fn gen_note_title_and_summary(id: i64) -> Res<()> {
    match service::gen_note_title_and_summary(id).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}
