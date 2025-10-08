use anyhow::bail;
use serde::{Deserialize, Serialize};

const APP_VERSION_ENDPOINT: &str = "https://package-release.coderbox.cn/fs-kb-app/app-windows.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppVersion {
    // 应用版本号
    pub version: String,
    // 发布描述
    pub description: String,
    // 下载地址
    pub download_url: String,
    // 资源版本号
    pub resource_version: String,
    // 资源下载地址
    // 废弃：通过资源版本号获取
    #[deprecated]
    pub resource_download_url: Option<String>,
}

pub async fn get_latest_version() -> anyhow::Result<AppVersion> {
    let versions = reqwest::get(APP_VERSION_ENDPOINT)
        .await?
        .json::<Vec<AppVersion>>()
        .await?;
    if versions.is_empty() {
        bail!("No versions found".to_string());
    }
    Ok(versions[0].clone())
}
pub async fn get_all_versions() -> anyhow::Result<Vec<AppVersion>> {
    let versions = reqwest::get(APP_VERSION_ENDPOINT)
        .await?
        .json::<Vec<AppVersion>>()
        .await?;
    if versions.is_empty() {
        bail!("No versions found".to_string());
    }
    Ok(versions)
}
pub async fn get_version(version: &str) -> anyhow::Result<AppVersion> {
    let versions = reqwest::get(APP_VERSION_ENDPOINT)
        .await?
        .json::<Vec<AppVersion>>()
        .await?;
    if versions.is_empty() {
        bail!("No versions found".to_string());
    }
    Ok(versions
        .iter()
        .find(|v| v.version == version)
        .ok_or(anyhow::anyhow!("Version {} not found", version))?
        .clone())
}

pub async fn get_resource_version_with_app_version(version: &str) -> anyhow::Result<String> {
    get_version(version).await.map(|v| v.resource_version)
}
