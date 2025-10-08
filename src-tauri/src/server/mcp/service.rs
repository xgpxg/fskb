use crate::common::id;
use crate::db::model::knowledge_base;
use crate::db::model::knowledge_base::KnowledgeBase;
use crate::db::model::mcp_server::{McpServer, McpServerBuilder, McpServerSource, McpServerStatus};
use crate::db::model::mcp_server_define::McpServerDefine;
use crate::db::{tools, Pool};
use crate::server::mcp::request::{McpServerAddReq, McpServerUpdateReq};
use crate::server::mcp::response::{McpServerDefineListRes, McpServerListRes};
use anyhow::bail;
use common::data_dir;
use mcp::mcp_manager;
use mcp::mcpsi::Mcpsi;
use rbs::value;
use std::fs;
use std::path::Path;

pub(crate) async fn list_all_mcp_server() -> anyhow::Result<Vec<McpServerDefineListRes>> {
    let list = McpServerDefine::select_all(Pool::get()?).await?;
    let installed_list = list_installed_mcp_server().await?;
    let list = list
        .into_iter()
        .map(|item| {
            let installed_version = match installed_list
                .iter()
                .find(|v| v.name.eq(&item.name.clone().unwrap()))
            {
                Some(v) => v.installed_version.clone(),
                None => None,
            };

            McpServerDefineListRes {
                name: item.name.clone().unwrap(),
                summary: item.summary.unwrap(),
                description: item.description,
                version: item.version.clone(),
                installed: installed_version.is_some(),
                installed_version: installed_version.clone(),
                need_upgrade: item.version != installed_version,
            }
        })
        .collect::<Vec<_>>();
    Ok(list)
}

pub(crate) async fn list_installed_mcp_server() -> anyhow::Result<Vec<McpServerListRes>> {
    let list = McpServer::select_by_map(
        Pool::get()?,
        value! {
            "status": vec![McpServerStatus::NotRun as i8,
                McpServerStatus::Ok as i8,
                McpServerStatus::Error as i8,
                McpServerStatus::Starting as i8,
                McpServerStatus::Stopping as i8,
                McpServerStatus::Upgrading as i8,
            ]
        },
    )
    .await?;

    let list = list
        .into_iter()
        .map(|item| McpServerListRes {
            id: item.id.unwrap(),
            name: item.name.unwrap(),
            summary: item.summary.unwrap(),
            description: item.description,
            need_upgrade: item.installed_version != item.latest_version,
            installed_version: item.installed_version,
            latest_version: item.latest_version,
            config: item.config,
            status: item.status,
            status_msg: item.status_msg,
        })
        .collect::<Vec<_>>();

    Ok(list)
}

pub(crate) async fn install_mcp_server(name: String) -> anyhow::Result<()> {
    // 从服务端点获取服务信息
    let mcp_server = Mcpsi::get_mcp_server(&name).await?;
    // 获取已安装的
    let mut installed = McpServer::select_by_map(Pool::get()?, value! {"name": &name}).await?;

    // 如果未安装，则插入一条，状态为安装中
    // 如果已安装，则更新状态为升级中
    if installed.is_empty() {
        let mcp_server = McpServerBuilder::default()
            .id(Some(id::next()))
            .name(Some(name.clone()))
            .summary(Some(mcp_server.summary))
            .description(Some(mcp_server.description))
            .config(Some(serde_json::to_string(&mcp_server.config)?))
            .configurable(mcp_server.configurable)
            .installed_version(mcp_server.version.clone())
            .latest_version(mcp_server.version.clone())
            .status(Some(McpServerStatus::Installing as i8))
            .source(Some(McpServerSource::Platform as i8))
            .build()?;
        McpServer::insert(Pool::get()?, &mcp_server).await?;
        installed.push(mcp_server);
    } else {
        // 先停止
        mcp_manager::stop_mcp_server(&name).await?;
        // 更新状态为升级中
        update_mcp_server_status(&name, McpServerStatus::Upgrading, "").await?;
    }

    // 安装目录
    //let dir = Path::new("data/mcp");
    let dir = data_dir!("mcp");

    // 执行安装
    match Mcpsi::install_mcp_server(&name, dir.to_path_buf()).await {
        Ok(_) => {}
        Err(e) => {
            log::error!("Mcp Server install fail: {}", e);
            update_mcp_server_status(&name, McpServerStatus::Error, e.to_string().as_str()).await?;
            bail!(e)
        }
    }

    let installed = installed.first().unwrap().clone();

    update_mcp_server_status(&name, McpServerStatus::Starting, "").await?;

    // 更新版本
    update_mcp_server_version(&name, &mcp_server.version.unwrap()).await?;
    // 启动
    mcp_manager::run_mcp_server(&installed.config.unwrap_or_default()).await?;

    update_mcp_server_status(&name, McpServerStatus::Ok, "").await?;

    Ok(())
}
async fn update_mcp_server_status(
    name: &str,
    status: McpServerStatus,
    status_msg: &str,
) -> anyhow::Result<()> {
    McpServer::update_by_map(
        Pool::get()?,
        &McpServerBuilder::default()
            .name(Some(name.to_string()))
            .status(Some(status as i8))
            .status_msg(Some(status_msg.to_string()))
            .build()?,
        value! {
            "name" : &name,
        },
    )
    .await?;
    Ok(())
}
async fn update_mcp_server_version(name: &str, version: &str) -> anyhow::Result<()> {
    McpServer::update_by_map(
        Pool::get()?,
        &McpServerBuilder::default()
            .name(Some(name.to_string()))
            .installed_version(Some(version.to_string()))
            .build()?,
        value! {
            "name" : &name,
        },
    )
    .await?;
    Ok(())
}
pub(crate) async fn uninstall_mcp_server(name: String) -> anyhow::Result<()> {
    let mcp_server = McpServer::select_by_map(Pool::get()?, value! {"name": &name}).await?;
    if mcp_server.is_empty() {
        return bail!("MCP工具不存在");
    }
    let mcp_server = mcp_server.first().unwrap();

    // 停止
    mcp_manager::stop_mcp_server(&name).await?;

    // 卸载
    //let dir = Path::new("data/mcp").join(&name);
    let dir = data_dir!("mcp").join(&name);
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }

    McpServer::delete_by_map(
        Pool::get()?,
        value! {
            "name" : &name,
        },
    )
    .await?;

    knowledge_base::remove_mcp_server_id(Pool::get()?, mcp_server.id.unwrap()).await?;

    Ok(())
}

