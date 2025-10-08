use crate::server::search::request::SearchReq;
use crate::server::search::response::{SearchRes, SearchResultEvent};
use crate::server::search::{kb_search, local_search};
use std::sync::{LazyLock, OnceLock};
use tauri::ipc::Channel;
use tokio::task::JoinHandle;

//static SEARCH_HANDLE: OnceLock<JoinHandle<Result<(), anyhow::Error>>> = OnceLock::new();
pub(crate) async fn search(
    req: SearchReq,
    channel: Channel<SearchResultEvent>,
) -> anyhow::Result<()> {
    let kb = kb_search::search(&req).await?;
    channel.send(SearchResultEvent::Kb(kb))?;

    // if let Some(handle) = SEARCH_HANDLE.get() {
    //     handle.abort();
    // }

    tokio::spawn(async move {
        local_search::search(&req, channel).await?;
        Ok::<(), anyhow::Error>(())
    });

    //SEARCH_HANDLE.get_or_init(move || handle);

    Ok(())
}
