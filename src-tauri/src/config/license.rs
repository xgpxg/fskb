use crate::config;
use crate::config::config::AppConfig;
use anyhow::bail;
use std::process::exit;

// 从环境变量中获取授权签名的KEY
#[cfg(not(debug_assertions))]
const LICENSE_SIGN_KEY: &str = env!("LICENSE_SIGN_KEY", "License sign key not found");

/// 检查授权文件
pub fn check_license(license_file: Option<String>) -> anyhow::Result<()> {
    #[cfg(not(debug_assertions))]
    {
        let license_file = if let Some(license_file) = license_file {
            license_file
        } else {
            bail!("License file not found");
        };
        let license = if let Ok(license) =
            license_gen::license::LicenseV1::read_from_file(license_file.as_str())
        {
            license
        } else {
            bail!("License file is invalid");
        };
        if let Err(e) = license.validate(crate::config::license::LICENSE_SIGN_KEY) {
            bail!("{}", e);
        }

        // 过期时间
        let expiration_time = chrono::NaiveDate::parse_from_str(&license.expiration, "%Y-%m-%d")?;
        // 当前时间
        let now = chrono::Local::now().date_naive();
        // 剩余天数
        let remaining = expiration_time.signed_duration_since(now).num_days();
        if remaining < 3 {
            log::warn!(
                "⏰  License will expire in {} days. Expiration time: {}",
                remaining,
                &license.expiration
            );
        }
    }

    Ok(())
}

pub fn get_license_simple_info(license_file: Option<String>) -> anyhow::Result<String> {
    let license_file = if let Some(license_file) = license_file {
        license_file
    } else {
        bail!("未找到License");
    };
    if let Ok(license) = license_gen::license::LicenseV1::read_from_file(license_file.as_str()) {
        return Ok(format!(
            "授权给：{}，有效期至：{}",
            license.license_to, license.expiration,
        ));
    }

    bail!("无效的License");
}

pub async fn task_check_license() {
    let license_file = &AppConfig::get().license;
    if let Err(e) = config::license::check_license(license_file.clone()) {
        log::error!("{}", e);
        exit(1)
    }
}
