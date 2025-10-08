use crate::server;

pub(crate) mod commands;
pub(crate) mod default;
mod request;
mod response;
mod service;
pub(crate) mod sync;

pub(crate) async fn init() {
    service::start_mcp_server_on_start().await;
}
