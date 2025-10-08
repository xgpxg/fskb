use crate::db::migrations::{AppVersion, Migrator};
use rbatis::RBatis;

pub(crate) struct Handler;

impl Migrator for Handler {
    fn version(&self) -> AppVersion {
        AppVersion::V0_1_0
    }

    async fn up(&self, conn: &mut RBatis) -> anyhow::Result<()> {
        todo!()
    }
}
