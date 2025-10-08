use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NoteAddReq {
    pub(crate) kb_id: i64,
    pub(crate) title: Option<String>,
    pub(crate) summary: Option<String>,
    pub(crate) content: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NoteUpdateReq {
    pub(crate) id: i64,
    pub(crate) title: Option<String>,
    pub(crate) summary: Option<String>,
    pub(crate) content: Option<String>,
}
