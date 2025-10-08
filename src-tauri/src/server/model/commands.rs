use crate::common::req::IdReq;
use crate::common::res::Res;
use crate::server::model::request::{ModelAddReq, ModelUpdateReq};
use crate::server::model::response::{ModelListRes, ModelSimpleListRes, OfflineModelListRes};
use crate::server::model::service;

/// 查询模型列表
#[tauri::command]
pub async fn list_all_models() -> Res<Vec<ModelListRes>> {
    match service::list_all().await {
        Ok(list) => Res::success(list),
        Err(e) => Res::error(&e.to_string()),
    }
}

/// 新增模型
#[tauri::command]
pub async fn add_model(req: ModelAddReq) -> Res<()> {
    match service::add(req).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

/// 更新模型
#[tauri::command]
pub async fn update_model(req: ModelUpdateReq) -> Res<()> {
    match service::update(req).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

/// 删除模型
#[tauri::command]
pub async fn delete_model(req: IdReq) -> Res<()> {
    match service::delete(req).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

/// 查询可用模型
///
/// 注意：该接口仅返回文本生成类模型
#[tauri::command]
pub async fn available_models() -> Res<Vec<ModelSimpleListRes>> {
    match service::available().await {
        Ok(list) => Res::success(list),
        Err(e) => Res::error(&e.to_string()),
    }
}

/// 查询所有可用模型
#[tauri::command]
pub async fn all_available_models(task_type: Option<i8>) -> Res<Vec<ModelSimpleListRes>> {
    match service::all_available(task_type).await {
        Ok(list) => Res::success(list),
        Err(e) => Res::error(&e.to_string()),
    }
}

/// 查询所有可用的离线模型
#[tauri::command]
pub async fn list_all_offline_models() -> Res<Vec<OfflineModelListRes>> {
    match service::list_all_offline_models().await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(&e.to_string()),
    }
}

#[tauri::command]
pub async fn install_offline_model(name: String) -> Res<()> {
    match service::install_offline_model(name).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

#[tauri::command]
pub async fn uninstall_offline_model(name: String) -> Res<()> {
    match service::uninstall_offline_model(name).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

#[tauri::command]
pub async fn run_offline_model(name: String) -> Res<()> {
    match service::run_offline_model(name).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

#[tauri::command]
pub async fn stop_offline_model(name: String) -> Res<()> {
    match service::stop_offline_model(name).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}
