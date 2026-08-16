use std::fs;
use std::path::Path;

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use reqwest::multipart::{Form, Part};
use tauri::{AppHandle, Emitter, Manager};
use url::Url;

use crate::config::{
    DMS_LOG_AND_NOTIFY_ENDPOINT_DEFAULT, DMS_UPLOAD_ENDPOINT_DEFAULT, LEGACY_DMS_UPLOAD_PATH,
};
use crate::models::{Directory, SyncLogEntry, UploadItem, UploadOutcome, WatchLogEntry};
use crate::state::AppState;
use crate::store::{read_json_array, write_json_array};
use crate::util::{generate_id, now_millis};

const DIRECTORIES_FILE: &str = "directories.json";
const WATCH_LOGS_FILE: &str = "watch-logs.json";
const SYNC_LOGS_FILE: &str = "sync-logs.json";

#[tauri::command]
pub async fn upload_document(app: AppHandle, item: UploadItem) -> Result<UploadOutcome, String> {
    let state = app.state::<AppState>();
    let _permit = state.upload_lock.lock().await;

    let directories: Vec<Directory> = read_json_array(&app, DIRECTORIES_FILE, "directories");
    let directory = directories
        .into_iter()
        .find(|directory| directory.path == item.directory_path)
        .ok_or_else(|| format!("Directory not found for path: {}", item.directory_path))?;

    let dms_endpoint = resolve_dms_endpoint(&[
        Some(directory.dms_endpoint.clone()),
        item.dms_endpoint.clone(),
    ])?;

    if get_upload_token(&dms_endpoint).is_none() {
        return Err(
            "DMS upload endpoint is missing the agent token query parameter: t".to_string(),
        );
    }

    let file_path = Path::new(&item.file);
    let bytes = fs::read(file_path).map_err(|error| error.to_string())?;
    let file_name = item
        .file_name
        .clone()
        .filter(|name| !name.is_empty())
        .or_else(|| {
            file_path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| format!("unknown-{}", now_millis()));

    let mut part = Part::bytes(bytes).file_name(file_name.clone());

    if let Some(file_type) = &item.file_type {
        part = part
            .mime_str(file_type)
            .map_err(|error| error.to_string())?;
    }

    let form = Form::new().part("file", part);
    let client = reqwest::Client::new();

    let response = client
        .post(&dms_endpoint)
        .multipart(form)
        .send()
        .await
        .map_err(|error| describe_connect_error(&error, &dms_endpoint))?;

    let status = response.status();
    let response_json: serde_json::Value = response.json().await.unwrap_or(serde_json::Value::Null);

    if !status.is_success() {
        return Err(describe_status_error(status, &response_json));
    }

    let document_ids = get_uploaded_document_ids(&response_json);

    create_document_log_and_notify(&client, &dms_endpoint, &document_ids).await;

    let sync_entry = SyncLogEntry {
        id: generate_id(),
        directory_path: item.directory_path.clone(),
        file: item.file.clone(),
        file_name,
        dms_endpoint: dms_endpoint.clone(),
        document_ids: document_ids.clone(),
        created_at: now_millis(),
    };

    let mut sync_logs: Vec<SyncLogEntry> = read_json_array(&app, SYNC_LOGS_FILE, "syncLogs");
    sync_logs.push(sync_entry.clone());
    write_json_array(&app, SYNC_LOGS_FILE, "syncLogs", &sync_logs)?;

    let mut watch_logs: Vec<WatchLogEntry> = read_json_array(&app, WATCH_LOGS_FILE, "watchLogs");

    for entry in &mut watch_logs {
        if entry.id == item.id {
            entry.is_sync = true;
            entry.dms_endpoint = dms_endpoint.clone();
        }
    }

    write_json_array(&app, WATCH_LOGS_FILE, "watchLogs", &watch_logs)?;

    let _ = app.emit("sync-log-created", &sync_entry);

    Ok(UploadOutcome {
        dms_endpoint,
        document_ids,
    })
}

fn describe_connect_error(error: &reqwest::Error, dms_endpoint: &str) -> String {
    if error.is_connect() {
        format!(
            "Cannot connect to DMS upload endpoint ({dms_endpoint}). Start the DMS server or update the configured endpoint."
        )
    } else {
        error.to_string()
    }
}

fn describe_status_error(status: reqwest::StatusCode, response: &serde_json::Value) -> String {
    let service_message = response
        .get("error")
        .and_then(|value| value.get("message"))
        .or_else(|| response.get("data").and_then(|value| value.get("message")))
        .or_else(|| response.get("message"))
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| response.to_string());

    format!("Upload failed with {status}: {service_message}")
}

