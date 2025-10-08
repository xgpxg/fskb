use crate::common::res::Res;
use crate::utils::file_util;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use common::data_dir;
use std::path::Path;

pub(crate) fn image_path_to_base64(path: &str) -> anyhow::Result<String> {
    let image = std::fs::read(path)?;
    let base64 = BASE64_STANDARD.encode(image);
    Ok(base64)
}

type Result<T> = std::result::Result<Res<T>, String>;

pub(crate) async fn copy_chat_file_to_data_dir(kb_id: i64, path: String) -> Result<String> {
    let path = Path::new(&path);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    //let dest_dir = format!("data/chat/kb/{}", kb_id);
    let dest_dir = data_dir!("chat", "kb", kb_id.to_string())
        .to_string_lossy()
        .into_owned();
    let dest = file_util::generate_unique_file_path(&dest_dir, file_name);
    std::fs::copy(&path, &dest).map_err(|e| e.to_string())?;
    Ok(Res::success(dest))
}

pub(crate) async fn save_chat_file_to_data_dir(
    kb_id: i64,
    file_name: String,
    bytes: Vec<u8>,
) -> Result<String> {
    let file_name = Path::new(&file_name).file_name().unwrap().to_str().unwrap();
    let dest_dir = data_dir!("chat", "kb", kb_id.to_string())
        .to_string_lossy()
        .into_owned();
    let dest = file_util::generate_unique_file_path(&dest_dir, file_name);
    std::fs::write(&dest, bytes).map_err(|e| e.to_string())?;
    Ok(Res::success(dest))
}
