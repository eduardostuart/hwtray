//! Shared application state managed by Tauri.
//!
//! Wraps config, discovery, and telemetry behind async mutexes.
//! Provides high-level methods so callers never touch the locks directly.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use tauri::{AppHandle, Emitter};
use tokio::sync::{Mutex, Notify};

use homewizard_core::config::{AppConfig, AppSettings, SavedDevice};
use homewizard_core::discovery::DiscoveryService;
use homewizard_core::telemetry::TelemetryStore;

use crate::events::warn_on_err;
use crate::types::{DeviceFoundEvent, InitialState};

/// Default arrow x-offset (center of a 400px window).
const DEFAULT_ARROW_OFFSET: f32 = 200.0;

pub struct AppState {
    pub(crate) config: Mutex<AppConfig>,
    discovery: Mutex<DiscoveryService>,
    pub(crate) telemetry: Mutex<TelemetryStore>,
    config_path: PathBuf,
    telemetry_path: PathBuf,
    /// Arrow x-offset stored as f32 bits in an atomic for lock-free access.
    arrow_offset: AtomicU32,
    /// Mirrors `settings.always_on_top` for lock-free reads from the tray
    /// and window-creation code paths, which run on the main thread and
    /// cannot afford to wait on the config mutex.
    always_on_top: AtomicBool,
    /// Wakes the poll loop when settings change so new intervals apply immediately.
    pub(crate) settings_changed: Notify,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        config_path: PathBuf,
        telemetry: TelemetryStore,
        telemetry_path: PathBuf,
    ) -> Self {
        let always_on_top = AtomicBool::new(config.settings.always_on_top);
        Self {
            config: Mutex::new(config),
            discovery: Mutex::new(DiscoveryService::new()),
            telemetry: Mutex::new(telemetry),
            config_path,
            telemetry_path,
            arrow_offset: AtomicU32::new(DEFAULT_ARROW_OFFSET.to_bits()),
            always_on_top,
            settings_changed: Notify::new(),
        }
    }

    /// Start mDNS discovery and emit `device_found` events to the frontend.
    pub async fn start_discovery(&self, app: AppHandle) -> Result<(), String> {
        let mut discovery = self.discovery.lock().await;
        let mut rx = discovery.start()?;

        tauri::async_runtime::spawn(async move {
            while let Some(device) = rx.recv().await {
                warn_on_err(
                    "emit device_found",
                    app.emit("device_found", DeviceFoundEvent::from(device)),
                );
            }
        });

        Ok(())
    }

    /// Stop the running mDNS discovery.
    pub async fn stop_discovery(&self) -> Result<(), String> {
        self.discovery.lock().await.stop();
        Ok(())
    }

    pub async fn get_devices(&self) -> Vec<SavedDevice> {
        self.config.lock().await.devices.clone()
    }

    pub async fn save_devices(&self, devices: Vec<SavedDevice>) -> Result<(), String> {
        for d in &devices {
            d.validate()?;
        }
        self.config.lock().await.devices = devices;
        self.save_config().await
    }

    pub async fn rename_device(&self, id: String, name: String) -> Result<SavedDevice, String> {
        let updated = {
            let mut config = self.config.lock().await;
            config.rename_device(&id, &name)?.clone()
        };
        self.save_config().await?;
        Ok(updated)
    }

    pub async fn identify_device(&self, id: String) -> Result<(), String> {
        let (ip, port) = {
            let config = self.config.lock().await;
            let device = config
                .devices
                .iter()
                .find(|d| d.id == id)
                .ok_or_else(|| format!("device {id} not found"))?;
            (device.ip.clone(), device.port)
        };
        let client = homewizard_api::client::HwClient::new(&ip, port);
        client.identify().await.map_err(|e| e.to_string())
    }

    pub async fn get_settings(&self) -> AppSettings {
        self.config.lock().await.settings.clone()
    }

    pub async fn update_settings(&self, settings: AppSettings) -> Result<(), String> {
        self.always_on_top
            .store(settings.always_on_top, Ordering::Relaxed);
        self.config.lock().await.settings = settings;
        self.settings_changed.notify_one();
        self.save_config().await
    }

    /// Lock-free read of the currently-configured `always_on_top` setting.
    pub fn get_always_on_top(&self) -> bool {
        self.always_on_top.load(Ordering::Relaxed)
    }

    /// Build a snapshot of devices, settings, and cached telemetry for the frontend.
    pub async fn get_initial_state(&self) -> InitialState {
        let config = self.config.lock().await;
        let telemetry = self.telemetry.lock().await;
        InitialState::build(&config, &telemetry)
    }

    pub fn set_arrow_offset(&self, offset: f64) {
        self.arrow_offset
            .store((offset as f32).to_bits(), Ordering::Relaxed);
    }

    pub fn get_arrow_offset(&self) -> f64 {
        f32::from_bits(self.arrow_offset.load(Ordering::Relaxed)) as f64
    }

    pub async fn save_config(&self) -> Result<(), String> {
        let snapshot = self.config.lock().await.clone();
        snapshot
            .save_to(&self.config_path)
            .await
            .map_err(|e| format!("Failed to save config: {e}"))
    }

    pub async fn save_telemetry(&self) -> Result<(), String> {
        let snapshot = self.telemetry.lock().await.clone();
        snapshot
            .save_to(&self.telemetry_path)
            .await
            .map_err(|e| format!("Failed to save telemetry: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_state(config: AppConfig) -> (AppState, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let state = AppState::new(
            config,
            dir.path().join("config.json"),
            TelemetryStore::default(),
            dir.path().join("telemetry.json"),
        );
        (state, dir)
    }

    #[tokio::test]
    async fn always_on_top_reflects_initial_config() {
        let mut config = AppConfig::default();
        config.settings.always_on_top = true;
        let (state, _dir) = test_state(config);

        assert!(state.get_always_on_top());
    }

    #[tokio::test]
    async fn always_on_top_updates_without_needing_the_config_lock() {
        let (state, _dir) = test_state(AppConfig::default());
        assert!(!state.get_always_on_top());

        let mut settings = state.get_settings().await;
        settings.always_on_top = true;
        state.update_settings(settings).await.unwrap();

        // Holding the config lock must not block the read.
        let _guard = state.config.lock().await;
        assert!(state.get_always_on_top());
    }

    #[tokio::test]
    async fn save_devices_rejects_invalid_input_without_touching_config() {
        let (state, _dir) = test_state(AppConfig::default());
        let bad = vec![SavedDevice {
            id: "x".into(),
            name: "bad".into(),
            product_type: "HWE-P1".into(),
            ip: "not-an-ip".into(),
            port: 80,
        }];

        let before = state.get_devices().await;
        assert!(state.save_devices(bad).await.is_err());
        let after = state.get_devices().await;
        assert_eq!(
            before.len(),
            after.len(),
            "config must not change on rejected input"
        );
    }

    #[tokio::test]
    async fn rename_device_persists_new_name() {
        let mut config = AppConfig::default();
        config.devices.push(SavedDevice {
            id: "abc".into(),
            name: "Old".into(),
            product_type: "HWE-P1".into(),
            ip: "192.168.1.50".into(),
            port: 80,
        });
        let (state, _dir) = test_state(config);

        let updated = state
            .rename_device("abc".into(), "New".into())
            .await
            .unwrap();
        assert_eq!(updated.name, "New");

        let reloaded = AppConfig::load_from(&state.config_path).unwrap();
        assert_eq!(reloaded.devices[0].name, "New");
    }

    #[tokio::test]
    async fn rename_device_returns_error_on_empty_name() {
        let mut config = AppConfig::default();
        config.devices.push(SavedDevice {
            id: "abc".into(),
            name: "Old".into(),
            product_type: "HWE-P1".into(),
            ip: "192.168.1.50".into(),
            port: 80,
        });
        let (state, _dir) = test_state(config);

        assert!(state
            .rename_device("abc".into(), "  ".into())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn identify_device_fails_for_unknown_id() {
        let (state, _dir) = test_state(AppConfig::default());
        let err = state.identify_device("missing".into()).await.unwrap_err();
        assert!(err.contains("not found") || err.contains("missing"));
    }
}
