//! Tauri IPC command handlers. Each command is a thin wrapper
//! that delegates to [`AppState`](crate::state::AppState).

use std::sync::Arc;

use tauri::State;

use crate::state::AppState;
use crate::types::InitialState;
use homewizard_core::config::{AppSettings, SavedDevice};

/// Begin mDNS discovery, emitting `device_found` events as devices appear.
#[tauri::command]
pub async fn start_discovery(
    state: State<'_, Arc<AppState>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    state.start_discovery(app).await
}

/// Stop the running mDNS discovery daemon.
#[tauri::command]
pub async fn stop_discovery(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    state.stop_discovery().await
}

/// Persist the given device list to the config file.
#[tauri::command]
pub async fn save_devices(
    state: State<'_, Arc<AppState>>,
    devices: Vec<SavedDevice>,
) -> Result<(), String> {
    state.save_devices(devices).await
}

/// Return the saved device list from config.
#[tauri::command]
pub async fn get_saved_devices(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<SavedDevice>, String> {
    Ok(state.get_devices().await)
}

/// Return devices, settings, and cached telemetry in one call for instant startup.
#[tauri::command]
pub async fn get_initial_state(state: State<'_, Arc<AppState>>) -> Result<InitialState, String> {
    Ok(state.get_initial_state().await)
}

/// Return the current application settings.
#[tauri::command]
pub async fn get_settings(state: State<'_, Arc<AppState>>) -> Result<AppSettings, String> {
    Ok(state.get_settings().await)
}

/// Replace settings with the provided values and persist to disk.
#[tauri::command]
pub async fn update_settings(
    state: State<'_, Arc<AppState>>,
    settings: AppSettings,
) -> Result<(), String> {
    state.update_settings(settings).await
}

/// Return the current popover arrow x-offset in logical pixels.
#[tauri::command]
pub async fn get_arrow_offset(state: State<'_, Arc<AppState>>) -> Result<f64, String> {
    Ok(state.get_arrow_offset())
}
