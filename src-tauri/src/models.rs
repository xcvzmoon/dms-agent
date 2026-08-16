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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadItem {
    pub id: String,
    pub file: String,
    pub directory_path: String,
    pub dms_endpoint: Option<String>,
    pub file_type: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncLogEntry {
    pub id: String,
    pub directory_path: String,
    pub file: String,
    pub file_name: String,
    pub dms_endpoint: String,
    pub document_ids: Vec<u64>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadOutcome {
    pub dms_endpoint: String,
    pub document_ids: Vec<u64>,
}
