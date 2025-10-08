use crate::common::result::{AppError, AppResult};
use crate::db_error;
use common::{app_dir, data_dir};
use fastdate::{set_offset_sec, DurationFrom};
use rbatis::executor::RBatisTxExecutor;
use rbatis::RBatis;
use rbdc_pool_fast::FastPool;
use rbdc_sqlite::options::SqliteConnectOptions;
use rbdc_sqlite::SqliteDriver;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::ops::Deref;
use std::process::exit;
use std::str::FromStr;
use std::sync::OnceLock;
use std::time::Duration;

mod migrations;
pub(crate) mod model;
pub(crate) mod tools;

static RB: OnceLock<RBatis> = OnceLock::new();

pub struct Pool;
impl Pool {
    pub fn get<'a>() -> AppResult<&'a RBatis> {
        match RB.get() {
            None => {
                log::error!("rbatis not init");
                let bt = backtrace::Backtrace::new();
                log::error!("Backtrace:\n{:?}", bt);
                Err(AppError::InitError("rbatis not init".to_string()))
            }
            Some(rb) => Ok(rb),
        }
    }
}

//const DB_URL: &str = "sqlite://data/sqlite/fs-kb-server.db";
pub async fn init() {
    let db_url = &data_dir!("sqlite", "fs-kb-app.db")
        .to_string_lossy()
        .into_owned();
    log::info!("db_url: {}", db_url);
    let db_url = &format!("sqlite://{}", db_url);
    // 设置时区
    set_offset_sec(Duration::from_hour(8).as_secs() as i32);

    let rb = RBatis::new();
    let opts = SqliteConnectOptions::from_str(db_url).unwrap();
    if let Err(e) =
        rb.init_option::<SqliteDriver, SqliteConnectOptions, FastPool>(SqliteDriver {}, opts)
    {
        log::error!("rbatis init error: {}", e);
        exit(1);
    }

    rb.exec(include_str!("sql/init.sql"), vec![])
        .await
        .map_err(|e| {
            log::error!("rbatis init error: {}", e);
            exit(1);
        })
        .unwrap();

    // 执行升级
    migrations::run(&mut rb.clone()).await;

    log::info!("rbatis init success");
    RB.get_or_init(|| rb);
}

/// 执行事务闭包
/// - exec：闭包，返回Ok则提交事务，否则回滚
pub async fn tx<'a, F, R, RV>(exec: F) -> AppResult<RV>
where
    F: Fn(RBatisTxExecutor) -> R,
    R: Future<Output = AppResult<RV>>,
{
    let tx = match Pool::get()?.acquire_begin().await {
        Ok(tx) => tx,
        Err(e) => {
            log::error!("事务异常: {}", e);
            return Err(db_error!(e));
        }
    };

    let result = exec(tx.clone()).await;

    match result {
        Ok(result) => {
            match tx.commit().await {
                Ok(_) => log::debug!("事务提交成功，事务ID：{}", tx.tx_id),
                Err(e) => {
                    log::error!("事务提交失败，事务ID：{}， 原因： {}", tx.tx_id, e);
                    return Err(db_error!(e));
                }
            };
            Ok(result)
        }
        Err(e) => {
            log::debug!("事务闭包执行失败，即将回滚，错误原因: {}", e);
            match tx.rollback().await {
                Ok(_) => {
                    log::debug!("事务回滚成功，事务ID：{}", tx.tx_id);
                    Err(e)
                }
                Err(e) => {
                    log::error!("事务回滚失败，事务ID：{}， 原因： {}", tx.tx_id, e);
                    Err(db_error!(e))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Count {
    pub count: usize,
}

impl Deref for Count {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.count
    }
}
