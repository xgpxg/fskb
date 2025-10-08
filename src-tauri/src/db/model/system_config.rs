use derive_builder::Builder;
use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
pub struct SystemConfig {
    pub config_key: Option<String>,
    pub config_value: Option<String>,
}

crud!(SystemConfig {});
