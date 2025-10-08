use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchReq {
    // 搜索关键字
    pub(crate) kw: String,
}
