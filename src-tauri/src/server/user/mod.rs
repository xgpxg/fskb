use crate::db::model::user_profile::UserProfile;
use crate::db::Pool;

pub(crate) mod commands;
pub(crate) mod memory;
mod request;
mod response;
mod service;

pub(crate) struct User {}

impl User {
    pub(crate) async fn profile() -> UserProfile {
        let profile = UserProfile::select_all(Pool::get().unwrap()).await.unwrap();
        if profile.is_empty() {
            return UserProfile::default();
        }
        let profile = profile[0].clone();
        profile
    }
    pub(crate) async fn enable_profile_memory() -> bool {
        Self::profile().await.enable_profile_memory.unwrap_or(0) == 1
    }
    pub(crate) async fn profile_memory_model_id() -> Option<i64> {
        Self::profile().await.profile_memory_model_id
    }
}
