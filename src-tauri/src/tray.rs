use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::commands::watch::{run_watchers_internal, stop_watchers_internal};
use crate::state::AppState;

pub fn build_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let toggle_watchers =
        MenuItem::with_id(app, "toggle-watchers", "Run Watchers", true, None::<&str>)?;
    let show_app = MenuItem::with_id(app, "show-app", "Show App", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&toggle_watchers, &show_app, &quit])?;

    let mut builder = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(true);

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder
        .on_menu_event(|app, event| match event.id().as_ref() {
            "toggle-watchers" => toggle_watchers_handler(app),
            "show-app" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)?;

    Ok(())
}

fn toggle_watchers_handler<R: Runtime>(app: &AppHandle<R>) {
    let state = app.state::<AppState>();
    let is_running = match state.watchers.lock() {
        Ok(watchers) => !watchers.is_empty(),
        Err(_) => return,
    };

    let result = if is_running {
        stop_watchers_internal(app)
    } else {
        run_watchers_internal(app)
    };

    if let Err(error) = result {
        log::error!("failed to toggle watchers from tray: {error}");
        return;
    }

    let _ = app.emit("watchers-status-changed", !is_running);
}
