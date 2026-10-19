#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clipboard;
mod commands;
mod config;
mod error;
mod state;

use std::sync::Arc;

use tauri::{Manager, RunEvent, WindowEvent};

use state::AppState;

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let config_file = app.path().app_config_dir()?.join("config.json");
            let state = Arc::new(AppState::new(config_file));
            state::spawn_auto_lock(app.handle().clone(), Arc::clone(&state));
            app.manage(state);
            Ok(())
        })
        .on_window_event(|window, event| {
            // Lock (and wipe our clipboard content) when the window closes.
            if let WindowEvent::CloseRequested { .. } | WindowEvent::Destroyed = event {
                if let Some(state) = window.try_state::<Arc<AppState>>() {
                    state.lock_vault();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::vault_status,
            commands::choose_vault_file,
            commands::create_vault,
            commands::unlock_vault,
            commands::reload_vault,
            commands::change_master_password,
            commands::lock_vault,
            commands::list_entries,
            commands::get_entry,
            commands::reveal_password,
            commands::copy_password,
            commands::copy_username,
            commands::save_entry,
            commands::delete_entry,
            commands::generate_password,
            commands::export_vault,
            commands::touch_activity,
            commands::set_auto_lock_minutes,
        ])
        .build(tauri::generate_context!())
        .expect("error while building Burrow");

    app.run(|handle, event| {
        if let RunEvent::Exit = event {
            if let Some(state) = handle.try_state::<Arc<AppState>>() {
                state.lock_vault();
            }
        }
    });
}
