mod modelscope;

use crate::modelscope::ModelScope;
use bytes::Buf;
use common::{data_dir, temp_dir};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::net::TcpListener;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};

const MODELS_ENDPOINT: &str = "https://package-release.coderbox.cn/fs-kb-app/model-list-<os>.json";
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub struct ModelManager;

static PIDS: LazyLock<Arc<Mutex<HashMap<String, u32>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDefine {
    pub name: String,
    pub summary: String,
    pub description: String,
    pub task_type: i8,
    pub deploy_file: String,
    pub model_file: String,
    pub command: String,
}
impl ModelManager {
    pub async fn list() -> anyhow::Result<Vec<ModelDefine>> {
        log::info!("fetching model list");
        let os = env::consts::OS;
        let endpoint = MODELS_ENDPOINT.replace("<os>", os);
        let resp = reqwest::get(endpoint)
            .await?
            .json::<Vec<ModelDefine>>()
            .await?;
        log::info!("fetched {} models", resp.len());
        Ok(resp)
    }
    pub async fn get(name: &str) -> anyhow::Result<ModelDefine> {
        let model = Self::list()
            .await?
            .into_iter()
            .find(|m| m.name == name)
            .ok_or(anyhow::anyhow!("model not found"))?;
        Ok(model)
    }
    pub async fn install(name: &str) -> anyhow::Result<()> {
        let model = Self::get(name).await?;

        // 临时下载目录
        let download_temp_dir = temp_dir!(name);
        std::fs::create_dir_all(&download_temp_dir)?;

        // 模型部署程序临时文件
        let temp_deploy_file_path = download_temp_dir.join("deploy.tar.gz");

        // 模型相关保存路径
        let path = data_dir!("model", name);
        // 模型文件路径
        let model_path = path.join("model");

        std::fs::create_dir_all(&path)?;
        std::fs::create_dir_all(&model_path)?;

        // 下载模型部署程序
        log::info!(
            "downloading deploy file to {}",
            temp_deploy_file_path.display()
        );
        let mut response = reqwest::get(model.deploy_file).await?;
        let mut temp_deploy_file = std::fs::File::create(&temp_deploy_file_path)?;
        while let Some(chunk) = response.chunk().await? {
            std::io::Write::write_all(&mut temp_deploy_file, &chunk)?;
        }

        // 解压模型部署程序
        unzip(&temp_deploy_file_path, &path);
        // 清理临时文件
        std::fs::remove_file(&temp_deploy_file_path)?;

        // 下载模型文件
        match &model.model_file {
            // modelscope下载
            model_file if model_file.starts_with("modelscope://") => {
                let model_id = model_file.replace("modelscope://", "");
                log::info!("downloading model from modelscope, model id: {}", model_id);
                ModelScope::download(&model_id, &model_path, |progress| {}).await?;
            }
            // http下载
            model_file => {
                let temp_model_file_path = download_temp_dir.join(format!("{}.tar.gz", name));
                log::info!(
                    "downloading model file to {}",
                    temp_model_file_path.display()
                );
                let mut response = reqwest::get(model_file).await?;
                let mut temp_model_file = std::fs::File::create(&temp_model_file_path)?;
                while let Some(chunk) = response.chunk().await? {
                    std::io::Write::write_all(&mut temp_model_file, &chunk)?;
                }
                unzip(&temp_model_file_path, &model_path);
                std::fs::remove_file(&temp_model_file_path)?;
            }
        }

        log::info!("model {} install success", name);

        Ok(())
    }

    pub async fn uninstall(name: &str) -> anyhow::Result<()> {
        let path = data_dir!("model", name);
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        Ok(())
    }

    pub async fn start(name: &str) -> anyhow::Result<String> {
        println!("starting model {}", name);
        let exe = data_dir!("model", name, "deploy.exe");
        let model_path = data_dir!("model", name, "model");
        let port = find_free_port();
        let process = std::process::Command::new(exe)
            .arg("--port")
            .arg(port.to_string())
            .args(&["--model-path", model_path.to_str().unwrap()])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .expect("❌ failed to execute process");
        {
            PIDS.lock()
                .map_err(|e| anyhow::anyhow!("Failed to lock PIDS: {:?}", e))?
                .insert(name.to_string(), process.id());
        }
        Ok(format!("http://localhost:{}", port))
    }
    pub async fn stop(name: &str) -> anyhow::Result<()> {
        let dir = data_dir!("model", name);
        let pid_file = dir.join(".pid");
        if !pid_file.exists() {
            return Ok(());
        }
        let pid = std::fs::read_to_string(&pid_file)?;
        #[cfg(windows)]
        {
            std::process::Command::new("taskkill")
                .args(&["/T", "/F", "/PID"])
                .arg(pid.to_string())
                .creation_flags(CREATE_NO_WINDOW)
                .output()?;
            std::fs::remove_file(&pid_file)?;
        }
        #[cfg(unix)]
        {
            // Unix/Linux/macOS 系统
            Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()?;
        }

        {
            PIDS.lock()
                .map_err(|e| anyhow::anyhow!("Failed to lock PIDS: {:?}", e))?
                .remove(name);
        }

        Ok(())
    }

    pub async fn shutdown() {
        let names: Vec<String> = {
            let pids = PIDS.lock().unwrap();
            pids.keys().cloned().collect()
        };
        for name in names {
            if let Err(e) = Self::stop(&name).await {
                log::error!("Failed to stop model {}: {}", name, e);
            }
        }
    }
}

fn find_free_port() -> u16 {
    loop {
        let port = rand::random_range(10000..65535);
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return port;
        }
    }
}

fn unzip(src: &PathBuf, dest_dir: &PathBuf) {
    std::process::Command::new("tar")
        .args(&[
            "-xzf",
            src.to_str().unwrap(),
            "-C",
            dest_dir.to_str().unwrap(),
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .expect("❌ failed to execute process");
}

mod tests {
    #[tokio::test]
    async fn test_model_list() {
        let list = super::ModelManager::list().await.unwrap();
        println!("{:?}", list);
    }

    #[tokio::test]
    async fn test_model_get() {
        let model = super::ModelManager::get("qwen3-0.6b").await.unwrap();
        println!("{:?}", model);
    }

    #[tokio::test]
    async fn test_model_install() {
        super::ModelManager::install("qwen3-0.6b").await.unwrap();
    }

    #[tokio::test]
    async fn test_model_uninstall() {
        super::ModelManager::uninstall("qwen3-0.6b").await.unwrap();
    }

    #[tokio::test]
    async fn test_model_start() {
        let api = super::ModelManager::start("qwen3-0.6b").await.unwrap();
        println!("{:?}", api);
    }

    #[tokio::test]
    async fn test_model_stop() {
        super::ModelManager::stop("qwen3-0.6b").await.unwrap();
    }
}
