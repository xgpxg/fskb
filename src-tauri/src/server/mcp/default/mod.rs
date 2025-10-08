use crate::server::mcp::default::kb_mcp::KbMcp;
use openai_dive::v1::resources::chat::ChatCompletionTool;

/// 内置知识库文档搜索工具名称
pub const KB_DOC_SEARCH_TOOL: &str = concat!("inner", "A-_-A", "kb_doc_search");
/// 内置知识库表格搜索工具名称
pub const KB_TABLE_SEARCH_TOOL: &str = concat!("inner", "A-_-A", "kb_table_search");
/// 查询知识库项
pub const KB_LIST_ITEM_TOOL: &str = concat!("inner", "A-_-A", "kb_list_item");
/// 内置markdown转其他文件格式工具名称
pub const MD_TO_ANY_TOOL: &str = concat!("inner", "A-_-A", "md-to-any");

pub(crate) mod kb_mcp;

pub(crate) enum DefaultMcpServer {
    KbMcp(KbMcp),
}

impl DefaultMcpServer {
    pub(crate) fn is_default_mcp(tool_name: &str) -> bool {
        tool_name == KB_DOC_SEARCH_TOOL
            || tool_name == KB_TABLE_SEARCH_TOOL
            || tool_name == KB_LIST_ITEM_TOOL
    }
    pub(crate) fn new(tool_name: &str) -> anyhow::Result<Self> {
        match tool_name {
            KB_DOC_SEARCH_TOOL | KB_TABLE_SEARCH_TOOL | KB_LIST_ITEM_TOOL => {
                Ok(DefaultMcpServer::KbMcp(KbMcp))
            }
            _ => {
                anyhow::bail!("未知的工具名称：{}", tool_name);
            }
        }
    }
    pub(crate) async fn call(&self, tool_name: &str, parameters: &str) -> anyhow::Result<String> {
        match self {
            DefaultMcpServer::KbMcp(kb_mcp) => kb_mcp.call(tool_name, parameters).await,
        }
    }

    pub(crate) fn all_tools() -> Vec<ChatCompletionTool> {
        let mut tools = vec![];
        tools.extend(kb_mcp::default_tools());
        tools
    }
}
