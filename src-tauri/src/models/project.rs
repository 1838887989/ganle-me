use serde::{Deserialize, Serialize};

/// 项目
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: String,
    pub icon: String,
    pub archived: bool,
    pub created_at: String,
}