pub(crate) async fn run_mcp_server(name: String) -> anyhow::Result<()> {
    let mcp_server = McpServer::select_by_map(Pool::get()?, value! {"name": &name}).await?;
    if mcp_server.is_empty() {
        return bail!("工具不存在");
    }
    let mcp_server = mcp_server.first().unwrap();

    update_mcp_server_status(&name, McpServerStatus::Starting, "").await?;

    match mcp_manager::run_mcp_server(&mcp_server.config.clone().unwrap()).await {
        Ok(_) => {
            update_mcp_server_status(&name, McpServerStatus::Ok, "").await?;
        }
        Err(e) => {
            update_mcp_server_status(&name, McpServerStatus::Error, &e.to_string()).await?;
        }
    }

    Ok(())
}

pub(crate) async fn stop_mcp_server(name: String) -> anyhow::Result<()> {
    update_mcp_server_status(&name, McpServerStatus::Stopping, "").await?;

    mcp_manager::stop_mcp_server(&name).await?;

    update_mcp_server_status(&name, McpServerStatus::NotRun, "").await?;

    Ok(())
}

pub(crate) async fn add_mcp_server(req: McpServerAddReq) -> anyhow::Result<()> {
    let mcp_server = McpServerBuilder::default()
        .id(Some(id::next()))
        .name(Some(req.name))
        .summary(Some(req.summary))
        .description(req.description)
        .config(Some(req.config))
        .source(Some(McpServerSource::Custom as i8))
        .status(Some(McpServerStatus::NotRun as i8))
        .installed_version(Some("0.0.0".to_string()))
        .latest_version(Some("0.0.0".to_string()))
        .create_time(Some(tools::now()))
        .build()?;

    McpServer::insert(Pool::get()?, &mcp_server).await?;

    Ok(())
}
pub(crate) async fn update_mcp_server(req: McpServerUpdateReq) -> anyhow::Result<()> {
    let old = McpServer::select_by_map(Pool::get()?, value! {"id": req.id}).await?;
    if old.is_empty() {
        return bail!("工具不存在");
    }
    let old = old.first().unwrap();

    let mcp_server = McpServerBuilder::default()
        .name(Some(req.name))
        .summary(Some(req.summary))
        .description(req.description)
        .config(Some(req.config.clone()))
        .update_time(Some(tools::now()))
        .build()?;

    McpServer::update_by_map(Pool::get()?, &mcp_server, value! {"id": req.id}).await?;

    let name = &old.name.clone().unwrap();
    // 配置变更，重启服务
    if !old.config.clone().unwrap_or_default().eq(&req.config) {
        update_mcp_server_status(name, McpServerStatus::Stopping, "").await?;
        mcp_manager::stop_mcp_server(name).await?;
        update_mcp_server_status(name, McpServerStatus::Starting, "").await?;
        mcp_manager::run_mcp_server(&old.config.clone().unwrap()).await?;
        update_mcp_server_status(name, McpServerStatus::Ok, "").await?;
    }

    Ok(())
}

pub(crate) async fn start_mcp_server_on_start() {
    let list = McpServer::select_by_map(
        Pool::get().unwrap(),
        value! {
            "status" :McpServerStatus::Ok as i8,
        },
    )
    .await
    .unwrap();
    for item in list.iter() {
        log::info!(
            "App restart, starting mcp server {}",
            item.name.clone().unwrap()
        );
        match mcp_manager::run_mcp_server(&item.config.clone().unwrap()).await {
            Ok(_) => {}
            Err(e) => {
                log::error!(
                    "App restart, mcp server {} start error: {}",
                    item.name.clone().unwrap(),
                    e
                );
                update_mcp_server_status(
                    item.name.clone().unwrap().as_str(),
                    McpServerStatus::Error,
                    e.to_string().as_str(),
                )
                .await
                .unwrap();
            }
        }
    }
}
