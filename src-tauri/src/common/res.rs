use crate::common::result::AppError;
use rbatis::Page;
use serde::{Deserialize, Serialize};
use serde_json::json;

///通用Json响应返回
#[derive(Debug, Serialize, Deserialize)]
pub struct Res<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

///原始数据返回，返回文本或字节数组
pub enum RawData {
    Text(String),
    Bytes(Vec<u8>),
    Error(String),
}

/// 响应成功
const SUCCESS_CODE: i32 = 0;
/// 系统错误
const ERROR_CODE: i32 = 1;

impl<T> Res<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Res {
            code: SUCCESS_CODE,
            msg: "".to_string(),
            data: Some(data),
        }
    }

    pub fn error(msg: &str) -> Self {
        Res {
            code: ERROR_CODE,
            msg: msg.to_string(),
            data: None,
        }
    }

    pub fn from_error(error: AppError) -> Self {
        match error {
            // MessageError传递给调用方
            AppError::MessageError(e) => Res {
                code: ERROR_CODE,
                msg: e.to_string(),
                data: None,
            },
            // MessageCodeError传递给调用方
            AppError::MessageCodeError(code, e) => Res {
                code,
                msg: e.to_string(),
                data: None,
            },
            // 其他错误，打印错误日志，对外屏蔽错误细节
            e => {
                log::error!("{}", e);
                Res {
                    code: ERROR_CODE,
                    msg: "系统异常".to_string(),
                    data: None,
                }
            }
        }
    }

    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    pub fn to_json_string(&self) -> String {
        json!(&self).to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageRes<T> {
    pub page_num: u64,
    pub page_size: u64,
    pub total: u64,
    pub list: Vec<T>,
}

pub trait IntoPageRes<I, T>
where
    I: Send + Sync,
    T: Send + Sync,
{
    fn convert_to_page_res<F>(self, f: F) -> PageRes<T>
    where
        F: Fn(Vec<I>) -> Vec<T>;
}

impl<I, T> IntoPageRes<I, T> for Page<I>
where
    I: Send + Sync,
    T: Send + Sync,
{
    fn convert_to_page_res<F>(self, f: F) -> PageRes<T>
    where
        F: Fn(Vec<I>) -> Vec<T>,
    {
        let list = f(self.records);
        PageRes {
            page_num: self.page_no,
            page_size: self.page_size,
            total: self.total,
            list,
        }
    }
}

impl<I> Into<PageRes<I>> for Page<I>
where
    I: Send + Sync,
{
    fn into(self) -> PageRes<I> {
        PageRes {
            page_num: self.page_no,
            page_size: self.page_size,
            total: self.total,
            list: self.records,
        }
    }
}
