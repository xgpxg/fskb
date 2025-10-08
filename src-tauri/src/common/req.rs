use rbatis::PageRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdReq {
    pub id: i64,
}

/// 分页trait
pub trait Pagination {
    fn page_num(&self) -> u64;
    fn page_size(&self) -> u64;

    /// 转换为rbatis的分页参数
    fn to_rb_page(&self) -> PageRequest {
        PageRequest::new(self.page_num(), self.page_size())
    }
}

#[macro_export]
macro_rules! impl_pagination {
    ($s:ty) => {
        impl crate::common::req::Pagination for $s {
            fn page_num(&self) -> u64 {
                self.page.page_num
            }

            fn page_size(&self) -> u64 {
                self.page.page_size
            }
        }
    };
}

/// 分页请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageReq {
    pub page_num: u64,
    pub page_size: u64,
}

impl Default for PageReq {
    fn default() -> Self {
        PageReq {
            page_num: 1,
            page_size: 10,
        }
    }
}
