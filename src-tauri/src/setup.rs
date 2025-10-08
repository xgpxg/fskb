use crate::{db, init_dir, server, task, VERSION};
use common::app_dir;
use serde::Serialize;
use std::fs;
use std::os::windows::process::CommandExt;
use std::process::{exit, Command};
use tauri::{App, Emitter, Manager};
use updater::{PhaseRunner, PhaseStatus};

/// 启动前置处理
///
/// 1. 检查是否需要更新
/// 2. 检查资源目录是否正确
pub(crate) fn setup(app: &mut App) -> anyhow::Result<()> {
    let splash_window = app.get_webview_window("splashscreen").unwrap();
    let main_window = app.get_webview_window("main").unwrap();

    // 检查是否需要展示启动屏
    // 需要满足以下条件之一：
    // 1. 需要更新，即存在.update文件且当前版本非最新版本。.update文件由用户在系统设置中手动点击更新后生成。
    // 2. 需要加载资源，即不存在resources目录，或者resources版本和应用中要求的resources版本不一致
    // 3. 新版已安装，但还未执行变更时，即存在.update时，也需要显示启动屏
    tauri::async_runtime::spawn(async move {
        #[cfg(not(debug_assertions))]
        let pass = match updater::Phase::check(VERSION).await {
            Ok(pass) => pass,
            Err(err) => {
                log::error!("{}", err);
                exit(1)
            }
        };
        #[cfg(debug_assertions)]
        let pass = true;
        if pass {
            // 还未执行变更，显示启动屏，提示正在执行变更
            if fs::exists(app_dir!(".update"))? {
                // 显示启动屏
                splash_window.show()?;
                // 仅模拟一下进度，提升体验
                for i in 0..=100 {
                    splash_window.emit("setup", SetupEvent::ApplyUpdate { progress: i as f64 })?;
                    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
                }
            }
            // 应用内部初始化
            match init().await {
                Ok(_) => {}
                Err(e) => {
                    log::error!("{}", e);
                    exit(1);
                }
            }

            splash_window.close()?;

            // 删除.update文件
            let _ = fs::remove_file(app_dir!(".update"));

            // 延时一下 等待前端加载完成
            // 这里后续优化，先偷个懒
            #[cfg(debug_assertions)]
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            #[cfg(not(debug_assertions))]
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;

            main_window.emit("loaded", None::<String>)?;
            // 显示主窗口
            main_window.show()?;
            return Ok(());
        }
        if !pass {
            // 显示启动屏
            splash_window.show()?;
        }

        // 检查更新
        updater::Phase::CheckUpdate {
            current_version: VERSION.to_string(),
        }
        .run(|status| match status {
            PhaseStatus::Running {
                downloaded,
                total,
                progress,
            } => {
                splash_window
                    .emit(
                        "setup",
                        SetupEvent::Update {
                            total,
                            downloaded,
                            progress,
                        },
                    )
                    .unwrap();
            }
            PhaseStatus::Finished { file_path } => {
                // 调用安装程序
                let mut command = Command::new("cmd");
                match file_path {
                    None => {
                        splash_window
                            .emit(
                                "setup",
                                SetupEvent::Error("Download package fail".to_string()),
                            )
                            .unwrap();
                    }
                    Some(file_path) => {
                        log::info!("Installer downloaded，file path: {}", file_path);
                        const CREATE_NO_WINDOW: u32 = 0x08000000;
                        let _ = command
                            .arg("/C")
                            .arg("start")
                            .arg(&file_path)
                            .creation_flags(CREATE_NO_WINDOW)
                            .spawn();
                        // 关闭本程序
                        exit(0);
                    }
                }
            }
            PhaseStatus::Skip(why) => {
                log::info!("Skip update, because {}", why);
            }
            PhaseStatus::Error(e) => {
                splash_window.emit("setup", SetupEvent::Error(e)).unwrap();
            }
        })
        .await?;

        // 检查资源
        // 当当前版本要求的资源版和已有的版本不一致时，会走到这里
        updater::Phase::CheckResources {
            current_version: VERSION.to_string(),
        }
        .run(|status| match status {
            PhaseStatus::Running {
                downloaded,
                total,
                progress,
            } => {
                splash_window
                    .emit(
                        "setup",
                        SetupEvent::LoadResource {
                            total,
                            downloaded,
                            progress,
                        },
                    )
                    .unwrap();
            }
            PhaseStatus::Finished { .. } => {}
            PhaseStatus::Skip(why) => {
                log::info!("Skip check resource, because {}", why);
            }
            PhaseStatus::Error(e) => {
                splash_window.emit("setup", SetupEvent::Error(e)).unwrap();
            }
        })
        .await?;

        // 资源更新完成后，不需要重启应用，直接初始化后进入即可。

        // 应用内部初始化
        match init().await {
            Ok(_) => {}
            Err(e) => {
                log::error!("{}", e);
                exit(1);
            }
        }

        // 清理.update文件
        // 这里再次清理是因为防止某些情况下，更新完成后，.update未被删除掉
        let _ = fs::remove_file(app_dir!(".update"));

        // 关闭启动屏
        splash_window.close()?;

        main_window.emit("loaded", None::<String>)?;
        // 显示主窗口
        main_window.show()?;

        Ok::<(), anyhow::Error>(())
    });
    Ok(())
}

async fn init() -> anyhow::Result<()> {
    // 初始化目录
    init_dir()?;
    // 初始化id生成器
    crate::common::id::init();
    // 初始化数据库
    db::init().await;
    // 初始化向量库
    engine::init().await;
    // 初始化嵌入模型
    embedding::init().await;
    // 初始化doc-to-pdf
    doc_to_pdf::init()?;
    // 初始化MCP
    mcp::init();
    // 启动定时任务
    task::start().await?;

    // 可以异步执行的初始化
    tauri::async_runtime::spawn(async move {
        // 启动MCP服务
        server::mcp::init().await;
        //启动离线模型
        server::model::init().await;
    });

    Ok(())
}

#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data")]
pub(crate) enum SetupEvent {
    Update {
        total: u64,
        downloaded: u64,
        progress: f64,
    },
    LoadResource {
        total: u64,
        downloaded: u64,
        progress: f64,
    },
    ApplyUpdate {
        progress: f64,
    },
    Error(String),
}
