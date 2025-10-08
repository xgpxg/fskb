use crate::{Input, Split};
use std::fs;
use std::path::Path;

pub struct MdInput;
impl Input for MdInput {
    type Output = crate::Result<String>;

    fn read(path: impl AsRef<Path>) -> Self::Output {
        Ok(fs::read_to_string(path)?)
    }
}
impl Split for MdInput {}
#[test]
fn test_md() {
    let output = MdInput::read("/mnt/d/download/rknn.md").unwrap();
    println!("{}", output);
}
