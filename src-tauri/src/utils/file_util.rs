use crate::common::pool::HTTP_CLIENT;
use crate::common::res::Res;
use crate::utils::file_util;
use anyhow::bail;
use common::{app_dir, data_dir, file_dir};
use nanoid::nanoid;
use std::path::Path;

/// 生成保存文件名
///
/// 文件名保存格式：YYYYMMDD-10位随机字符串_原始文件名。
pub fn make_save_file(file_name: &str) -> anyhow::Result<(String, String)> {
    let today = fastdate::DateTime::now().format("YYYYMM");
    //let dir = format!("data/file/{}", today);
    let dir = file_dir!(today).to_string_lossy().into_owned();
    match std::fs::exists(&dir) {
        Ok(exists) => {
            if !exists {
                std::fs::create_dir_all(&dir)?;
            }
        }
        Err(e) => {
            bail!("目录创建失败，原因：{}", e);
        }
    }

    // 保存的文件名
    let mut save_file = generate_unique_file_path(&dir, file_name);
    let save_file_name = Path::new(&save_file)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    Ok((save_file_name, save_file))
}

pub(crate) fn generate_unique_file_path(dir: &str, file_name: &str) -> String {
    let path = Path::new(dir);
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
    let dest_base = path.join(file_name);
    if !dest_base.exists() {
        return dest_base.to_string_lossy().into_owned();
    }

    let file_name = dest_base
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let ext = dest_base
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    let mut i = 1;
    loop {
        let dest = path.join(format!("{}_{}.{}", file_name, i, ext));
        if !Path::new(&dest).exists() {
            break dest.to_string_lossy().into_owned();
        }
        i += 1;
    }
}

pub fn make_download_file(file_name: &str) -> String {
    format!("/file/download/{}", file_name)
}
pub fn make_kb_ref_file(
    table_name: &str,
    kb_import_record_id: i64,
    file_name: &str,
) -> anyhow::Result<(String, String, String)> {
    let uni_dir = &make_kb_ref_uni_dir(kb_import_record_id);

    //let dir = format!("data/database/{}.lance/refs/{}", table_name, uni_dir);
    let dir = data_dir!("database", format!("{}.lance", table_name), "refs", uni_dir)
        .to_string_lossy()
        .into_owned();

    match std::fs::exists(&dir) {
        Ok(exists) => {
            if !exists {
                std::fs::create_dir_all(&dir)?;
            }
        }
        Err(e) => {
            bail!("目录创建失败，原因：{}", e);
        }
    }
    let save_file_name = format!("{}-{}_{}", table_name, uni_dir, file_name);
    let save_file = format!("{}/{}", dir, save_file_name);
    let download_file = format!("/file/kb/ref/{}", save_file_name);
    Ok((save_file_name, save_file, download_file))
}
fn make_kb_ref_uni_dir(kb_import_record_id: i64) -> String {
    let uni_dir = kb_import_record_id.to_string();
    // uni_dir 固定为10位，不足的前补零，超过保留后10位。
    let uni_dir = if uni_dir.len() > 10 {
        uni_dir[uni_dir.len() - 10..].to_string()
    } else {
        format!("{:0>10}", uni_dir)
    };
    uni_dir
}

#[allow(unused)]
pub fn remove_kb_ref_dir(table_name: &str, kb_import_record_id: i64) -> anyhow::Result<()> {
    let uni_dir = make_kb_ref_uni_dir(kb_import_record_id);
    //let dir = format!("data/database/{}.lance/refs/{}", table_name, uni_dir);
    let dir = data_dir!("database", format!("{}.lance", table_name), "refs", uni_dir)
        .to_string_lossy()
        .into_owned();
    std::fs::remove_dir_all(dir)?;

    Ok(())
}

#[allow(unused)]
pub async fn upload(path: &str) -> anyhow::Result<String> {
    let base_url = "https://oneapi.coderbox.cn/openapi/upload/addTaskFile";
    let form = reqwest::multipart::Form::new().file("file", path).await?;
    let response = HTTP_CLIENT.post(base_url).multipart(form).send().await?;
    let res = response.json::<Res<String>>().await?;

    Ok(res.data.unwrap_or_default())
}
