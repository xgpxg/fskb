pub mod mcp_manager;
pub mod mcpsi;

use anyhow::{anyhow, bail};
use common::{app_dir, resources_dir};
use dashmap::DashMap;
use rmcp::service::RunningService;
use rmcp::transport::{ConfigureCommandExt, SseClientTransport, TokioChildProcess};
use rmcp::{RoleClient, ServiceExt};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::collections::HashMap;
use std::env;
use tokio::process::Command;

pub use rmcp::model::JsonObject;
pub use rmcp::model::Tool;

type Result<T> = anyhow::Result<T>;

const CREATE_NO_WINDOW: u32 = 0x08000000;

/// MCP客户端管理器
pub struct McpClientManage {
    /// 服务列表
    pub services: DashMap<String, RunningService<RoleClient, ()>>,
}

impl McpClientManage {
    pub fn new() -> Self {
        Self {
            services: DashMap::new(),
        }
    }

    /// 运行服务
    /// - config: 配置文件，json格式
    pub async fn run(&self, config: &str) -> anyhow::Result<()> {
        let config = parse_config(config)?;
        for (name, config) in config.mcp_servers.into_iter() {
            if let Some(url) = config.url {
                let transport = SseClientTransport::start(url).await?;
                let service = ().serve(transport).await?;
                self.services.insert(name.to_string(), service);
                return Ok(());
            }
            // 检查命令是否支持
            config.check_command_support()?;
            if let Some(command) = config.command {
                // windows下特殊处理
                // 由于command路径配置为相对路径，所以这里需要将command转换为绝对路径
                #[cfg(windows)]
                let mut command = {
                    if command.ends_with(".exe") {
                        format!("{}/{}", app_dir!().to_string_lossy().into_owned(), command)
                    } else {
                        command
                    }
                };
                match command.as_str() {
                    "uvx" => {}
                    "npx" => {
                        #[cfg(windows)]
                        {
                            command = format!("{}.cmd", command);
                        }
                    }
                    "sh" => {}
                    _ => {
                        //return Err(anyhow!("Invalid command"));
                    }
                }
                let service = ()
                    .serve(TokioChildProcess::new(Command::new(command).configure(
                        |cmd| {
                            cmd.args(config.args.unwrap_or_default());
                            cmd.envs(config.env.unwrap_or_default());
                            cmd.creation_flags(CREATE_NO_WINDOW);
                        },
                    ))?)
                    .await?;
                self.services.insert(name.to_string(), service);
                return Ok(());
            }
        }
        bail!("Invalid config");
    }

    /// 停止服务
    /// - name: 服务名
    pub async fn stop(&self, name: &str) -> anyhow::Result<()> {
        if let Some((_, service)) = self.services.remove(name) {
            service.cancel().await?;
        }
        Ok(())
    }

    /// 列出所有服务
    pub async fn list_all_services(&self) -> anyhow::Result<Vec<String>> {
        Ok(self.services.iter().map(|s| s.key().clone()).collect())
    }

    /// 列出服务所有工具
    /// - name: 服务名
    pub async fn list_all_tools(&self, name: &str) -> anyhow::Result<Vec<rmcp::model::Tool>> {
        if let Some(entry) = self.services.get(name) {
            let result = entry
                .value()
                .list_all_tools()
                .await
                .map_err(|e| anyhow!("工具列表获取失败：{}", e))?;
            return Ok(result);
        }
        bail!("Service {} not found or not running", name);
    }

    /// 调用工具
    /// - name: 服务名
    /// - request: 工具请求参数
    pub async fn call_tool(
        &self,
        name: &str,
        request: rmcp::model::CallToolRequestParam,
    ) -> anyhow::Result<rmcp::model::CallToolResult> {
        if let Some(entry) = self.services.get(name) {
            let result = entry.value().call_tool(request).await?;
            return Ok(result);
        }
        bail!("Service {} not found or not running", name);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServersConfig {
    #[serde(rename = "mcpServers")]
    pub mcp_servers: HashMap<String, McpServerConfig>,
}

impl McpServersConfig {
    pub fn get_server_name(&self) -> String {
        self.mcp_servers
            .keys()
            .next()
            .map(|x| x.to_string())
            .unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub url: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
}

pub fn parse_config(config: &str) -> anyhow::Result<McpServersConfig> {
    let config: McpServersConfig = serde_json::from_str(config)?;
    if config.mcp_servers.len() > 1 {
        bail!("暂不支持多个Server配置节点");
    }
    Ok(config)
}

impl McpServerConfig {
    // 检查启动命令是否支持
    pub fn check_command_support(&self) -> anyhow::Result<()> {
        let command = self
            .command
            .as_ref()
            .ok_or(anyhow!("MCP Server启动命令为空，无法执行"))?
            .as_ref();
        match command {
            "sh" | "bash" => Ok(()),
            "uvx" => {
                // 检查uvx是否安装
                if get_program_version("uvx").is_none() {
                    bail!("uvx未安装")
                } else {
                    Ok(())
                }
            }
            "npx" => {
                // 检查npx是否安装
                if get_program_version(if cfg!(windows) { "npx.cmd" } else { "npx" }).is_none() {
                    bail!("npx未安装")
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

fn get_program_version(program: &str) -> Option<String> {
    let output = std::process::Command::new(program)
        .arg("--version")
        .output();
    match output {
        Ok(output) => Some(String::from_utf8_lossy(&output.stdout).trim().to_string()),
        Err(e) => {
            log::error!("{}", e);
            None
        }
    }
}

pub fn init() {
    unsafe {
        let nodejs = resources_dir!("driver", "nodejs");
        let python = resources_dir!("driver", "libreoffice", "program", "python-core-3.10.18");
        let uv = resources_dir!("driver", "uv");
        if let Some(path) = env::var_os("PATH") {
            let mut paths = env::split_paths(&path).collect::<Vec<_>>();
            paths.push(nodejs);
            paths.push(python);
            paths.push(uv);
            let new_path = env::join_paths(paths).unwrap();
            env::set_var("PATH", new_path);
        } else {
            env::set_var("PATH", nodejs);
        }
    }
}
#[tokio::test]
async fn test_mcp_client_manage() -> anyhow::Result<()> {
    env_logger::Builder::new().init();
    let manage = McpClientManage::new();

    manage
        .run(
            r#"
  {
"mcpServers": {
        "image-qa-online": {
          "command": "../../data/mcp/image-qa-online/image-qa-online.exe",
          "args": []
        }
      }
}

    "#,
        )
        .await?;

    let tools = manage.list_all_tools("image-qa-online").await?;
    println!("tools: {:?}", tools);

    let mut params = Map::new();
    params.insert("prompt".into(), "图片里有什么？".into());
    params.insert("image_paths".into(), ["../../data/temp/1.png"].into());

    manage
        .call_tool(
            "image-qa-online",
            rmcp::model::CallToolRequestParam {
                name: "chat_to_image".into(),
                arguments: Some(params),
            },
        )
        .await?;

    Ok(())
}
