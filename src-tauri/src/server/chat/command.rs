use crate::server::chat::request::UserMessageContent;
use input::docx::DocxInput;
use input::pdf::PdfInput;
use input::txt::TxtInput;
use input::Input;
use std::path::Path;

pub(crate) fn is_command_message(content: &UserMessageContent) -> bool {
    content.command.is_some()
}

pub(crate) async fn run_command(content: &UserMessageContent) -> anyhow::Result<String> {
    Command::parse(content)?.execute().await
}

enum Command {
    GetText(String),
    Translate(String),
}

impl Command {
    fn parse(content: &UserMessageContent) -> anyhow::Result<Self> {
        let command = content.command.as_ref().unwrap();
        let command = command.split_whitespace().collect::<Vec<_>>();
        match command[0] {
            "GetText" => Ok(Command::GetText(
                content.files.clone().unwrap().get(0).unwrap().to_string(),
            )),
            "Translate" => Ok(Command::Translate(command[1].to_string())),
            _ => Err(anyhow::anyhow!("Unknown command")),
        }
    }
}

impl Command {
    async fn execute(&self) -> anyhow::Result<String> {
        match self {
            Command::GetText(file_path) => Ok(get_text(file_path).await?),
            Command::Translate(file_path) => Ok("".to_string()),
        }
    }
}

async fn get_text(file_path: &str) -> anyhow::Result<String> {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap();
    match ext {
        "txt" => Ok(TxtInput::read(file_path).map_err(|e| anyhow::anyhow!(e))?),
        "pdf" => Ok(PdfInput::read_and_not_snapshot(file_path)
            .map_err(|e| anyhow::anyhow!(e))?
            .pages
            .iter()
            .map(|page| page.text.clone())
            .collect::<String>()),
        "doc" | "docx" => Ok(DocxInput::read(file_path)
            .map_err(|e| anyhow::anyhow!(e))?
            .content),
        "png" | "jpg" | "jpeg" | "bmp" => Ok(ocr::run(file_path).map_err(|e| anyhow::anyhow!(e))?),
        _ => Err(anyhow::anyhow!("暂不支持从该文件中提取文本")),
    }
}
