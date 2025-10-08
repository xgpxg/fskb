use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct McpServerAddReq {
    pub(crate) name: String,
    pub(crate) summary: String,
    pub(crate) description: Option<String>,
    pub(crate) config: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct McpServerUpdateReq {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) summary: String,
    pub(crate) description: Option<String>,
    pub(crate) config: String,
}