async fn create_document_log_and_notify(
    client: &reqwest::Client,
    dms_endpoint: &str,
    document_ids: &[u64],
) {
    if document_ids.is_empty() {
        return;
    }

    let (application, user) = decode_agent_token_payload(dms_endpoint);

    let (Some(application), Some(user)) = (application, user) else {
        log::warn!("skipping document log creation because upload token payload is incomplete");
        return;
    };

    let mut request = client
        .post(DMS_LOG_AND_NOTIFY_ENDPOINT_DEFAULT)
        .json(&serde_json::json!({
            "appId": application,
            "content": "",
            "dateReceived": "",
            "documentIds": document_ids,
            "email": "",
            "subjectMatter": "",
            "userId": user,
        }));

    if let Some(token) = get_upload_token(dms_endpoint) {
        request = request.header("access-token", token);
    }

    if let Err(error) = request.send().await {
        log::warn!("document upload succeeded, but log creation failed: {error}");
    }
}

fn get_uploaded_document_ids(response: &serde_json::Value) -> Vec<u64> {
    let document_ids = response
        .get("data")
        .and_then(|data| data.get("documentIds").or_else(|| data.get("ids")))
        .or_else(|| response.get("documentIds"))
        .or_else(|| response.get("ids"));

    document_ids
        .and_then(|value| value.as_array())
        .map(|items| items.iter().filter_map(value_as_u64).collect())
        .unwrap_or_default()
}

fn value_as_u64(value: &serde_json::Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|text| text.parse().ok()))
}

fn decode_agent_token_payload(dms_endpoint: &str) -> (Option<u64>, Option<u64>) {
    let Some(token) = get_upload_token(dms_endpoint) else {
        return (None, None);
    };

    let Some(segment) = token.split('.').nth(1) else {
        return (None, None);
    };

    let Ok(decoded) = URL_SAFE_NO_PAD.decode(segment) else {
        return (None, None);
    };

    let Ok(payload) = serde_json::from_slice::<serde_json::Value>(&decoded) else {
        return (None, None);
    };

    let application = payload.get("application").and_then(value_as_u64);
    let user = payload.get("user").and_then(value_as_u64);

    (application, user)
}

fn parse_endpoint(value: &str) -> Result<Url, String> {
    Url::parse(value).map_err(|_| format!("DMS upload endpoint is invalid: {value}"))
}

fn get_query_param(url: &Url, key: &str) -> Option<String> {
    url.query_pairs()
        .find(|(name, _)| name == key)
        .map(|(_, value)| value.into_owned())
}

fn get_upload_token(endpoint: &str) -> Option<String> {
    let url = Url::parse(endpoint).ok()?;
    let token = get_query_param(&url, "t")?;

    (!token.trim().is_empty()).then_some(token)
}

fn resolve_dms_endpoint(candidates: &[Option<String>]) -> Result<String, String> {
    let endpoint_values: Vec<String> = candidates
        .iter()
        .filter_map(|value| value.clone())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect();

    let Some(endpoint) = endpoint_values.first() else {
        return Ok(DMS_UPLOAD_ENDPOINT_DEFAULT.to_string());
    };

    let is_legacy = endpoint.contains(LEGACY_DMS_UPLOAD_PATH);
    let target = if is_legacy {
        DMS_UPLOAD_ENDPOINT_DEFAULT
    } else {
        endpoint.as_str()
    };

    let mut upload_url = parse_endpoint(target)?;

    let token = endpoint_values
        .iter()
        .filter_map(|value| parse_endpoint(value).ok())
        .find_map(|url| get_query_param(&url, "t"));

    if let Some(token) = token
        && get_query_param(&upload_url, "t").is_none()
    {
        upload_url.query_pairs_mut().append_pair("t", &token);
    }

    Ok(upload_url.to_string())
}
