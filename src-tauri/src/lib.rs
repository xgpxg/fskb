use ::common::dir::AppDir;
use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::LazyLock;
use tauri::async_runtime::block_on;

mod common;
mod config;
mod constant;
mod db;
mod plugins;
mod server;
mod setup;
mod task;
mod updater;
mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    config::log_config::init_log();

    tauri::Builder::default()
        .setup(|app| {
            setup::setup(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(plugins::tauri_plugin_shutdown::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            server::kb::commands::hello,
            server::kb::commands::add_kb,
            server::kb::commands::list_all_kb,
            server::kb::commands::delete_kb,
            server::kb::commands::update_kb,
            server::kb::commands::kb_detail,
            server::kb::commands::add_kb_file,
            server::kb::commands::kb_import_record_list,
            server::kb::commands::delete_kb_import_record,
            server::chat::commands::chat,
            server::chat::commands::resume,
            server::chat::commands::list_all_history_messages,
            server::chat::commands::copy_chat_file_to_data_dir,
            server::chat::commands::save_chat_file_to_data_dir,
            server::chat::commands::clear_message,
            server::chat::commands::delete_message,
            server::model::commands::list_all_models,
            server::model::commands::add_model,
            server::model::commands::update_model,
            server::model::commands::delete_model,
            server::model::commands::available_models,
            server::model::commands::all_available_models,
            server::model::commands::list_all_offline_models,
            server::model::commands::install_offline_model,
            server::model::commands::uninstall_offline_model,
            server::model::commands::run_offline_model,
            server::model::commands::stop_offline_model,
            server::mcp::commands::list_all_mcp_server,
            server::mcp::commands::list_installed_mcp_server,
            server::mcp::commands::install_mcp_server,
            server::mcp::commands::uninstall_mcp_server,
            server::mcp::commands::run_mcp_server,
            server::mcp::commands::stop_mcp_server,
            server::mcp::commands::add_mcp_server,
            server::mcp::commands::update_mcp_server,
            common::commands::save_temp_file,
            common::commands::download,
            common::commands::cancel_download,
            common::commands::copy_file_to_file_dir,
            common::commands::save_file_to_file_dir,
            common::commands::save_note_file,
            server::search::commands::search,
            server::user::commands::update_user_profile,
            server::user::commands::get_user_profile,
            server::note::commands::add_note,
            server::note::commands::list_all_notes,
            server::note::commands::update_note,
            server::note::commands::delete_note,
            server::note::commands::gen_note_title_and_summary,
            updater::get_current_version,
            updater::get_latest_version,
            updater::make_update_flag_file,
            server::components::commands::gen_share_card,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_dir() -> anyhow::Result<()> {
    let data_dir = AppDir::data_dir();
    fs::create_dir_all(data_dir).context("Failed to create data directory")?;

    fs::create_dir_all(data_dir.join("database")).context("Failed to create database directory")?;
    fs::create_dir_all(data_dir.join("sqlite")).context("Failed to create sqlite directory")?;
    fs::create_dir_all(data_dir.join("cache")).context("Failed to create cache directory")?;
    fs::create_dir_all(data_dir.join("file")).context("Failed to create file directory")?;
    fs::create_dir_all(data_dir.join("mcp")).context("Failed to create mcp directory")?;
    fs::create_dir_all(data_dir.join("temp")).context("Failed to create temp directory")?;
    fs::create_dir_all(data_dir.join("model")).context("Failed to create model directory")?;
    fs::create_dir_all(data_dir.join("user")).context("Failed to create user directory")?;
    fs::create_dir_all(data_dir.join("note")).context("Failed to create note directory")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::log_config;
    use crate::db;
    use crate::db::model::knowledge_base_import_record::KnowledgeBaseImportFileContentExtractType;

    #[tokio::test]
    pub async fn test_upload() {
        crate::utils::file_util::upload("data/temp/1.png")
            .await
            .unwrap();
    }

    #[tokio::test]
    pub async fn test_enum() {
        println!(
            "{:?}",
            serde_json::to_string(&KnowledgeBaseImportFileContentExtractType::VisionModel {
                model_id: 1
            })
        );
    }

    #[tokio::test]
    pub async fn test_sqlite() {
        log_config::init_log();
        db::init().await;
    }
}
