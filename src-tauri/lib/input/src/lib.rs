pub mod csv;
pub mod docx;
pub mod md;
pub mod pdf;
pub mod txt;
pub mod url;
pub mod xlsx;

use std::cmp::min;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub trait Input {
    type Output;
    fn read(path: impl AsRef<Path>) -> Self::Output;
}

pub trait Split {
    fn split(s: String) -> Vec<String> {
        // 每512个字符切分
        let mut result = vec![];
        let chars: Vec<char> = s.chars().collect();
        let chunk_size = 512;
        //let overlap = 128;

        let mut start = 0;
        while start < chars.len() {
            let end = start + chunk_size;
            let chunk: String = chars[start..min(end, chars.len())].iter().collect();
            result.push(chunk);
            start = end;
        }
        result
    }
}
