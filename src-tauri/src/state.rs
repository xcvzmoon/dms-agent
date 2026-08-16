use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use notify::RecommendedWatcher;

#[derive(Default)]
pub struct AppState {
    pub watchers: Mutex<HashMap<PathBuf, RecommendedWatcher>>,
    pub upload_lock: tokio::sync::Mutex<()>,
}
