use crate::db::model::note::Note;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(default)]
pub(crate) struct NoteListRes {
    #[serde(flatten)]
    pub(crate) inner: Note,
}
