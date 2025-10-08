use crate::config::config;
use clap::Parser;
use std::process::exit;

/// 命令行参数
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliConfig {
    /// 配置文件
    #[arg(short, long, default_value = "config.yaml")]
    pub config: Option<String>,
}

impl CliConfig {
    /// 加载配置文件
    pub fn load(&self) -> anyhow::Result<()> {
        if let Some(file) = &self.config {
            log::info!("use config file: {}", file);
            if let Err(e) = config::init(file) {
                log::error!("load config error: {}", e);
                exit(1)
            }
            Ok(())
        } else {
            log::error!("config file not set");
            exit(1);
        }
    }
}
