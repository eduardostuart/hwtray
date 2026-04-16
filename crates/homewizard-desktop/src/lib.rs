//! HomeWizard desktop application entry point.
//!
//! Bootstraps the Tauri app with plugins, IPC commands, system tray, and polling loop.

mod commands;
mod events;
mod poller;
mod state;
mod tray;
mod tray_metrics;
mod types;
mod windows;

use std::sync::Arc;

use homewizard_core::config::AppConfig;
use homewizard_core::telemetry::TelemetryStore;

/// Bootstrap and run the Tauri desktop application.
pub fn run() {
    let app_state = Arc::new(init_state());

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_positioner::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::start_discovery,
            commands::stop_discovery,
            commands::save_devices,
            commands::get_saved_devices,
            commands::get_initial_state,
            commands::get_settings,
            commands::update_settings,
            commands::get_arrow_offset,
        ])
        .setup(on_setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Called once after the Tauri app is initialized.
fn on_setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    tray::setup(app)?;
    poller::start_polling(app.handle().clone());
    Ok(())
}

/// Load persisted config and telemetry from disk.
fn init_state() -> state::AppState {
    let config_path = AppConfig::default_path();
    let config = AppConfig::load_from(&config_path).unwrap_or_default();

    let telemetry_path = TelemetryStore::default_path();
    let telemetry = TelemetryStore::load_from(&telemetry_path);

    state::AppState::new(config, config_path, telemetry, telemetry_path)
}
