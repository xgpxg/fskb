use serde_json::{Map, Value};
use std::collections::HashMap;

mod summary_extract;

const BASE_URL: &str = "https://open.bigmodel.cn/api/paas/v4";
const MODEL_NAME: &str = "glm-4-flash-250414";
const API_KEY: &str = env!("ZP_API_KEY");

pub async fn extract_summary(content: &str) -> (String, String) {
    match summary_extract::extra(content, BASE_URL, MODEL_NAME, API_KEY).await {
        Ok(result) => match result {
            Some(result) => match serde_json::from_str::<HashMap<String, String>>(&result) {
                Ok(value) => (
                    value.get("title").unwrap_or(&"".to_string()).to_string(),
                    value.get("summary").unwrap_or(&"".to_string()).to_string(),
                ),
                Err(e) => {
                    log::error!("Extract summary json error: {}", e);
                    ("".to_string(), "".to_string())
                }
            },
            None => ("".to_string(), "".to_string()),
        },
        Err(e) => {
            log::error!("Extract summary error: {}", e);
            ("".to_string(), "".to_string())
        }
    }
}
