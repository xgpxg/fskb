use crate::db::model::user_profile::{UserProfile, UserProfileBuilder};
use crate::db::Pool;
use crate::server::user::request::UserProfileUpdateReq;
use crate::server::user::response::UserProfileRes;
use rbs::value;

pub(crate) async fn update_user_profile(req: UserProfileUpdateReq) -> anyhow::Result<()> {
    let profile = UserProfileBuilder::default()
        .enable_profile_memory(req.enable_profile_memory)
        .profile_memory_model_id(req.profile_memory_model_id)
        .build()?;
    // 目前单用户，不传条件，更新整个表，表里实际上只有一条数据
    UserProfile::update_by_map(Pool::get()?, &profile, value! {}).await?;
    Ok(())
}

pub(crate) async fn get_user_profile() -> anyhow::Result<UserProfileRes> {
    let profile = UserProfile::select_all(Pool::get()?).await?;
    if profile.is_empty() {
        return Ok(UserProfileRes::default());
    }

    let profile = profile[0].clone();
    Ok(UserProfileRes {
        enable_profile_memory: profile.enable_profile_memory,
        profile_memory_model_id: profile.profile_memory_model_id,
    })
}
