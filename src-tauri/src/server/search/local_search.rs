use crate::server::search::request::SearchReq;
use crate::server::search::response::{LocalSearchItemRes, LocalSearchRes, SearchResultEvent};
use common::resources_dir;
use futures_util::future::join_all;
use std::fs;
use std::path::Path;
use std::process::ExitStatus;
use sysinfo::{KillError, ProcessesToUpdate, Signal, System};
use tauri::ipc::Channel;
use tokio::join;
use tokio::process::Command;

pub(crate) async fn search(
    req: &SearchReq,
    channel: Channel<SearchResultEvent>,
) -> anyhow::Result<()> {
    let kw = req.kw.clone();
    if kw.is_empty() {
        return Ok(());
    }

    // 先kill掉搜索进程
    kill_process_by_name("rg.exe");

    let mut paths = Vec::new();
    for c in 'D'..='Z' {
        let p = format!("{}:/", c);
        if Path::new(&p).exists() {
            paths.push(p);
        }
    }

    let mut dirs = vec![];
    // 获取paths下第一层路径
    for path in &paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(|e| e.ok()) {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        dirs.push(entry.path().to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    let mut handles = Vec::new();
    for dir in dirs {
        let handle = tokio::spawn({
            let channel = channel.clone();
            let kw = kw.clone();
            async move {
                _search(kw.as_str(), &dir, channel.clone()).await?;
                Ok::<(), anyhow::Error>(())
            }
        });
        handles.push(handle);
    }

    join_all(handles).await;

    channel.send(SearchResultEvent::Local(LocalSearchRes {
        items: vec![],
        has_next: false,
    }))?;

    Ok(())
}

async fn _search(kw: &str, dir: &str, channel: Channel<SearchResultEvent>) -> anyhow::Result<()> {
    log::info!("searching in {}", dir);
    let exe = resources_dir!("bin", "rg.exe")
        .to_string_lossy()
        .into_owned();
    let mut command = Command::new(exe);
    command.arg(dir);
    command.arg("--files");
    command.arg("--glob").arg(format!("*{}*", kw));
    command.arg("--glob-case-insensitive");

    // 执行命令并获取输出
    let output = command.output().await?;

    if !output.status.success() {
        log::warn!("{}", String::from_utf8_lossy(&output.stderr));
    }

    let output_str = String::from_utf8(output.stdout)?;

    for path in output_str.lines() {
        let filename = Path::new(path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let item = LocalSearchItemRes {
            filename,
            filepath: path.to_string(),
        };

        log::info!("local search item: {:?}", item);
        channel.send(SearchResultEvent::Local(LocalSearchRes {
            items: vec![item],
            has_next: true,
        }))?
    }

    log::info!("search in {} done", dir);

    Ok(())
}

fn kill_process_by_name(process_name: &str) {
    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    for (pid, process) in sys.processes() {
        if process.name() == process_name {
            match process.kill_and_wait() {
                Ok(_) => {}
                Err(e) => {
                    log::warn!("Error killing process: {:?}", e);
                }
            };
            println!("Killed process {} with PID {:?}", process_name, pid);
        }
    }
}
