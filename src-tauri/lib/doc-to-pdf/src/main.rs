use doc_to_pdf::convert;
use std::env::args;

fn main() -> anyhow::Result<()> {
    let src = args().nth(1).expect("source file not be empty");
    let dest = args().nth(2).expect("dest file not be empty");

    convert(&src, &dest)?;

    Ok(())
}
