use derive_builder::Builder;
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, htmlsql, htmlsql_select_page};
use serde::{Deserialize, Serialize};

/// 用户
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
pub struct ChatMessage {
    pub id: Option<i64>,
    /// 消息ID
    pub message_id: Option<i64>,
    /// 父级消息ID
    pub parent_message_id: Option<i64>,
    /// 知识库ID
    pub knowledge_base_id: Option<i64>,
    /// 消息角色：system、user、assistant
    pub role: Option<String>,
    /// 消息内容。
    /// 助手消息为文本格式，
    /// 用户消息为格式：
    /// ```json
    /// {
    ///     "text": "xxx",
    ///     "images": ["xxx"],
    ///     "audios": ["xxx"],
    ///     "videos": ["xxx"],
    ///     "files": ["xxx"],
    /// }
    /// ```
    /// 统一处理为字符串
    #[serde(deserialize_with = "crate::common::deserialize_to_string")]
    pub content: Option<String>,
    /// 消息状态：pending、finished、error。
    /// - 规定：回复中的消息状态均为pending，不论回复的时成功还是失败。
    /// - 规定：finished和error状态仅在入库时进行修改，推送过程中保持pending不变。
    pub status: Option<String>,
    /// 创建人ID
    pub create_user_id: Option<i64>,
    /// 修改人ID
    pub update_user_id: Option<i64>,
    /// 创建时间
    pub create_time: Option<DateTime>,
    /// 更新时间
    pub update_time: Option<DateTime>,
    /// 备注
    pub remark: Option<String>,
    /// 用户ID
    pub user_id: Option<i64>,
    ///  租户ID
    pub tenant_id: Option<i64>,
    /// 是否删除
    pub is_delete: Option<i8>,
}

impl ChatMessage {
    pub fn gen_channel_key(&self) -> String {
        format!(
            "{}-{}",
            &self.knowledge_base_id.clone().unwrap(),
            &self.message_id.unwrap()
        )
    }
}

#[derive(strum_macros::Display)]
pub enum ChatMessageStatus {
    #[strum(to_string = "pending")]
    Pending,
    #[strum(to_string = "finished")]
    Finished,
    #[strum(to_string = "error")]
    Error,
}

#[derive(strum_macros::Display)]
pub enum ChatMessageRole {
    #[strum(to_string = "system")]
    #[allow(unused)]
    System,
    #[strum(to_string = "user")]
    User,
    #[strum(to_string = "assistant")]
    Assistant,
}

impl From<&str> for ChatMessageRole {
    fn from(role: &str) -> Self {
        match role {
            "system" => ChatMessageRole::System,
            "user" => ChatMessageRole::User,
            "assistant" => ChatMessageRole::Assistant,
            _ => ChatMessageRole::User,
        }
    }
}

impl From<String> for ChatMessageRole {
    fn from(role: String) -> Self {
        match role.as_str() {
            "system" => ChatMessageRole::System,
            "user" => ChatMessageRole::User,
            "assistant" => ChatMessageRole::Assistant,
            _ => ChatMessageRole::User,
        }
    }
}

crud!(ChatMessage {});
htmlsql!(last_message_id(rb: &dyn Executor,knowledge_base_id: i64) -> Option<i64> => "src/db/mapper/chat_message.html");
htmlsql!(list_history_messages(rb: &dyn Executor,knowledge_base_id: i64, last_message_id: Option<i64>) ->Vec<ChatMessage> => "src/db/mapper/chat_message.html");
htmlsql!(list_history_messages_limit(rb: &dyn Executor,knowledge_base_id: i64, limit:Option<i32>) ->Vec<ChatMessage> => "src/db/mapper/chat_message.html");
htmlsql!(get_one_message(rb: &dyn Executor,knowledge_base_id: i64,message_id: i64) -> Option<ChatMessage> => "src/db/mapper/chat_message.html");
