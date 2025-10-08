use crate::common::id;
use crate::db::model::note;
use crate::db::model::note::{Note, NoteBuilder};
use crate::db::{tools, Pool};
use crate::server::note::request::{NoteAddReq, NoteUpdateReq};
use crate::server::note::response::NoteListRes;
use anyhow::bail;
use rbs::value;

pub(crate) async fn add(req: NoteAddReq) -> anyhow::Result<i64> {
    let note = NoteBuilder::default()
        .id(Some(id::next()))
        .knowledge_base_id(Some(req.kb_id))
        .title(req.title)
        .summary(req.summary)
        .content(req.content)
        .create_time(Some(tools::now()))
        .build()?;
    Note::insert(Pool::get()?, &note).await?;
    Ok(note.id.unwrap())
}

pub(crate) async fn list_all_notes(
    kb_id: i64,
    filter_text: Option<String>,
) -> anyhow::Result<Vec<NoteListRes>> {
    let list = note::list_notes(Pool::get()?, kb_id, filter_text).await?;
    let mut list = list
        .into_iter()
        .map(|item| NoteListRes { inner: item })
        .collect::<Vec<_>>();
    list.sort_by(|a, b| b.inner.id.cmp(&a.inner.id));
    Ok(list)
}

pub(crate) async fn delete_note(id: i64) -> anyhow::Result<()> {
    Note::delete_by_map(
        Pool::get()?,
        value! {
            "id": id,
        },
    )
    .await?;
    Ok(())
}

pub(crate) async fn update_note(req: NoteUpdateReq) -> anyhow::Result<()> {
    let update = NoteBuilder::default()
        .title(req.title)
        .summary(req.summary)
        .content(req.content)
        .build()?;
    Note::update_by_map(
        Pool::get()?,
        &update,
        value! {
            "id": req.id,
        },
    )
    .await?;

    Ok(())
}

pub(crate) async fn gen_note_title_and_summary(id: i64) -> anyhow::Result<()> {
    let note = Note::select_by_map(
        Pool::get()?,
        value! {
            "id": id
        },
    )
    .await?;
    if note.is_empty() {
        bail!("笔记不存在");
    }
    let content = note.first().unwrap().clone().content.unwrap_or_default();
    let (title, summary) = summary::extract_summary(&content).await;

    let update = NoteBuilder::default()
        .title(Some(title))
        .summary(Some(summary))
        .build()?;
    Note::update_by_map(
        Pool::get()?,
        &update,
        value! {
            "id": id,
        },
    )
    .await?;
    Ok(())
}
