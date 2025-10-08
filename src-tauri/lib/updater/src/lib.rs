use crate::app_version::{get_latest_version, get_resource_version_with_app_version, get_version};
use common::{app_dir, resources_dir};
use serde::{Deserialize, Serialize};
use std::fs;

mod app_downloader;
pub mod app_version;
mod downloader;
mod resources_downloader;

pub enum Phase {
    CheckUpdate { current_version: String },
    CheckResources { current_version: String },
}
pub enum PhaseStatus<'a> {
    Skip(&'a str),
    Running {
        downloaded: u64,
        total: u64,
        progress: f64,
    },
    Finished {
        file_path: Option<String>,
    },
    Error(String),
}

impl Phase {
    /// 检查是否需要显示启动屏
    pub async fn check(version: &str) -> anyhow::Result<bool> {
        Ok(
            // 资源目录存在
            Self::check_resource_dir_exists().await?
                // 资源目录版本与应用要求的资源版本一致
                && Self::check_resource_version(version).await?
                // 用户未允许更新 或 用户允许更新 且 当前版本不是最新版本
                && (!Self::check_enable_update().await?|| Self::check_is_latest_version(version).await?
            ),
        )
    }

    /// 检查资源目录是否存在
    async fn check_resource_dir_exists() -> anyhow::Result<bool> {
        let exists = resources_dir!().exists();
        log::info!("check resource dir exists: {}", exists);
        Ok(exists)
    }

    /// 检查资源版本
    /// - version: 应用版本（注意不是资源版本）
    async fn check_resource_version(version: &str) -> anyhow::Result<bool> {
        let resource_version = resources_dir!().join(".version");
        if !resource_version.exists() {
            log::info!("check resource version, resource version file not exists");
            return Ok(false);
        }
        // 本地的资源版本
        let resource_version = fs::read_to_string(resource_version)?;
        // 应用版本需要的资源版本
        let app_version = get_version(version).await?;
        log::info!(
            "check resource version, expect: {}, current: {}",
            app_version.resource_version,
            resource_version
        );
        Ok(app_version.resource_version == resource_version)
    }

    /// 检查是否是最新版本
    /// - version: 应用版本
    async fn check_is_latest_version(version: &str) -> anyhow::Result<bool> {
        let latest_version = get_latest_version().await?;
        log::info!(
            "check latest version, latest: {}, current: {}",
            latest_version.version,
            version
        );
        Ok(version == latest_version.version)
    }

    /// 检查是否启用升级
    /// 用户在系统设置点击更新后会生成一个.update文件，每次重新启动时检查是否存在该文件，
    /// 如果存在则检查版本号，当前版本号与最新版本号不同时，执行下载升级逻辑
    /// 该文件会在执行完升级后删除
    async fn check_enable_update() -> anyhow::Result<bool> {
        let exists = fs::exists(app_dir!(".update"))?;
        Ok(exists)
    }
}
pub trait PhaseRunner {
    async fn run(&self, callback: impl Fn(PhaseStatus)) -> anyhow::Result<()>;
}

impl PhaseRunner for Phase {
    async fn run(&self, callback: impl Fn(PhaseStatus)) -> anyhow::Result<()> {
        match self {
            Phase::CheckUpdate { current_version } => {
                // 检查用户是否允许升级
                let pass = Phase::check_enable_update().await?;
                if !pass {
                    log::info!("User did not choose to update");
                    callback(PhaseStatus::Skip("User did not choose to update"));
                    return Ok(());
                }
                // 检查当前版本是否是最新版本
                let pass = Phase::check_is_latest_version(current_version).await?;
                if pass {
                    log::info!("Current version is the latest version");
                    callback(PhaseStatus::Skip("Current version is the latest version"));
                    return Ok(());
                }

                log::info!("Downloading new app package...");
                // 下载更新
                let file_path = app_downloader::download(|downloaded, total| {
                    callback(PhaseStatus::Running {
                        downloaded,
                        total,
                        progress: downloaded as f64 / total as f64 * 100.0,
                    });
                })
                .await?;
                log::info!("App package downloaded");
                callback(PhaseStatus::Finished {
                    file_path: Some(file_path),
                });
                Ok(())
            }
            Phase::CheckResources { current_version } => {
                // 检查资源目录是否存在
                let pass = Phase::check_resource_dir_exists().await?;
                let pass = pass && Phase::check_resource_version(current_version).await?;
                if pass {
                    callback(PhaseStatus::Skip("Resource is latest"));
                    return Ok(());
                }
                let resource_version =
                    get_resource_version_with_app_version(current_version).await?;
                resources_downloader::download(&resource_version, |downloaded, total| {
                    callback(PhaseStatus::Running {
                        downloaded,
                        total,
                        progress: downloaded as f64 / total as f64 * 100.0,
                    });
                })
                .await?;
                callback(PhaseStatus::Finished { file_path: None });
                Ok(())
            }
        }
    }
}
