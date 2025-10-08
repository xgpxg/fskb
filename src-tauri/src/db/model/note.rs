use derive_builder::Builder;
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, htmlsql};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
pub struct Note {
    pub id: Option<i64>,
    /// 知识库ID
    pub knowledge_base_id: Option<i64>,
    /// 标题
    pub title: Option<String>,
    /// 摘要
    pub summary: Option<String>,
    /// 内容
    pub content: Option<String>,
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
    /// 是否删除
    pub is_delete: Option<i8>,
}

crud!(Note {});
htmlsql!(list_notes(rb: &dyn Executor,knowledge_base_id: i64, filter_text: Option<String>) ->Vec<Note> => "src/db/mapper/note.html");
