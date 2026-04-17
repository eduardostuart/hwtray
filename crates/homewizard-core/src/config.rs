//! Application configuration: saved devices, settings, and tray metrics.

use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{info, warn};

/// A saved device reference persisted in the config file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedDevice {
    /// Unique device identifier (typically the serial number).
    pub id: String,
    /// User-facing display name.
    pub name: String,
    /// Product type string (e.g. `"HWE-P1"`).
    pub product_type: String,
    /// Local network IP address.
    pub ip: String,
    /// HTTP port (usually 80).
    pub port: u16,
}

impl SavedDevice {
    /// Validate that the device has a parseable IP, a non-zero port, and
    /// non-empty id/name. Called at the IPC boundary to reject bad input
    /// before it reaches the poller.
    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("device id cannot be empty".into());
        }
        if self.name.trim().is_empty() {
            return Err(format!("device {} has empty name", self.id));
        }
        if self.port == 0 {
            return Err(format!("device {} has invalid port 0", self.id));
        }
        self.ip
            .parse::<std::net::IpAddr>()
            .map_err(|_| format!("device {} has invalid IP '{}'", self.id, self.ip))?;
        Ok(())
    }
}

/// Application settings persisted in the config file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Polling interval in milliseconds.
    pub poll_interval_ms: u64,
    /// UI theme (`"dark"` or `"light"`).
    pub theme: String,
    /// Send a notification when a device goes offline.
    pub notify_offline: bool,
    /// Launch the app at system login.
    pub launch_at_login: bool,
    /// Show the polling indicator in the UI.
    #[serde(default = "default_true")]
    pub show_poll_indicator: bool,
    /// User-defined ordering of devices on the dashboard.
    #[serde(default)]
    pub dashboard_order: Vec<String>,
    /// Keep the window above all other windows.
    #[serde(default)]
    pub always_on_top: bool,
    /// Device IDs hidden from the dashboard.
    #[serde(default)]
    pub hidden_devices: Vec<String>,
    /// Metrics to display in the macOS menu bar.
    #[serde(default)]
    pub tray_metrics: Vec<TrayMetricConfig>,
}

/// A metric to display in the macOS menu bar tray.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrayMetricConfig {
    /// Device ID to read from.
    pub device_id: String,
    /// Which field to display.
    pub field: TrayMetricField,
    /// Custom label shown above the value. Defaults to device name if empty.
    #[serde(default)]
    pub label: String,
}

/// Available fields for tray display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrayMetricField {
    ActivePower,
    TotalGas,
    WaterFlow,
}

impl TrayMetricField {
    /// Stable string representation matching serde's snake_case serialization.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ActivePower => "active_power",
            Self::TotalGas => "total_gas",
            Self::WaterFlow => "water_flow",
        }
    }
}

fn default_true() -> bool {
    true
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            poll_interval_ms: 500,
            theme: "dark".to_string(),
            notify_offline: true,
            launch_at_login: false,
            show_poll_indicator: true,
            always_on_top: false,
            dashboard_order: Vec::new(),
            hidden_devices: Vec::new(),
            tray_metrics: Vec::new(),
        }
    }
}

/// Full application config — serialized to JSON file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub devices: Vec<SavedDevice>,
    pub settings: AppSettings,
}

impl AppConfig {
    /// Load config from a file. Returns default config if file doesn't exist.
    pub fn load_from(path: &Path) -> Result<Self, std::io::Error> {
        if !path.exists() {
            info!(
                "Config file not found at {}, using defaults",
                path.display()
            );
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)?;
        match serde_json::from_str(&content) {
            Ok(config) => {
                info!("Loaded config from {}", path.display());
                Ok(config)
            }
            Err(e) => {
                warn!(
                    "Failed to parse config at {}: {e}. Using defaults.",
                    path.display()
                );
                Ok(Self::default())
            }
        }
    }

    /// Save config atomically (tempfile + rename), non-blocking.
    pub async fn save_to(&self, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        crate::fs_utils::atomic_write(path, json.as_bytes()).await?;
        info!("Saved config to {}", path.display());
        Ok(())
    }

