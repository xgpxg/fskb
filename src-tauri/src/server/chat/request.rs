use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UserMessageContent {
    pub(crate) text: String,
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub(crate) images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub(crate) audios: Option<Vec<String>>,
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub(crate) videos: Option<Vec<String>>,
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub(crate) files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub(crate) rules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) command: Option<String>,
}
fn skip_if_empty<T>(opt: &Option<Vec<T>>) -> bool {
    match opt {
        None => true,
        Some(vec) => vec.is_empty(),
    }
}
