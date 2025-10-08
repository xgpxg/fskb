use crate::VERSION;
use common::app_dir;
use std::fs;
use updater::app_version::AppVersion;

#[tauri::command]
pub(crate) async fn get_current_version() -> String {
    VERSION.to_string()
}

#[tauri::command]
pub(crate) async fn get_latest_version() -> Result<AppVersion, String> {
    updater::app_version::get_latest_version()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub(crate) async fn make_update_flag_file() -> Result<(), String> {
    // 生成.update文件，下次启动时触发更新
    let update_lock_file = app_dir!(".update");
    fs::write(&update_lock_file, "1").map_err(|e| e.to_string())?;
    Ok(())
}
