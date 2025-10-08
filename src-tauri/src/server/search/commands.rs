use crate::common::res::Res;
use crate::server::search::request::SearchReq;
use crate::server::search::response::{SearchRes, SearchResultEvent};
use crate::server::search::service;
use tauri::ipc::Channel;

#[tauri::command]
pub(crate) async fn search(req: SearchReq, channel: Channel<SearchResultEvent>) -> Res<()> {
    match service::search(req, channel).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}
