use crate::common::res::Res;
use crate::utils::file_util;
use common::{data_dir, temp_dir};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, LazyLock, Mutex};
use tauri::{AppHandle, Emitter};
use tokio::sync::oneshot;

type Result<T> = std::result::Result<Res<T>, String>;
#[tauri::command]
pub(crate) async fn save_temp_file(file_name: String, bytes: Vec<u8>) -> Result<String> {
    let file_name = Path::new(&file_name).file_name().unwrap().to_str().unwrap();
    //let path = format!("{}/{}", "data/temp", file_name);
    let path = temp_dir!(file_name).to_string_lossy().into_owned();
    fs::write(&path, bytes).map_err(|e| e.to_string())?;
    Ok(Res::success(path))
}

#[tauri::command]
pub(crate) async fn copy_file_to_file_dir(src: String) -> Result<String> {
    let file_name = Path::new(&src).file_name().unwrap().to_str().unwrap();
    let (_, save_file_path) = file_util::make_save_file(file_name).map_err(|e| e.to_string())?;
    fs::copy(&src, &save_file_path).map_err(|e| e.to_string())?;
    Ok(Res::success(save_file_path))
}

#[tauri::command]
pub(crate) async fn save_file_to_file_dir(file_name: String, bytes: Vec<u8>) -> Result<String> {
    let file_name = Path::new(&file_name).file_name().unwrap().to_str().unwrap();
    let (_, save_file_path) = file_util::make_save_file(file_name).map_err(|e| e.to_string())?;
    fs::write(&save_file_path, bytes).map_err(|e| e.to_string())?;
    Ok(Res::success(save_file_path))
}

#[tauri::command]
pub(crate) async fn save_note_file(
    note_id: i64,
    file_name: String,
    bytes: Vec<u8>,
) -> Result<String> {
    let file_name = Path::new(&file_name).file_name().unwrap().to_str().unwrap();
    let dir = data_dir!("note", &note_id.to_string());
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    let file_name = file_util::generate_unique_file_path(&dir.to_string_lossy(), file_name);
    let path = dir.join(file_name).to_string_lossy().into_owned();
    fs::write(&path, bytes).map_err(|e| e.to_string())?;
    Ok(Res::success(path))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DownloadEvent {
    // 唯一标识
    id: String,
    // 进度
    progress: f64,
    // 已下载字节数
    downloaded: u64,
    // 总字节数
    total_size: u64,
}

// 全局下载任务管理器
struct DownloadManager {
    cancellers: HashMap<String, oneshot::Sender<()>>,
}

impl DownloadManager {
    fn new() -> Self {
        Self {
            cancellers: HashMap::new(),
        }
    }

    fn add(&mut self, id: String, cancel_tx: oneshot::Sender<()>) {
        self.cancellers.insert(id, cancel_tx);
    }

    fn remove(&mut self, id: &str) -> Option<oneshot::Sender<()>> {
        self.cancellers.remove(id)
    }
}

static DOWNLOAD_MANAGER: LazyLock<Arc<Mutex<DownloadManager>>> =
    LazyLock::new(|| Arc::new(Mutex::new(DownloadManager::new())));
#[tauri::command]
pub(crate) async fn download(app: AppHandle, id: String, url: String, dest: String) -> Result<()> {
    let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();

    {
        let mut manager = DOWNLOAD_MANAGER.lock().unwrap();
        manager.add(id.clone(), cancel_tx);
    }

    let client = reqwest::Client::new();
    let res = client
        .get(url.clone())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // 文件总大小
    let total_size = res.content_length().unwrap_or(0);

    // 创建目录
    fs::create_dir_all(Path::new(&dest).parent().unwrap()).map_err(|e| e.to_string())?;

    // 创建文件
    let mut file = BufWriter::new(File::create(&dest).map_err(|e| e.to_string())?);

    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = {
        tokio::select! {
            item = stream.next() => item,
            _ = &mut cancel_rx => {
                DOWNLOAD_MANAGER.lock().unwrap().remove(&id);
                fs::remove_file(&dest).ok();
                return Ok(Res::success(()));
            }
        }
    } {
        let chunk = item.map_err(|e| e.to_string())?;

        file.write_all(&chunk).map_err(|e| e.to_string())?;

        downloaded += chunk.len() as u64;

        // 计算进度
        let progress = (downloaded as f64 / total_size as f64) * 100.0;

        // 发送进度事件到前端
        app.emit(
            "download-progress",
            DownloadEvent {
                id: id.clone(),
                progress,
                downloaded,
                total_size,
            },
        )
        .map_err(|e| e.to_string())?;
    }

    file.flush().map_err(|e| e.to_string())?;

    Ok(Res::success(()))
}

#[tauri::command]
pub(crate) async fn cancel_download(id: String) -> Result<()> {
    let mut manager = DOWNLOAD_MANAGER.lock().unwrap();
    if let Some(cancel_tx) = manager.remove(&id) {
        let _ = cancel_tx.send(());
    }
    Ok(Res::success(()))
}
