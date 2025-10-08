use crate::McpServersConfig;
use serde::{Deserialize, Serialize};
use std::env;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::time::Duration;
use tokio::fs;

pub static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(3))
        .read_timeout(Duration::from_secs(60 * 60 * 24))
        .pool_max_idle_per_host(5)
        .build()
        .unwrap()
});

/// MCP Server列表端点
const MCP_SERVER_ENDPOINT: &str = "https://package-release.coderbox.cn/fs-kb-app/servers-<os>.json";

const CREATE_NO_WINDOW: u32 = 0x08000000;

/// 自研的MCP Server管理器
pub struct Mcpsi;

impl Mcpsi {
    /// 获取MCP Server列表
    pub async fn list_mcp_servers() -> anyhow::Result<Vec<ServerEndpoint>> {
        log::info!("fetching mcp servers");
        let os = env::consts::OS;
        let resp = HTTP_CLIENT
            .get(MCP_SERVER_ENDPOINT.replace("<os>", os))
            .send()
            .await?;
        let servers = resp.json::<Vec<ServerEndpoint>>().await?;
        log::info!("fetched {} mcp servers", servers.len());
        Ok(servers)
    }

    /// 获取MCP Server
    pub async fn get_mcp_server(name: &str) -> anyhow::Result<ServerEndpoint> {
        let servers = Self::list_mcp_servers().await?;
        Ok(servers
            .iter()
            .find(|s| s.name == name)
            .ok_or(anyhow::anyhow!("server not found"))?
            .clone())
    }

    /// 安装MCP Server
    pub async fn install_mcp_server(name: &str, path: PathBuf) -> anyhow::Result<()> {
        let servers = Self::list_mcp_servers().await?;
        servers
            .iter()
            .find(|s| s.name == name)
            .ok_or(anyhow::anyhow!("server not found"))?
            .install(path)
            .await?;
        Ok(())
    }

    pub async fn uninstall_mcp_server(name: &str, path: PathBuf) -> anyhow::Result<()> {
        let servers = Self::list_mcp_servers().await?;
        servers
            .iter()
            .find(|s| s.name == name)
            .ok_or(anyhow::anyhow!("server not found"))?
            .uninstall(path)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerEndpoint {
    pub name: String,
    pub summary: String,
    pub description: String,
    pub config: McpServersConfig,
    pub url: String,
    /// 类型：stdio | sse
    pub r#type: String,
    pub configurable: Option<i8>,
    pub version: Option<String>,
}

impl ServerEndpoint {
    pub async fn install(&self, path: PathBuf) -> anyhow::Result<()> {
        log::info!("🔧  installing {}", &self.name);
        let servers = Mcpsi::list_mcp_servers().await?;
        for server in servers {
            if &server.name == &self.name {
                match server.r#type.as_str() {
                    "stdio" => {
                        log::info!("downloading from {}", &server.url);
                        let download_url = &server.url;
                        // 从url提取文件名
                        let file_name = download_url
                            .split('/')
                            .last()
                            .unwrap()
                            .split('?')
                            .next()
                            .unwrap();
                        let res = HTTP_CLIENT.get(download_url).send().await?;
                        let bytes = res.bytes().await?;
                        let dir = path.join(&self.name);
                        if !dir.exists() {
                            fs::create_dir_all(&dir).await?;
                        }
                        let file_path = dir.join(file_name);

                        fs::write(&file_path, bytes).await?;

                        log::info!("extracting {}", &file_path.to_str().unwrap());
                        // 使用tar命令解压tar.gz
                        std::process::Command::new("tar")
                            .args(&[
                                "-xzf",
                                file_path.to_str().unwrap(),
                                "-C",
                                dir.to_str().unwrap(),
                            ])
                            .creation_flags(CREATE_NO_WINDOW)
                            .output()
                            .expect("❌ failed to execute process");

                        log::info!("✅  {} installed", &self.name);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub async fn uninstall(&self, path: PathBuf) -> anyhow::Result<()> {
        log::info!("🧹  uninstalling {}", &self.name);
        let dir = path.join(&self.name);
        if dir.exists() {
            fs::remove_dir_all(&dir).await?;
        }
        log::info!("🗑️  {} uninstalled", &self.name);
        Ok(())
    }
}

#[tokio::test]
async fn test_server_list() {
    let list = Mcpsi::list_mcp_servers().await.unwrap();
    println!("{:?}", list);
}
