use crate::db::model::chat_message::ChatMessage;
use dashmap::DashMap;
use serde::Serialize;
use std::sync::LazyLock;
use tauri::ipc::Channel;

mod chat_helper;
mod chat_model;
mod command;
pub(crate) mod commands;
mod request;
mod response;
mod service;

#[derive(Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub(crate) enum ChatEvent {
    // 对话开始，ChatMessage为初始状态的助手消息，content为空字符串
    Start(ChatMessage),
    // 对话进行中, ChatMessage为助手回复的流式消息
    Message(ChatMessage),
    // 对话结束，ChatMessage的content固定为[DONE]
    Done(ChatMessage),
}

// 和前端通信的channel
static STREAM_CHANNEL: LazyLock<DashMap<String, Channel<ChatEvent>>> =
    LazyLock::new(|| DashMap::new());

// 全量消息缓存
static MESSAGE_CACHE: LazyLock<DashMap<String, ChatMessage>> = LazyLock::new(|| DashMap::new());

impl ChatEvent {
    pub(crate) fn build_channel_key(kb_id: i64, message_id: i64) -> String {
        format!("{}-{}", kb_id, message_id)
    }

    fn channel_key(&self) -> String {
        match self {
            ChatEvent::Start(message) | ChatEvent::Done(message) | ChatEvent::Message(message) => {
                message.gen_channel_key()
            }
        }
    }

    pub(crate) fn append_cache_message(message: &ChatMessage) {
        let ck = message.gen_channel_key();
        // 在作用域内释放锁，否则后面update操作会导致死锁
        {
            let old = MESSAGE_CACHE.get_mut(&ck);
            if old.is_none() {
                MESSAGE_CACHE.insert(ck.clone(), message.clone());
                return;
            }
        }
        MESSAGE_CACHE.entry(ck.clone()).and_modify(|v| {
            let new_content = message.content.as_deref().unwrap_or_default();
            v.content = Some(v.content.take().unwrap_or_default() + new_content);
        });
    }

    pub(crate) async fn send(self) -> anyhow::Result<()> {
        let ck = &self.channel_key();
        if let Some(channel) = STREAM_CHANNEL.get(ck) {
            channel.send(self)?;
        }
        Ok(())
    }

    pub(crate) fn add_channel(channel_key: &str, channel: Channel<ChatEvent>) {
        // 获取并移除旧channel
        {
            STREAM_CHANNEL.remove(channel_key);
        }
        STREAM_CHANNEL.insert(channel_key.to_string(), channel);
    }

    pub(crate) fn new_resume_channel(
        channel_key: &str,
        channel: Channel<ChatEvent>,
        event: ChatEvent,
    ) -> anyhow::Result<()> {
        channel.send(event)?;
        Self::add_channel(channel_key, channel);
        Ok(())
    }

    pub(crate) fn get_cache_message(channel_key: &str) -> Option<ChatMessage> {
        MESSAGE_CACHE.get(channel_key).map(|v| v.value().clone())
    }

    // 关闭channel
    pub(crate) fn close_channel(channel_key: &str) {
        MESSAGE_CACHE.remove(channel_key);
        STREAM_CHANNEL.remove(channel_key);
    }
}
