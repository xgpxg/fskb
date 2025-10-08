use crate::common::id;
use crate::db::model::mcp_server_define::{McpServerDefine, McpServerDefineBuilder};
use crate::db::{tools, Pool};
use crate::{db, db_error};
use mcp::mcpsi::Mcpsi;
use rbs::value;

pub(crate) async fn sync_all_mcp_servers() {
    let list = match Mcpsi::list_mcp_servers().await {
        Ok(list) => list,
        Err(e) => {
            log::error!("Sync mcp servers error: {}", e);
            return;
        }
    };

    // 更新到数据库
    let mut mcp_server_defines = vec![];
    for item in list.into_iter() {
        let mcp_server_define = McpServerDefineBuilder::default()
            .id(Some(id::next()))
            .name(Some(item.name))
            .summary(Some(item.summary))
            .description(Some(item.description))
            .url(Some(item.url))
            .config(Some(serde_json::to_string(&item.config).unwrap()))
            .version(item.version)
            .configurable(item.configurable)
            .create_time(Some(tools::now()))
            .build()
            .unwrap();
        mcp_server_defines.push(mcp_server_define);
    }
    db::tx(|tx| {
        let value = mcp_server_defines.clone();
        async move {
            McpServerDefine::delete_by_map(&tx, value! {})
                .await
                .map_err(|e| db_error!(e))?;
            McpServerDefine::insert_batch(&tx, &value, 10)
                .await
                .map_err(|e| db_error!(e))?;
            Ok(())
        }
    })
    .await
    .unwrap();
}
