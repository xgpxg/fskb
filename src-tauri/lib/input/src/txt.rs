use crate::{Input, Split};
use encoding_rs::GB18030;
use std::fs;
use std::path::Path;

pub struct TxtInput;

impl Input for TxtInput {
    type Output = crate::Result<String>;

    fn read(path: impl AsRef<Path>) -> Self::Output {
        let bytes = fs::read(path.as_ref())?;

        // 尝试 UTF-8
        if let Ok(s) = String::from_utf8(bytes.clone()) {
            return Ok(s);
        }

        // 尝试 GBK
        let (cow, _, _) = GB18030.decode(&bytes);

        Ok(cow.into_owned())
    }
}

impl Split for TxtInput {}
