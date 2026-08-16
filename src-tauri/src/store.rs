use serde::{Serialize, de::DeserializeOwned};
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

pub fn read_json_array<R: Runtime, T: DeserializeOwned>(
    app: &AppHandle<R>,
    file: &str,
    key: &str,
) -> Vec<T> {
    let Ok(store) = app.store(file) else {
        return Vec::new();
    };

    store
        .get(key)
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default()
}

pub fn write_json_array<R: Runtime, T: Serialize>(
    app: &AppHandle<R>,
    file: &str,
    key: &str,
    items: &[T],
) -> Result<(), String> {
    let store = app.store(file).map_err(|error| error.to_string())?;
    let value = serde_json::to_value(items).map_err(|error| error.to_string())?;

    store.set(key, value);
    store.save().map_err(|error| error.to_string())?;

    Ok(())
}
