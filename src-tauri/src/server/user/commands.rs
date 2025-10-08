use crate::common::res::Res;
use crate::server::user::request::UserProfileUpdateReq;
use crate::server::user::response::UserProfileRes;
use crate::server::user::service;

#[tauri::command]
pub(crate) async fn update_user_profile(req: UserProfileUpdateReq) -> Res<()> {
    match service::update_user_profile(req).await {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

#[tauri::command]
pub(crate) async fn get_user_profile() -> Res<UserProfileRes> {
    match service::get_user_profile().await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(&e.to_string()),
    }
}
