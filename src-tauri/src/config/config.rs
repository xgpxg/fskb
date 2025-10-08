use anyhow::anyhow;
use serde::Deserialize;
use std::cell::OnceCell;
use std::env;
use std::fs::File;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Config {
    /// 服务器配置
    #[serde(default = "ServerConfig::default")]
    pub server: ServerConfig,
    /// 数据库配置，如果未在yaml中配置，则默认从环境变量中获取
    #[serde(default = "DatabaseConfig::default")]
    #[allow(unused)]
    pub database: DatabaseConfig,
    /// 短信配置
    pub sms: Option<AliSmsConfig>,
    /// 邮箱配置
    pub mail: Option<MailConfig>,
    /// redis配置
    #[allow(unused)]
    pub redis: Option<RedisConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: Default::default(),
            database: Default::default(),
            sms: None,
            mail: None,
            redis: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    /// HTTP服务端口
    #[serde(default = "ServerConfig::default_address")]
    pub address: String,
    /// HTTP服务端口
    #[serde(default = "ServerConfig::default_port")]
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: ServerConfig::default_address(),
            port: ServerConfig::default_port(),
        }
    }
}

impl ServerConfig {
    fn default_port() -> u16 {
        9000
    }
    fn default_address() -> String {
        "127.0.0.1".to_string()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(unused)]
pub struct DatabaseConfig {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            url: env::var("OWL_DB_URL").unwrap_or_default(),
            username: None,
            password: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AliSmsConfig {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub sign_name: String,
    pub template_code: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MailConfig {
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(unused)]
pub struct RedisConfig {
    pub host: String,
    pub port: i32,
}

impl Config {
    pub fn from_file(file: &str) -> anyhow::Result<Self> {
        let file = File::open(file)?;
        let result: Config = serde_yaml::from_reader(file)?;
        Ok(result)
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub(crate) fn init(config_file: &str) -> anyhow::Result<()> {
    let result = Config::from_file(config_file);
    match result {
        Ok(config) => {
            CONFIG.get_or_init(|| {

                #[cfg(feature = "redis-cache")]
                if config.redis.is_none() {
                    log::error!("Redis未配置");
                    std::process::exit(1);
                }

                config
            });
            Ok(())
        }
        Err(e) => Err(anyhow!(format!(
            "can not load config file: {}: {}",
            config_file,
            e.to_string()
        ))),
    }
}

// pub(crate) fn init_default() -> anyhow::Result<()> {
//     CONFIG.get_or_init(|| Config::default());
//     Ok(())
// }

pub(crate) struct AppConfig;

impl<'a> AppConfig {
    pub fn get() -> &'a Config {
        CONFIG.get().unwrap()
    }

    pub fn server() -> &'a ServerConfig {
        &Self::get().server
    }

    #[allow(unused)]
    pub fn database() -> &'a DatabaseConfig {
        &Self::get().database
    }

    pub fn sms() -> Option<&'a AliSmsConfig> {
        Self::get().sms.as_ref()
    }

    pub fn mail() -> Option<&'a MailConfig> {
        Self::get().mail.as_ref()
    }

    #[cfg(feature = "redis-cache")]
    pub fn redis() -> Option<&'a RedisConfig> {
        Self::get().redis.as_ref()
    }
}
