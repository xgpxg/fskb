use futures_util::StreamExt;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

const FILES_URL: &str = "https://modelscope.cn/api/v1/models/<model_id>/repo/files?Recursive=true";
const DOWNLOAD_URL: &str = "https://modelscope.cn/models/<model_id>/resolve/master/<path>";

const UA: (&str, &str) = (
    "User-Agent",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36",
);
pub(crate) struct ModelScope;

#[derive(Debug, Deserialize)]
struct ModelScopeResponse {
    #[serde(rename = "Code")]
    code: i32,
    #[serde(rename = "Data")]
    data: ModelScopeResponseData,
}

#[derive(Debug, Deserialize)]
struct ModelScopeResponseData {
    #[serde(rename = "Files")]
    files: Vec<RepoFile>,
}
#[derive(Debug, Deserialize)]
struct RepoFile {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Path")]
    path: String,
    #[serde(rename = "Size")]
    size: u64,
}
impl ModelScope {
    pub async fn download(
        model_id: &str,
        save_path: &PathBuf,
        progress: impl Fn(f64),
    ) -> anyhow::Result<()> {
        let files_url = FILES_URL.replace("<model_id>", model_id);
        log::info!("downloading model form: {}", files_url);
        log::info!("save dir: {}", save_path.display());

        let resp = reqwest::get(files_url).await?;
        let model_scope_response = resp.json::<ModelScopeResponse>().await?;
        let data = model_scope_response.data;
        let repo_files = data.files;
        let total_size = repo_files
            .iter()
            .map(|repo_file| repo_file.size)
            .sum::<u64>();
        log::info!("total size: {}", total_size);

        fs::create_dir_all(save_path)?;

        let mut downloaded_size = 0;

        let client = reqwest::Client::builder().connect_timeout(std::time::Duration::from_secs(10));
        let client = client.build()?;

        for repo_file in repo_files.iter() {
            let path = &repo_file.path;
            let splits = path.split('/').collect::<Vec<_>>();
            let dir = if splits.len() > 1 {
                let dir = save_path.join(
                    splits
                        .iter()
                        .take(splits.len() - 1)
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join("/"),
                );
                fs::create_dir_all(&dir)?;
                dir
            } else {
                save_path.to_path_buf()
            };
            let name = &repo_file.name;
            let mut file = BufWriter::new(File::create(&dir.join(name))?);

            let url = DOWNLOAD_URL
                .replace("<model_id>", model_id)
                .replace("<path>", &path);
            log::info!("downloading {} from {}", name, url);

            let response = client.get(url).header(UA.0, UA.1).send().await?;
            let mut stream = response.bytes_stream();
            while let Some(item) = stream.next().await {
                let chunk = item?;
                file.write_all(&chunk)?;

                downloaded_size += chunk.len() as u64;
                progress(downloaded_size as f64 / total_size as f64 * 100.0);
            }
        }

        Ok(())
    }
}

#[tokio::test]
async fn test_download() -> anyhow::Result<()> {
    ModelScope::download(
        "Qwen/Qwen3-0.6B",
        &PathBuf::from("data/model/Qwen/Qwen3-0.6B"),
        |progress| {
            println!("progress: {}", progress);
        },
    )
    .await?;
    Ok(())
}
