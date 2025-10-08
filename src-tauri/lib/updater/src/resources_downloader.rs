use crate::downloader;
use common::dir::AppDir;
use std::fs;

pub(crate) async fn download(
    version: &str,
    progress_handler: impl Fn(u64, u64),
) -> anyhow::Result<()> {
    // 资源包名称
    let package_name = "resources.tar.gz";
    log::info!("downloading {}", package_name);

    // 下载路径
    let download_file = AppDir::temp_dir()
        .join(package_name)
        .to_string_lossy()
        .to_string();
    // 下载压缩包
    // 检查是否支持IPV6
    let url = if check_ipv6_support().await {
        log::info!("IPV6 is supported");
        format!(
            "http://v6-package-release.coderbox.cn:5080/fs-kb-app/app/resource/{}/{}",
            version, package_name
        )
    } else {
        log::info!("IPV6 is not supported, use IPV4");
        format!(
            "https://package-release.coderbox.cn/fs-kb-app/app/resource/{}/{}",
            version, package_name
        )
    };

    downloader::download(&url, &download_file.clone(), |downloaded, total| {
        progress_handler(downloaded, total);
    })
    .await
    .map_err(|e| anyhow::anyhow!(e))?;

    log::info!("extracting {}", package_name);

    // 解压路径
    let extract_path = &AppDir::resources_dir().to_string_lossy().to_string();
    // 创建解压目录
    fs::create_dir_all(extract_path)?;

    // 使用tar命令解压tar.gz
    let output = {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            std::process::Command::new("tar")
                .args(&["-xzf", &download_file, "-C", extract_path])
                .creation_flags(CREATE_NO_WINDOW)
                .output()
        }

        #[cfg(not(target_os = "windows"))]
        {
            std::process::Command::new("tar")
                .args(&["-xzf", &download_file, "-C", extract_path])
                .output()
        }
    }
    .expect("❌ failed to execute process");

    // 检查解压是否成功
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        log::error!("Unzip {} fail: {}", package_name, error_msg);
        return Err(anyhow::anyhow!("Unzip fail: {}", error_msg));
    }

    log::info!("Unzip {} success", package_name);

    let _ = fs::remove_file(&download_file);

    Ok(())
}

async fn check_ipv6_support() -> bool {
    match reqwest::get("https://v6-oneapi.coderbox.cn/openapi/public/myip").await {
        Ok(_) => true,
        Err(_) => false,
    }
}
