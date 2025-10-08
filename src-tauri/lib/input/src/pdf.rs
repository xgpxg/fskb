use crate::{Input, Split};
use common::{resources_dir, temp_dir};
use log::log;
use pdfium_render::prelude::{PdfPageObjectsCommon, Pdfium};
use std::env::temp_dir;
use std::path::Path;

pub struct PdfInput;

#[derive(Debug)]
pub struct PdfOutput {
    /// 每一页的内容
    pub pages: Vec<PageContent>,
}

/// 页面内容
#[derive(Debug)]
pub struct PageContent {
    /// 页码，从0开始
    pub page_index: usize,
    /// 页面内容
    pub text: String,
    /// 快照图片
    pub snapshot: String,
}

#[cfg(target_os = "linux")]
const LIB_PATH: &str = "resources/dylib/linux/libpdfium.so";
//#[cfg(target_os = "windows")]
//const LIB_PATH: &str = "resources/dylib/windows/pdfium.dll";

impl Input for PdfInput {
    type Output = crate::Result<PdfOutput>;

    fn read(path: impl AsRef<Path>) -> Self::Output {
        let mut data = vec![];
        #[cfg(target_os = "windows")]
        let lib_path = resources_dir!("dylib", "windows", "pdfium.dll");
        let binds = Pdfium::bind_to_library(lib_path)?;
        Pdfium::new(binds)
            .load_pdf_from_file(&path, None)?
            .pages()
            .iter()
            .enumerate()
            .for_each(|(index, page)| {
                // 提取文本
                let text = match page.text() {
                    Ok(text) => &text.all(),
                    Err(err) => {
                        log::error!(
                            "⚠️  read pdf text fail, file: {}, current page: {}, error: {}",
                            path.as_ref().to_string_lossy(),
                            index,
                            err
                        );
                        ""
                    }
                };

                // 毫米转像素，转换为 300 DPI 下的像素值
                let mm_to_pixel = |mm: f32, dpi: f32| (mm * dpi / 25.4) as u32;
                let width_pixel = mm_to_pixel(page.width().to_mm(), 150.0) as i32;
                let height_pixel = mm_to_pixel(page.height().to_mm(), 150.0) as i32;
                let image = page.render(width_pixel, height_pixel, None).unwrap();
                // 保存图片
                //let image_save_path = format!("data/temp/{}.png", uuid::Uuid::new_v4().to_string());
                let image_save_path =
                    temp_dir!(format!("{}.png", uuid::Uuid::new_v4().to_string()))
                        .to_string_lossy()
                        .into_owned();

                let image = image.as_image();
                image.save(&image_save_path).expect("save pdf image fail");

                log::info!("saved pdf snapshot image to: {}", image_save_path);

                data.push(PageContent {
                    page_index: index,
                    text: text.to_string(),
                    snapshot: image_save_path,
                })
            });

        Ok(PdfOutput { pages: data })
    }
}

impl PdfInput {
    pub fn read_and_not_snapshot(path: impl AsRef<Path>) -> crate::Result<PdfOutput> {
        let mut data = vec![];
        #[cfg(target_os = "windows")]
        let lib_path = resources_dir!("dylib", "windows", "pdfium.dll");
        let binds = Pdfium::bind_to_library(lib_path)?;
        Pdfium::new(binds)
            .load_pdf_from_file(&path, None)?
            .pages()
            .iter()
            .enumerate()
            .for_each(|(index, page)| {
                // 提取文本
                let text = match page.text() {
                    Ok(text) => &text.all(),
                    Err(err) => {
                        log::error!(
                            "⚠️  read pdf text fail, file: {}, current page: {}, error: {}",
                            path.as_ref().to_string_lossy(),
                            index,
                            err
                        );
                        ""
                    }
                };
                data.push(PageContent {
                    page_index: index,
                    text: text.to_string(),
                    snapshot: "".to_string(),
                })
            });

        Ok(PdfOutput { pages: data })
    }
}
#[derive(Debug)]
pub struct PdfSplitResult {
    /// 文本，512个字符
    pub text: String,
    /// 引用的快照图片
    pub snapshot: Vec<String>,
}
impl PdfOutput {
    pub fn split(self) -> Vec<PdfSplitResult> {
        let mut result = Vec::new();
        let mut temp_text = String::new();
        let mut temp_ref_snapshots = Vec::new();
        for page_content in self.pages {
            // 当前页文本
            let mut text = page_content.text;
            // 当前页快照图片
            let snapshot = page_content.snapshot;
            // 临时存储
            temp_ref_snapshots.push(snapshot.clone());
            // 不足512的
            let mut count = temp_text.chars().count() + text.chars().count();
            while count > 512 {
                let pos = 512 - temp_text.chars().count();
                let first = text.chars().take(pos).collect::<String>();
                let last = text.chars().skip(pos).collect::<String>();

                result.push(PdfSplitResult {
                    text: format!("{}{}", temp_text, first),
                    snapshot: temp_ref_snapshots.clone(),
                });
                text = last.to_string();
                temp_text.clear();
                temp_ref_snapshots.clear();
                temp_ref_snapshots.push(snapshot.clone());
                count = text.chars().count();
            }
            temp_text.push_str(&text);
        }
        if !temp_text.is_empty() {
            result.push(PdfSplitResult {
                text: temp_text.to_string(),
                snapshot: temp_ref_snapshots.clone(),
            });
        }
        result
    }
}

#[test]
fn test_pdf() {
    let output = PdfInput::read("/mnt/d/download/家庭种菜超简单cx.pdf").unwrap();
    println!("{:?}", output);
}

#[test]
fn test_split() {}
