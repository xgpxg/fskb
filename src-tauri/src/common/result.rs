pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// 初始化错误
    #[error("init error: {0}")]
    InitError(String),
    /// 数据库错误
    #[error("db error: {0}")]
    DbError(rbatis::Error),
    /// 系统错误
    #[error("system error: {0}")]
    SystemError(anyhow::Error),
    /// 业务错误。改错误不会传递到前端，仅内部科可见。所有的业务逻辑处理均使用该错误。
    #[error("business error: {0}")]
    BusinessError(anyhow::Error),
    /// 提示类错误。该错误会传递到前端，前端显示给用户，且不会输出到日志。
    #[error("{0}")]
    MessageError(String),
    /// 提示类错误。该错误会传递到前端，并指定一个错误编码，前端显示给用户，且不会输出到日志。
    #[error("{1}")]
    MessageCodeError(i32, String),
}

#[macro_export]
macro_rules! message_error {
    ($e:expr) => {
        {
            use crate::common::result::AppError;
            AppError::MessageError($e.to_string())
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        {
            use crate::common::result::AppError;
            AppError::MessageError(format!($fmt, $($arg)*))
        }
    };
}

#[macro_export]
macro_rules! message_code_error {
    ($code:expr, $e:expr) => {
        {
            use crate::common::result::AppError;
            AppError::MessageCodeError($code, $e.to_string())
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        {
            use crate::common::result::AppError;
            AppError::MessageCodeError($code, format!($fmt, $($arg)*))
        }
    };
}

#[macro_export]
macro_rules! business_error {
    ($e:expr) => {
        {
            use crate::common::result::AppError;
            AppError::BusinessError(anyhow::anyhow!($e))
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        {
            use crate::common::result::AppError;
            AppError::BusinessError(anyhow::anyhow!(format!($fmt, $($arg)*)))
        }
    };
}

#[macro_export]
macro_rules! db_error {
    ($e:expr) => {{
        use crate::common::result::AppError;
        AppError::DbError($e)
    }};
}
