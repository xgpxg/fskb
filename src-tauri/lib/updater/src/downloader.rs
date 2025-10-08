use futures_util::StreamExt;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, LazyLock, Mutex};
use tokio::sync::oneshot;

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
pub(crate) async fn download(
    url: &str,
    dest: &str,
    progress_handler: impl Fn(u64, u64),
) -> anyhow::Result<()> {
    let id = url.to_string();
    let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();

    {
        let mut manager = DOWNLOAD_MANAGER.lock().unwrap();
        manager.add(id.clone(), cancel_tx);
    }

    let client = reqwest::Client::new();
    let res = client.get(url.clone()).send().await?;

    // 文件总大小
    let total_size = res.content_length().unwrap_or(0);

    // 创建目录
    fs::create_dir_all(Path::new(&dest).parent().unwrap())?;

    // 创建文件
    let mut file = BufWriter::new(File::create(&dest)?);

    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = {
        tokio::select! {
            item = stream.next() => item,
            _ = &mut cancel_rx => {
                DOWNLOAD_MANAGER.lock().unwrap().remove(&id);
                fs::remove_file(&dest).ok();
                return Ok(());
            }
        }
    } {
        let chunk = item?;

        file.write_all(&chunk)?;

        downloaded += chunk.len() as u64;

        // 计算进度
        let progress = (downloaded as f64 / total_size as f64) * 100.0;

        progress_handler(downloaded, total_size);
    }

    file.flush()?;

    Ok(())
}
