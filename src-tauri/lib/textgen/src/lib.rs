use serde::{Deserialize, Serialize};

pub mod share_card_generate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextGenModelConfig {
    pub base_url: String,
    pub model_name: String,
    pub api_key: Option<String>,
}
