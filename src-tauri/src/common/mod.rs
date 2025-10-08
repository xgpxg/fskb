use rbatis::rbdc::DateTime;
use serde::{Deserialize, Deserializer, Serializer};

pub mod id;

#[allow(unused)]
pub(crate) mod pool;

#[allow(unused)]
pub mod res;

pub(crate) mod commands;
pub mod req;
#[allow(unused)]
pub mod result;

/// 序列化时间
///
/// 用于将rbatis映射的时间`DateTime`在传给前端时，序列化为字符串
pub fn serialize_datetime<S: Serializer>(
    time: &Option<DateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match time {
        None => serializer.serialize_none(),
        Some(dt) => serializer.serialize_str(&dt.format("YYYY-MM-DD hh:mm:ss")),
    }
}

pub fn deserialize_to_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::String(s) => Ok(Some(s)),
        _ => Ok(Some(value.to_string())),
    }
}
