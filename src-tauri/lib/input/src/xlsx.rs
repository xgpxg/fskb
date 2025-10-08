use crate::{Input, Split};
use calamine::{RangeDeserializerBuilder, Reader, Xls, Xlsx, open_workbook};
use std::path::Path;

pub struct XlsxInput;

#[derive(Debug)]
pub struct XlsxOutput {
    /// 文本内容
    pub headers: Vec<String>,
    /// 图片
    pub rows: Vec<Vec<String>>,
}

impl Input for XlsxInput {
    type Output = crate::Result<XlsxOutput>;

    fn read(path: impl AsRef<Path>) -> Self::Output {
        let extension = path.as_ref().extension().unwrap().to_str().unwrap();
        match extension {
            "xlsx" => {
                let mut workbook: Xlsx<_> = open_workbook(path)?;
                let range = workbook.worksheets();
                // 暂只支持获取第一个sheet
                let sheet = range[0].to_owned().1;
                let mut iter = RangeDeserializerBuilder::new()
                    .has_headers(false)
                    .from_range(&sheet)?;
                // 首行作为表头，只支持单个表头
                let headers: Vec<String> = iter.next().transpose()?.unwrap_or_default();
                let headers = trim_headers(headers);

                let mut rows: Vec<Vec<String>> = iter.map(|r| r.unwrap_or_default()).collect();
                trim_rows(&headers, &mut rows);

                Ok(XlsxOutput { headers, rows })
            }
            "xls" => {
                let mut workbook: Xls<_> = open_workbook(path)?;
                let range = workbook.worksheets();
                let sheet = range[0].to_owned().1;
                let mut iter = RangeDeserializerBuilder::new()
                    .has_headers(false)
                    .from_range(&sheet)?;

                // 首行作为表头，只支持单个表头
                let headers: Vec<String> = iter.next().transpose()?.unwrap_or_default();
                let headers = trim_headers(headers);

                let mut rows: Vec<Vec<String>> = iter.map(|r| r.unwrap_or_default()).collect();
                trim_rows(&headers, &mut rows);

                Ok(XlsxOutput { headers, rows })
            }
            _ => unimplemented!(),
        }
    }
}

impl XlsxInput {
    /// 提取表头
    pub fn extra_headers(path: impl AsRef<Path>) -> crate::Result<Vec<String>> {
        let extension = path.as_ref().extension().unwrap().to_str().unwrap();
        match extension {
            "xlsx" => {
                let mut workbook: Xlsx<_> = open_workbook(path)?;
                let range = workbook.worksheets();
                let sheet = range[0].to_owned().1;
                let mut iter = RangeDeserializerBuilder::new()
                    .has_headers(false)
                    .from_range(&sheet)?;

                // 首行作为表头，只支持单个表头
                let headers: Vec<String> = iter.next().transpose()?.unwrap_or_default();
                let headers = trim_headers(headers);

                Ok(headers)
            }
            "xls" => {
                let mut workbook: Xls<_> = open_workbook(path)?;
                let range = workbook.worksheets();
                let sheet = range[0].to_owned().1;
                let mut iter = RangeDeserializerBuilder::new()
                    .has_headers(false)
                    .from_range(&sheet)?;

                // 首行作为表头，只支持单个表头
                let headers: Vec<String> = iter.next().transpose()?.unwrap_or_default();
                let headers = trim_headers(headers);

                Ok(headers)
            }
            _ => unimplemented!(),
        }
    }
}

fn trim_headers(headers: Vec<String>) -> Vec<String> {
    let mut headers = headers;
    headers.insert(0, "行号".to_string());
    headers
        .iter()
        .enumerate()
        .filter_map(|(index, h)| {
            let h = h.trim();
            if h == "" {
                Some(format!("unknown_{}", index))
            } else {
                Some(h.to_string())
            }
        })
        .collect::<Vec<_>>()
}

/// 将每一行的字段数量与表头保持一致
fn trim_rows(headers: &Vec<String>, rows: &mut Vec<Vec<String>>) {
    for (row_index, row) in rows.iter_mut().enumerate() {
        // 追加行号
        row.insert(0, (row_index + 1).to_string());
        if row.len() < headers.len() {
            while row.len() < headers.len() {
                row.push("".to_string());
            }
        }
        if row.len() > headers.len() {
            while row.len() > headers.len() {
                row.pop();
            }
        }
    }
}

impl XlsxOutput {
    pub fn to_text(&self) -> String {
        let mut text = String::new();
        let headers = &self.headers;
        for (row_index, row) in self.rows.iter().enumerate() {
            for (cell_index, cell) in row.iter().enumerate() {
                text += &format!(
                    "{}: ",
                    headers.get(cell_index).unwrap_or(&"unknown".to_string())
                );
                text += cell;
                text += "\t";
            }
            text += "\n";
        }
        text
    }
}

impl Split for XlsxOutput {
    fn split(s: String) -> Vec<String> {
        let mut result = vec![];
        for line in s.lines() {
            result.push(line.to_string());
        }
        result
    }
}
#[test]
fn test_xlsx() -> crate::Result<()> {
    let path = "/mnt/d/download/W020170213333264378621.xlsx";
    let output = XlsxInput::read(path)?;
    let text = output.to_text();
    let split = XlsxOutput::split(text);
    for s in split {
        println!("{}", s);
    }
    Ok(())
}
