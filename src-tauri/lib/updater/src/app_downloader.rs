use crate::{app_version, downloader};
use common::dir::AppDir;
use std::fs;

pub(crate) async fn download(progress_handler: impl Fn(u64, u64)) -> anyhow::Result<String> {
    let latest_version = app_version::get_latest_version().await?;
    log::info!("latest version: {}", latest_version.version);
    let app_download_url = latest_version.download_url;
    let app_package_name = app_download_url.split('/').last().unwrap();

    log::info!("downloading {}", app_package_name);
    // 下载路径
    let download_file = AppDir::app_root_dir()
        .join(app_package_name)
        .to_string_lossy()
        .to_string();

    downloader::download(
        &app_download_url,
        &download_file.clone(),
        |downloaded, total| {
            progress_handler(downloaded, total);
        },
    )
    .await
    .map_err(|e| anyhow::anyhow!(e))?;

    log::info!("downloaded {}", app_package_name);

    Ok(download_file)
}

async fn check_ipv6_support() -> bool {
    match reqwest::get("https://v6-oneapi.coderbox.cn/openapi/public/myip").await {
        Ok(_) => true,
        Err(_) => false,
    }
}
