use crate::common::res::Res;
use crate::db::model::chat_message;
use crate::server::chat::request::UserMessageContent;
use crate::server::chat::{chat_helper, service, ChatEvent};
use tauri::ipc::Channel;

#[tauri::command]
pub(crate) async fn chat(
    kb_id: i64,
    content: UserMessageContent,
    channel: Channel<ChatEvent>,
) -> Res<()> {
    match service::chat(kb_id, content, channel).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn resume(kb_id: i64, message_id: i64, channel: Channel<ChatEvent>) -> Res<()> {
    match service::resume(kb_id, message_id, channel).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn list_all_history_messages(
    kb_id: i64,
    last_message_id: Option<i64>,
) -> Res<Vec<chat_message::ChatMessage>> {
    match service::list_history_messages(kb_id, last_message_id).await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn copy_chat_file_to_data_dir(
    kb_id: i64,
    path: String,
) -> Result<Res<String>, String> {
    chat_helper::copy_chat_file_to_data_dir(kb_id, path).await
}

#[tauri::command]
pub(crate) async fn save_chat_file_to_data_dir(
    kb_id: i64,
    file_name: String,
    bytes: Vec<u8>,
) -> Result<Res<String>, String> {
    chat_helper::save_chat_file_to_data_dir(kb_id, file_name, bytes).await
}

#[tauri::command]
pub(crate) async fn clear_message(kb_id: i64) -> Res<()> {
    match service::clear_message(kb_id).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn delete_message(id: i64) -> Res<()> {
    match service::delete_message(id).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}
