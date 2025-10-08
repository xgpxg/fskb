use crate::db::Count;
use crate::server::model::request::ModelListReq;
use derive_builder::Builder;
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, htmlsql, htmlsql_select_page};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
pub struct Model {
    pub id: Option<i64>,
    /// 模型名称
    pub name: Option<String>,
    /// 模型描述
    pub description: Option<String>,
    /// 来源：1内置 2自建
    pub source: Option<i8>,
    /// 图标
    pub icon: Option<String>,
    /// 状态：0未启用 1已启用 2异常 3安装中（仅离线模型） 4启动中（仅离线模型）
    pub status: Option<i8>,
    /// 状态信息
    pub status_msg: Option<String>,
    /// 接口地址
    pub base_url: Option<String>,
    /// api key
    pub api_key: Option<String>,
    /// 最大token
    pub max_token: Option<i32>,
    /// 适用的任务类型：1文本生成 2视觉问答
    pub task_type: Option<i8>,
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

pub enum ModelStatus {
    /// 未启用
    Disable = 0,
    /// 已启用
    Enable = 1,
    /// 异常
    Error = 2,
    /// 安装中(仅离线模型)
    Installing = 3,
    /// 启动中(仅离线模型)
    Starting = 4,
}

pub enum ModelSource {
    /// 内置
    #[allow(unused)]
    Inner = 1,
    /// 自建
    Custom = 2,
    /// 本地
    Local = 3,
}

pub enum ModelTaskType {
    /// 文本生成
    TextGen = 1,
    /// 视觉问答
    VisionQA = 2,
}

crud!(Model {});
htmlsql_select_page!(list_page(param: &ModelListReq) -> Model => "src/db/mapper/model.html");
htmlsql!(check_name_exists(rb: &dyn Executor, name: &str, exclude_id: Option<i64>) -> Count => "src/db/mapper/model.html");
