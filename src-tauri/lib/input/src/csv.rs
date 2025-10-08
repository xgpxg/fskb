use crate::{Input, Split};
use std::path::Path;

pub struct CsvInput;

#[derive(Debug)]
pub struct CsvOutput {
    /// 文本内容
    pub headers: Vec<String>,
    /// 图片
    pub rows: Vec<Vec<String>>,
}

impl Input for CsvInput {
    type Output = crate::Result<CsvOutput>;

    fn read(path: impl AsRef<Path>) -> Self::Output {
        let mut reader = csv::Reader::from_path(path)?;
        let mut rows: Vec<Vec<String>> = vec![];
        let header_record = reader.headers()?;
        let headers = header_record
            .iter()
            .enumerate()
            .map(|(index, s)| {
                return if s == "" {
                    format!("unknown_{}", index)
                } else {
                    s.to_string()
                };
            })
            .collect();
        for record in reader.records() {
            rows.push(record?.iter().map(|s| s.to_string()).collect());
        }
        Ok(CsvOutput { headers, rows })
    }
}
impl CsvInput {
    pub fn extra_headers(path: impl AsRef<Path>) -> crate::Result<Vec<String>> {
        let mut reader = csv::Reader::from_path(path)?;
        let header_record = reader.headers()?;
        let headers = header_record
            .iter()
            .enumerate()
            .map(|(index, s)| {
                return if s == "" {
                    format!("unknown_{}", index)
                } else {
                    s.to_string()
                };
            })
            .collect();
        Ok(headers)
    }
}

impl CsvOutput {
    pub fn to_text(&self) -> String {
        let mut text = String::new();
        let headers = &self.headers;
        for row in self.rows.iter() {
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

impl Split for CsvOutput {
    fn split(s: String) -> Vec<String> {
        let mut result = vec![];
        for line in s.lines() {
            result.push(line.to_string());
        }
        result
    }
}

#[test]
fn test_csv() {
    let path = "/mnt/d/download/train.csv";
    let output = CsvInput::read(path).unwrap();
    println!("{:?}", output);
}
