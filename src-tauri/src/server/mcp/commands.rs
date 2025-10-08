use crate::common::res::Res;
use crate::server::mcp::request::{McpServerAddReq, McpServerUpdateReq};
use crate::server::mcp::response::{McpServerDefineListRes, McpServerListRes};
use crate::server::mcp::service;

#[tauri::command]
pub(crate) async fn list_all_mcp_server() -> Res<Vec<McpServerDefineListRes>> {
    match service::list_all_mcp_server().await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn list_installed_mcp_server() -> Res<Vec<McpServerListRes>> {
    match service::list_installed_mcp_server().await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn install_mcp_server(name: String) -> Res<()> {
    match service::install_mcp_server(name).await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn uninstall_mcp_server(name: String) -> Res<()> {
    match service::uninstall_mcp_server(name).await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn run_mcp_server(name: String) -> Res<()> {
    match service::run_mcp_server(name).await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}
#[tauri::command]
pub(crate) async fn stop_mcp_server(name: String) -> Res<()> {
    match service::stop_mcp_server(name).await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn add_mcp_server(req: McpServerAddReq) -> Res<()> {
    match service::add_mcp_server(req).await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command]
pub(crate) async fn update_mcp_server(req: McpServerUpdateReq) -> Res<()> {
    match service::update_mcp_server(req).await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}
