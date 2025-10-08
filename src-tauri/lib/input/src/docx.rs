use crate::{Input, Split};
use docx_rs::{
    DocumentChild, ParagraphChild, RunChild, TableCellContent, TableChild, TableRowChild,
};
use image::DynamicImage;
use std::fs;
use std::path::Path;

pub struct DocxInput;

#[derive(Debug)]
pub struct DocxOutput {
    /// 文本内容
    pub content: String,
    /// 图片
    pub images: Vec<DynamicImage>,
}

impl Input for DocxInput {
    type Output = crate::Result<DocxOutput>;

    fn read(path: impl AsRef<Path>) -> Self::Output {
        let file = fs::read(path)?;
        let docx = docx_rs::read_docx(&file)?;
        let mut content = String::new();
        for document_child in docx.document.children {
            match document_child {
                DocumentChild::Paragraph(paragraph) => {
                    for child in paragraph.children {
                        if let ParagraphChild::Run(run) = child {
                            for run_child in run.children {
                                match run_child {
                                    RunChild::Text(text) => {
                                        content.push_str(&text.text);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    content.push('\n');
                }
                DocumentChild::Table(table) => {
                    table.rows.iter().for_each(|table_child| match table_child {
                        TableChild::TableRow(table_row) => {
                            table_row.cells.iter().for_each(
                                |table_row_child| match table_row_child {
                                    TableRowChild::TableCell(table_cell) => {
                                        table_cell.children.iter().for_each(|table_cell_child| {
                                            match table_cell_child {
                                                TableCellContent::Paragraph(paragraph) => {
                                                    paragraph.children.iter().for_each(
                                                        |paragraph_child| match paragraph_child {
                                                            ParagraphChild::Run(run) => {
                                                                run.children.iter().for_each(
                                                                    |run_child| match run_child {
                                                                        RunChild::Text(text) => {
                                                                            content.push_str(
                                                                                &text.text,
                                                                            );
                                                                        }
                                                                        _ => {}
                                                                    },
                                                                )
                                                            }
                                                            _ => {}
                                                        },
                                                    )
                                                }
                                                _ => {}
                                            }
                                            content.push_str(" | ");
                                        });
                                    }
                                },
                            );
                            content.push('\n');
                        }
                    })
                }
                other => {
                    println!("other: {:?}", other);
                }
            }
        }
        let mut images = Vec::new();
        docx.images.iter().for_each(|(_, _, image, _)| {
            if let Ok(image) = image::load_from_memory(&image.0) {
                images.push(image);
            }
        });
        Ok(DocxOutput { content, images })
    }
}

impl Split for DocxOutput {}
#[test]
fn test_docx() {
    let output = DocxInput::read("/mnt/d/download/活动发布审核流程-0819.docx").unwrap();
    println!("{:?}", output);
}
