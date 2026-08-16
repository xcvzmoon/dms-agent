use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    pub id: Option<String>,
    pub path: String,
    pub dms_endpoint: String,
    pub include_patterns: String,
    pub is_active: bool,
    pub last_sync: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum WatchAction {
    Add,
    Change,
    Unlink,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchLogEntry {
    pub id: String,
    pub directory_path: String,
    pub directory_id: Option<String>,
    pub file: String,
    pub dms_endpoint: String,
    pub is_sync: bool,
    pub created_at: u64,
    pub action: WatchAction,
}
