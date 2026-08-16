pub mod watch;

use tauri::AppHandle;

#[tauri::command]
pub fn start_watchers(app: AppHandle) -> Result<(), String> {
    watch::run_watchers_internal(&app)
}

#[tauri::command]
pub fn stop_watchers(app: AppHandle) -> Result<(), String> {
    watch::stop_watchers_internal(&app)
}
