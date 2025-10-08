use common::resources_dir;
use paddleocr::ImageData;
use std::path::{Path, PathBuf};

pub fn run(image_path: &str) -> anyhow::Result<String> {
    #[cfg(test)]
    let exe_path = "../../resources/driver/ocr/PaddleOCR-json.exe";
    #[cfg(not(test))]
    //let exe_path = "resources/driver/ocr/PaddleOCR-json.exe";
    let exe_path = resources_dir!("driver", "ocr", "PaddleOCR-json.exe");
    let mut p = paddleocr::Ppocr::new(PathBuf::from(exe_path), Default::default())
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    // 取绝对路径
    let image_path = Path::new(image_path).canonicalize()?;
    let path_buf: PathBuf = if cfg!(windows) {
        // 去掉 \\?\ 前缀
        let path_str = image_path.to_string_lossy().into_owned();
        path_str[4..].into()
    } else {
        image_path.into()
    };

    let result = p
        .ocr_and_parse(path_buf.into())
        .map_err(|e| anyhow::anyhow!(e))?;
    let mut text = String::new();
    for content in result {
        text += &content.text;
    }
    Ok(text)
}

#[test]
fn test_ocr() {
    let text = run(r#"../../data/temp/b6ffa0ef-5a6a-494c-8c1e-9eab59b82fb9.png"#).unwrap();
    println!("{}", text);
}
