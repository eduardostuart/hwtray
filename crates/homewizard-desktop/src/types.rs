//! Typed event payloads emitted to the frontend via Tauri events.

use std::collections::HashMap;

use serde::Serialize;

use homewizard_api::types::MeasurementData;
use homewizard_core::config::{AppConfig, AppSettings, SavedDevice};
use homewizard_core::discovery::DiscoveredDevice;
use homewizard_core::telemetry::TelemetryStore;

/// Snapshot of the full app state, sent to the frontend on startup.
#[derive(Serialize)]
pub struct InitialState {
    pub devices: Vec<SavedDevice>,
    pub settings: AppSettings,
    pub cached_sparklines: HashMap<String, Vec<f64>>,
    pub cached_home_sparklines: HashMap<String, Vec<f64>>,
    pub cached_data: HashMap<String, MeasurementData>,
}

impl InitialState {
    /// Build from current config and telemetry, collecting cached data per device.
    pub fn build(config: &AppConfig, telemetry: &TelemetryStore) -> Self {
        let mut cached_sparklines = HashMap::new();
        let mut cached_home_sparklines = HashMap::new();
        let mut cached_data = HashMap::new();

        for device in &config.devices {
            if let Some(buf) = telemetry.buffer(&device.id) {
                let values = buf.values();
                if !values.is_empty() {
                    cached_sparklines.insert(device.id.clone(), values);
                }
            }
            let home_key = format!("{}:home", device.id);
            if let Some(buf) = telemetry.buffer(&home_key) {
                let values = buf.values();
                if !values.is_empty() {
                    cached_home_sparklines.insert(device.id.clone(), values);
                }
            }
            if let Some(data) = telemetry.get_last_data(&device.id) {
                cached_data.insert(device.id.clone(), data.clone());
            }
        }

        Self {
            devices: config.devices.clone(),
            settings: config.settings.clone(),
            cached_sparklines,
            cached_home_sparklines,
            cached_data,
        }
    }
}

/// Real-time measurement update for a single device.
#[derive(Clone, Debug, Serialize)]
pub struct TelemetryUpdate {
    pub id: String,
    pub data: MeasurementData,
    pub sparkline: Vec<f64>,
    pub home_sparkline: Vec<f64>,
    pub online: bool,
}

/// Simple device ID event (online/offline transitions).
#[derive(Clone, Serialize)]
pub struct DeviceEvent {
    pub id: String,
}

/// Device discovered via mDNS, emitted to the frontend.
#[derive(Clone, Serialize)]
pub struct DeviceFoundEvent {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub serial: String,
    pub product_type: String,
    pub api_enabled: bool,
}

impl From<DiscoveredDevice> for DeviceFoundEvent {
    fn from(d: DiscoveredDevice) -> Self {
        Self {
            id: d.id().to_string(),
            name: d.name,
            ip: d.ip,
            port: d.port,
            serial: d.serial,
            product_type: d.product_type,
            api_enabled: d.api_enabled,
        }
    }
}