    /// Rename a saved device by id. Returns a reference to the updated device.
    ///
    /// Validates that `new_name` is non-empty after trimming. Does not persist —
    /// caller is responsible for calling `save_to` afterwards.
    pub fn rename_device(&mut self, id: &str, new_name: &str) -> Result<&SavedDevice, String> {
        if new_name.trim().is_empty() {
            return Err(format!("device {id} cannot be renamed to empty name"));
        }
        let device = self
            .devices
            .iter_mut()
            .find(|d| d.id == id)
            .ok_or_else(|| format!("device {id} not found"))?;
        device.name = new_name.to_string();
        Ok(device)
    }

    /// Default config file path on macOS.
    pub fn default_path() -> std::path::PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        path.push("com.homewizard-unofficial");
        path.push("config.json");
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn valid_device() -> SavedDevice {
        SavedDevice {
            id: "aabbccddeeff".to_string(),
            name: "P1 Meter".to_string(),
            product_type: "HWE-P1".to_string(),
            ip: "192.168.1.50".to_string(),
            port: 80,
        }
    }

    #[test]
    fn default_config_has_empty_devices() {
        let config = AppConfig::default();
        assert!(config.devices.is_empty());
        assert_eq!(config.settings.poll_interval_ms, 500);
    }

    #[test]
    fn valid_device_passes_validation() {
        assert!(valid_device().validate().is_ok());
    }

    #[test]
    fn empty_id_fails_validation() {
        let d = SavedDevice {
            id: String::new(),
            ..valid_device()
        };
        assert!(d.validate().is_err());
    }

    #[test]
    fn whitespace_id_fails_validation() {
        let d = SavedDevice {
            id: "   ".to_string(),
            ..valid_device()
        };
        assert!(d.validate().is_err());
    }

    #[test]
    fn empty_name_fails_validation() {
        let d = SavedDevice {
            name: String::new(),
            ..valid_device()
        };
        assert!(d.validate().is_err());
    }

    #[test]
    fn malformed_ip_fails_validation() {
        let d = SavedDevice {
            ip: "not-an-ip".to_string(),
            ..valid_device()
        };
        assert!(d.validate().is_err());
    }

    #[test]
    fn ipv6_passes_validation() {
        let d = SavedDevice {
            ip: "fe80::1".to_string(),
            ..valid_device()
        };
        assert!(d.validate().is_ok());
    }

    #[test]
    fn port_zero_fails_validation() {
        let d = SavedDevice {
            port: 0,
            ..valid_device()
        };
        assert!(d.validate().is_err());
    }

