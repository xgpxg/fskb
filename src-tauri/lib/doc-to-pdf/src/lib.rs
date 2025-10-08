use anyhow::{Error, bail};
use common::app_dir;
use log::log;
use std::cell::LazyCell;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;
use std::{env, fs, thread};

pub type Result<T> = anyhow::Result<T>;

/// 初始化 libreoffice 环境
pub fn init() -> Result<()> {
    // 修改 libreoffice 默认路径
    let current_dir = app_dir!();
    let current_dir = current_dir.to_str().unwrap().replace("\\", "/");

    #[cfg(test)]
    let libreoffice_root_dir = format!("{}/../../resources/driver/libreoffice", current_dir);

    #[cfg(not(test))]
    let libreoffice_root_dir = format!("{}/resources/driver/libreoffice", current_dir);

    unsafe {
        env::set_var("LIBREOFFICE_ROOT_DIR", &libreoffice_root_dir);
    }

    Ok(())
}

#[cfg(target_os = "linux")]
/// 将文件转换为 pdf
pub fn convert(src: &str, dest: &str) -> Result<()> {
    // 文件后缀
    let ext = Path::new(src).extension();
    if ext.is_none() {
        return bail!("无效的文件名");
    }
    let ext = ext.unwrap().to_str().unwrap();
    if ext != "docx"
        && ext != "doc"
        && ext != "xlsx"
        && ext != "xls"
        && ext != "pptx"
        && ext != "ppt"
        && ext != "csv"
    {
        return bail!("不支持的文件格式: {}", ext);
    }

    // 由于libreoffice路径不支持中文，所以先复制一个临时文件
    let temp = format!("data/temp/{}.{}", uuid::Uuid::new_v4().to_string(), ext);
    fs::copy(src, &temp)?;

    let libreoffice_root_dir = env::var("LIBREOFFICE_ROOT_DIR")?;
    let program_dir = format!("{}/program", libreoffice_root_dir);

    {
        use libreoffice_rs::Office;
        use libreoffice_rs::urls;
        use libreoffice_rs::urls::DocUrl;
        let mut office = Office::new(&program_dir)?;

        let doc_url = urls::local_into_abs(&temp)?;

        let mut doc = office.document_load(doc_url)?;

        let ok = doc.save_as(dest, "pdf", None);
        if !ok {
            log::error!("❌  save pdf file failed: {}", office.get_error());
            bail!("save pdf file failed: {}", office.get_error());
        }
    }

    fs::remove_file(temp)?;

    Ok(())
}

#[cfg(target_os = "windows")]
/// 将文件转换为 pdf
pub fn convert(src: &str, dest: &str) -> Result<()> {
    // 文件后缀
    let src_ext = Path::new(src).extension();
    if src_ext.is_none() {
        return bail!("无效的文件名");
    }
    let src_ext = src_ext.unwrap().to_str().unwrap();
    if src_ext != "docx"
        && src_ext != "doc"
        && src_ext != "xlsx"
        && src_ext != "xls"
        && src_ext != "pptx"
        && src_ext != "ppt"
        && src_ext != "csv"
        && src_ext != "pdf"
    {
        return bail!("不支持的文件格式: {}", src_ext);
    }
    let dest_ext = Path::new(dest).extension();
    if dest_ext.is_none() {
        return bail!("无效的文件名");
    }
    let dest_ext = dest_ext.unwrap().to_str().unwrap();
    if dest_ext != "pdf" {
        return bail!("仅支持转为为PDF格式");
    }

    let current_dir = env::current_exe()?;

    #[cfg(test)]
    let temp_dir = "../../data/temp";
    #[cfg(not(test))]
    let temp_dir = app_dir!("data", "temp").to_string_lossy().into_owned(); //"data/temp";

    // 由于libreoffice路径不支持中文，所以先复制一个临时文件
    let temp = format!(
        "{}/{}.{}",
        temp_dir,
        uuid::Uuid::new_v4().to_string(),
        src_ext
    );
    fs::copy(src, &temp)?;

    // 转换后的临时文件路径
    // 因为libreoffice转换时，输出路径只能指定到文件夹，转换后的文件名与源文件名相同
    // 所以这里获取转换后的文件路径，转换完成后重命名到指定的路径下
    let temp_dest_file = format!(
        "{}/{}.{}",
        temp_dir,
        Path::new(&temp).file_stem().unwrap().to_str().unwrap(),
        dest_ext
    );

    let libreoffice_root_dir = env::var("LIBREOFFICE_ROOT_DIR")?;
    let command = format!("{}/program/soffice.exe", libreoffice_root_dir);

    {
        log::info!("convert {} to pdf", src);

        // libreoffice的根目录
        let lo_root_dir = env::var("LIBREOFFICE_ROOT_DIR")?;
        // libreoffice的user目录，从已安装的用户目录中复制而来
        // 在windows中默认为C:\Users\15849\AppData\Roaming\LibreOffice\4
        let base_installation = format!("{}/{}", lo_root_dir, "installation/4/user");
        // 可执行文件路径
        // 由于UserInstallation需要绝对路径，所以需要获取绝对路径
        // 临时安装目录
        let temp_installation = format!(
            //"{}/data/temp/libreoffice/{}",
            "{}/libreoffice/{}",
            temp_dir,
            uuid::Uuid::new_v4().to_string()
        )
        .replace("\\", "/");
        // 创建临时安装目录
        fs::create_dir_all(&temp_installation)?;
        // 复制用户目录
        fs_extra::dir::copy(
            &base_installation,
            &temp_installation,
            &fs_extra::dir::CopyOptions::default(),
        )?;

        let output = Command::new(command)
            // 注意这里需要指定绝对路径
            .arg(&format!(
                "-env:UserInstallation=file:///{}",
                &temp_installation
            ))
            .arg("--headless")
            .arg("--convert-to")
            .arg("pdf")
            .arg("--outdir")
            .arg(temp_dir)
            .arg(&temp)
            .output()?;
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            fs::remove_dir_all(&temp_installation)?;
            return Err(anyhow::anyhow!("convert to pdf error: {}", err));
        }
        // 没有生成文件则转换失败
        if !Path::new(&temp_dest_file).exists() {
            return Err(anyhow::anyhow!("convert file failed"));
        }

        log::info!("convert {} to pdf success", src);

        fs::rename(&temp_dest_file, &dest)?;
        fs::remove_dir_all(&temp_installation)?;
    }

    fs::remove_file(temp)?;

    Ok(())
}
/// 使用cli转换
///
/// （废弃）由于libreoffice在7.x版本存在问题，会导致在drop释放资源时卡住，所以通过子进程的方式来解决这个问题
/// 已经降低版为6.4，单由于动态链接库的环境变量问题，还是只能通过子进程启动
pub fn convert_with_cli(src: &str, dest: &str) -> anyhow::Result<()> {
    let start = std::time::Instant::now();

    let command = app_dir!("resources", "bin", "doc-to-pdf")
        .to_string_lossy()
        .into_owned();
    let output = Command::new(command).arg(src).arg(dest).output()?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("{}", err));
    }
    log::info!(
        "convert {} to pdf success, use time: {} ms",
        src,
        start.elapsed().as_millis()
    );

    Ok(())
}

#[test]
fn test_convert() -> anyhow::Result<()> {
    init()?;
    let src = r#"D:\download\doc\train(1).csv"#;
    let dest = "../../data/file/train(1).pdf";
    convert(src, dest)?;

    Ok(())
}
