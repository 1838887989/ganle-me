use serde::{Deserialize, Serialize};

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub is_dark: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self { is_dark: false }
    }
}
