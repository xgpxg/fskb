use model::ModelManager;
use tauri::async_runtime::block_on;
use tauri::plugin::{Builder, TauriPlugin};

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    Builder::new("shutdown")
        .on_event(|app_handle, event| match event {
            tauri::RunEvent::Exit => {}
            tauri::RunEvent::ExitRequested { .. } => {
                block_on(async {
                    shutdown().await;
                });
            }
            _ => {}
        })
        .build()
}

async fn shutdown() {
    // 关闭本地模型
    log::info!("shutdown model");
    ModelManager::shutdown().await;
}
