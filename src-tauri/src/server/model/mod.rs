pub(crate) mod commands;
pub(crate) mod request;
pub(crate) mod response;
mod service;

pub(crate) use service::get_model_by_name;
pub(crate) async fn init() {
    service::start_offline_model_on_start().await;
}
