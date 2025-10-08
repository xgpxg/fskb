use crate::McpClientManage;
use rmcp::model::{CallToolRequestParam, CallToolResult, JsonObject, Tool};
use std::sync::LazyLock;

static MCP_FACTORY: LazyLock<McpClientManage> = LazyLock::new(|| McpClientManage::new());

pub async fn run_mcp_server(config: &str) -> anyhow::Result<()> {
    MCP_FACTORY.run(config).await?;
    Ok(())
}

pub async fn list_all_tools(name: &str) -> anyhow::Result<Vec<Tool>> {
    MCP_FACTORY.list_all_tools(name).await
}

pub async fn stop_mcp_server(name: &str) -> anyhow::Result<()> {
    MCP_FACTORY.stop(name).await
}

pub async fn call_tool(
    server_name: &str,
    tool_name: &str,
    args: Option<String>,
) -> anyhow::Result<CallToolResult> {
    let result = MCP_FACTORY
        .call_tool(
            server_name,
            CallToolRequestParam {
                name: tool_name.to_string().into(),
                arguments: if let Some(args) = args {
                    Some(JsonObject::from(serde_json::from_str(&args)?))
                } else {
                    None
                },
            },
        )
        .await?;
    Ok(result)
}