    #[test]
    fn serialize_and_deserialize_roundtrip() {
        let config = AppConfig {
            devices: vec![SavedDevice {
                id: "aabbccddeeff".to_string(),
                name: "P1 Meter".to_string(),
                product_type: "HWE-P1".to_string(),
                ip: "192.168.1.50".to_string(),
                port: 80,
            }],
            settings: AppSettings {
                poll_interval_ms: 5000,
                tray_metrics: Vec::new(),
                ..Default::default()
            },
        };

        let json = serde_json::to_string_pretty(&config).unwrap();
        let restored: AppConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.devices.len(), 1);
        assert_eq!(restored.devices[0].id, "aabbccddeeff");
        assert_eq!(restored.settings.poll_interval_ms, 5000);
    }

    #[test]
    fn load_from_file() {
        let mut file = NamedTempFile::new().unwrap();
        let json = r#"{"devices":[],"settings":{"poll_interval_ms":60,"theme":"light","notify_offline":false,"launch_at_login":true}}"#;
        file.write_all(json.as_bytes()).unwrap();

        let config = AppConfig::load_from(file.path()).unwrap();
        assert_eq!(config.settings.poll_interval_ms, 60);
        assert!(config.settings.launch_at_login);
    }

    #[test]
    fn load_missing_file_returns_default() {
        let config =
            AppConfig::load_from(std::path::Path::new("/nonexistent/path/config.json")).unwrap();
        assert!(config.devices.is_empty());
    }

    #[tokio::test]
    async fn save_and_reload() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");

        let mut config = AppConfig::default();
        config.devices.push(SavedDevice {
            id: "112233".to_string(),
            name: "Socket".to_string(),
            product_type: "HWE-SKT".to_string(),
            ip: "10.0.0.5".to_string(),
            port: 80,
        });

        config.save_to(&path).await.unwrap();
        let loaded = AppConfig::load_from(&path).unwrap();
        assert_eq!(loaded.devices[0].name, "Socket");
    }

    #[tokio::test]
    async fn save_leaves_no_tmp_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");

        AppConfig::default().save_to(&path).await.unwrap();

        let leftover: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.ends_with(".tmp"))
            .collect();
        assert!(
            leftover.is_empty(),
            "expected no .tmp files, found: {leftover:?}"
        );
    }

    #[tokio::test]
    async fn save_overwrites_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");

        let mut first = AppConfig::default();
        first.settings.poll_interval_ms = 1000;
        first.save_to(&path).await.unwrap();

        let mut second = AppConfig::default();
        second.settings.poll_interval_ms = 2000;
        second.save_to(&path).await.unwrap();

        let loaded = AppConfig::load_from(&path).unwrap();
        assert_eq!(loaded.settings.poll_interval_ms, 2000);
    }

    #[test]
    fn tray_metrics_roundtrip() {
        let mut config = AppConfig::default();
        config.settings.tray_metrics = vec![
            TrayMetricConfig {
                device_id: "abc".to_string(),
                field: TrayMetricField::ActivePower,
                label: "Power".to_string(),
            },
            TrayMetricConfig {
                device_id: "def".to_string(),
                field: TrayMetricField::TotalGas,
                label: String::new(),
            },
        ];

        let json = serde_json::to_string(&config).unwrap();
        let restored: AppConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.settings.tray_metrics.len(), 2);
        assert_eq!(
            restored.settings.tray_metrics[0].field,
            TrayMetricField::ActivePower
        );
        assert_eq!(restored.settings.tray_metrics[0].label, "Power");
        assert!(restored.settings.tray_metrics[1].label.is_empty());
    }

    #[test]
    fn tray_metric_field_as_str_matches_serde() {
        for field in [
            TrayMetricField::ActivePower,
            TrayMetricField::TotalGas,
            TrayMetricField::WaterFlow,
        ] {
            let metric = TrayMetricConfig {
                device_id: "x".to_string(),
                field,
                label: String::new(),
            };
            let json = serde_json::to_string(&metric).unwrap();
            assert!(json.contains(field.as_str()));
        }
    }

    #[test]
    fn missing_tray_metrics_defaults_to_empty() {
        let json = r#"{"devices":[],"settings":{"poll_interval_ms":500,"theme":"dark","notify_offline":true,"launch_at_login":false}}"#;
        let config: AppConfig = serde_json::from_str(json).unwrap();
        assert!(config.settings.tray_metrics.is_empty());
    }

    #[test]
    fn missing_label_defaults_to_empty_string() {
        let json = r#"{"device_id":"abc","field":"active_power"}"#;
        let metric: TrayMetricConfig = serde_json::from_str(json).unwrap();
        assert!(metric.label.is_empty());
    }

    #[test]
    fn invalid_metric_field_rejected() {
        let json = r#"{"device_id":"x","field":"invalid_field","label":""}"#;
        assert!(serde_json::from_str::<TrayMetricConfig>(json).is_err());
    }

    #[test]
    fn rename_device_updates_name() {
        let mut config = AppConfig::default();
        config.devices.push(valid_device());
        let updated = config
            .rename_device("aabbccddeeff", "Sala")
            .unwrap()
            .clone();
        assert_eq!(updated.name, "Sala");
        assert_eq!(config.devices[0].name, "Sala");
    }

    #[test]
    fn rename_device_rejects_empty_name() {
        let mut config = AppConfig::default();
        config.devices.push(valid_device());
        assert!(config.rename_device("aabbccddeeff", "   ").is_err());
        assert_eq!(config.devices[0].name, "P1 Meter");
    }

    #[test]
    fn rename_device_rejects_unknown_id() {
        let mut config = AppConfig::default();
        config.devices.push(valid_device());
        assert!(config.rename_device("missing", "X").is_err());
    }

    #[test]
    fn rename_device_preserves_other_devices_and_order() {
        let mut config = AppConfig::default();
        config.devices.push(valid_device());
        config.devices.push(SavedDevice {
            id: "ffeeddccbbaa".to_string(),
            name: "Socket".to_string(),
            product_type: "HWE-SKT".to_string(),
            ip: "192.168.1.51".to_string(),
            port: 80,
        });
        config.rename_device("aabbccddeeff", "Novo").unwrap();
        assert_eq!(config.devices.len(), 2);
        assert_eq!(config.devices[0].id, "aabbccddeeff");
        assert_eq!(config.devices[0].name, "Novo");
        assert_eq!(config.devices[1].name, "Socket");
    }
}
