use std::collections::HashSet;
use std::path::PathBuf;

use notify::{Event, EventKind, RecursiveMode, Watcher};
use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_notification::NotificationExt;

use crate::models::{Directory, WatchAction, WatchLogEntry};
use crate::state::AppState;
use crate::store::{read_json_array, write_json_array};
use crate::util::{generate_id, now_millis};

const DIRECTORIES_FILE: &str = "directories.json";
const WATCH_LOGS_FILE: &str = "watch-logs.json";

pub fn run_watchers_internal<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let directories: Vec<Directory> = read_json_array(app, DIRECTORIES_FILE, "directories");
    let active_directories: Vec<&Directory> = directories
        .iter()
        .filter(|directory| directory.is_active)
        .collect();
    let active_paths: HashSet<PathBuf> = active_directories
        .iter()
        .map(|directory| PathBuf::from(&directory.path))
        .collect();

    let state = app.state::<AppState>();
    let mut watchers = state
        .watchers
        .lock()
        .map_err(|_| "watcher state poisoned".to_string())?;

    watchers.retain(|path, _| active_paths.contains(path));

    for directory in active_directories {
        let path = PathBuf::from(&directory.path);

        if watchers.contains_key(&path) {
            continue;
        }

        let app_handle = app.clone();
        let directory = directory.clone();

        let mut watcher = notify::recommended_watcher(move |event: notify::Result<Event>| {
            if let Ok(event) = event {
                handle_watch_event(&app_handle, &directory, event);
            }
        })
        .map_err(|error| error.to_string())?;

        watcher
            .watch(&path, RecursiveMode::NonRecursive)
            .map_err(|error| error.to_string())?;

        watchers.insert(path, watcher);
    }

    Ok(())
}

pub fn stop_watchers_internal<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let state = app.state::<AppState>();
    let mut watchers = state
        .watchers
        .lock()
        .map_err(|_| "watcher state poisoned".to_string())?;

    watchers.clear();

    Ok(())
}

fn handle_watch_event<R: Runtime>(app: &AppHandle<R>, directory: &Directory, event: Event) {
    let action = match event.kind {
        EventKind::Create(_) => WatchAction::Add,
        EventKind::Modify(_) => WatchAction::Change,
        EventKind::Remove(_) => WatchAction::Unlink,
        _ => return,
    };

    for path in &event.paths {
        let entry = WatchLogEntry {
            id: generate_id(),
            directory_path: directory.path.clone(),
            directory_id: directory.id.clone(),
            file: path.display().to_string(),
            dms_endpoint: directory.dms_endpoint.clone(),
            is_sync: false,
            created_at: now_millis(),
            action,
        };

        let mut logs: Vec<WatchLogEntry> = read_json_array(app, WATCH_LOGS_FILE, "watchLogs");
        logs.push(entry.clone());

        if let Err(error) = write_json_array(app, WATCH_LOGS_FILE, "watchLogs", &logs) {
            log::error!("failed to write watch log: {error}");
        }

        let _ = app.emit("watch-log-created", &entry);

        if matches!(action, WatchAction::Add | WatchAction::Change) {
            let title = if action == WatchAction::Add {
                "File Added"
            } else {
                "File Changed"
            };

            let _ = app
                .notification()
                .builder()
                .title(title)
                .body(format!("{} in {}", entry.file, directory.path))
                .show();
        }
    }
}
