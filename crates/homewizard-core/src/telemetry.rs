//! Telemetry storage: ring buffers for sparkline data, device health
//! tracking, and disk persistence.

use std::collections::{HashMap, VecDeque};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tracing::{info, warn};

const OFFLINE_THRESHOLD: u32 = 3;
const RETENTION_SECS: u64 = 1800; // 30 minutes

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// A single timestamped reading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reading {
    /// Unix timestamp in seconds.
    pub ts: u64,
    /// Measured value (e.g. watts or liters per minute).
    pub value: f64,
}

/// Circular buffer for sparkline data, evicts readings older than 30 minutes.
/// Serializable so it can be persisted to disk.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelemetryBuffer {
    entries: VecDeque<Reading>,
}

impl TelemetryBuffer {
    /// Create a new empty buffer.
    pub fn new(_retention: std::time::Duration) -> Self {
        Self::default()
    }

    /// Add a reading at the current time.
    pub fn push(&mut self, value: f64) {
        self.entries.push_back(Reading {
            ts: now_secs(),
            value,
        });
        self.evict_stale();
    }

    /// Remove readings older than 30 minutes.
    pub fn evict_stale(&mut self) {
        let cutoff = now_secs().saturating_sub(RETENTION_SECS);
        while let Some(front) = self.entries.front() {
            if front.ts < cutoff {
                self.entries.pop_front();
            } else {
                break;
            }
        }
    }

    /// Get all values in chronological order (for sparkline rendering).
    pub fn values(&self) -> Vec<f64> {
        self.entries.iter().map(|r| r.value).collect()
    }

    /// Number of readings in the buffer.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// True if the buffer contains no readings.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Tracks online/offline state and last reading for a device.
#[derive(Debug, Default)]
pub struct TelemetrySnapshot {
    /// Number of consecutive failed poll attempts.
    pub consecutive_failures: u32,
    /// Most recent successful power reading in watts.
    pub last_power_w: Option<f64>,
}

impl TelemetrySnapshot {
    /// Record a successful reading.
    pub fn record_success(&mut self, power_w: f64) {
        self.consecutive_failures = 0;
        self.last_power_w = Some(power_w);
    }

    /// Record a failed poll attempt.
    pub fn record_failure(&mut self) {
        self.consecutive_failures += 1;
    }

    /// Device is offline after 3 consecutive failures.
    pub fn is_offline(&self) -> bool {
        self.consecutive_failures >= OFFLINE_THRESHOLD
    }
}

/// Persisted telemetry data for all devices.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelemetryStore {
    pub buffers: HashMap<String, TelemetryBuffer>,
    #[serde(default)]
    pub last_data: HashMap<String, homewizard_api::types::MeasurementData>,
}

impl TelemetryStore {
    /// Load from file. Returns empty store if file doesn't exist or is corrupt.
    pub fn load_from(path: &Path) -> Self {
        if !path.exists() {
            return Self::default();
        }
        match std::fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<Self>(&content) {
                Ok(mut store) => {
                    // Evict stale entries on load
                    for buf in store.buffers.values_mut() {
                        buf.evict_stale();
                    }
                    info!("Loaded telemetry from {}", path.display());
                    store
                }
                Err(e) => {
                    warn!("Failed to parse telemetry at {}: {e}", path.display());
                    Self::default()
                }
            },
            Err(e) => {
                warn!("Failed to read telemetry at {}: {e}", path.display());
                Self::default()
            }
        }
    }

    /// Save to file atomically (tempfile + rename).
    ///
    /// Uses async I/O so it doesn't block the tokio runtime, and writes to
    /// a sibling `.tmp` file followed by an atomic rename so a crash mid-write
    /// leaves either the old file intact or the new file complete.
    pub async fn save_to(&self, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(self).map_err(std::io::Error::other)?;
        crate::fs_utils::atomic_write(path, json.as_bytes()).await
    }

    /// Get or create buffer for a device.
    pub fn buffer_mut(&mut self, device_id: &str) -> &mut TelemetryBuffer {
        self.buffers.entry(device_id.to_string()).or_default()
    }

    /// Get buffer for a device (read-only).
    pub fn buffer(&self, device_id: &str) -> Option<&TelemetryBuffer> {
        self.buffers.get(device_id)
    }

    /// Cache the latest measurement data for a device.
    pub fn set_last_data(&mut self, device_id: &str, data: homewizard_api::types::MeasurementData) {
        self.last_data.insert(device_id.to_string(), data);
    }

    /// Get the cached measurement data for a device.
    pub fn get_last_data(
        &self,
        device_id: &str,
    ) -> Option<&homewizard_api::types::MeasurementData> {
        self.last_data.get(device_id)
    }

    /// Default telemetry file path.
    pub fn default_path() -> std::path::PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        path.push("com.homewizard-unofficial");
        path.push("telemetry.json");
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_stores_and_returns_values() {
        let mut buffer = TelemetryBuffer::default();
        for i in 0..10 {
            buffer.push(i as f64 * 10.0);
        }
        let values = buffer.values();
        assert_eq!(values.len(), 10);
        assert_eq!(values[0], 0.0);
        assert_eq!(values[9], 90.0);
    }

    #[test]
    fn snapshot_goes_offline_after_3_failures() {
        let mut snapshot = TelemetrySnapshot::default();
        assert!(!snapshot.is_offline());
        snapshot.record_failure();
        snapshot.record_failure();
        assert!(!snapshot.is_offline());
        snapshot.record_failure();
        assert!(snapshot.is_offline());
    }

    #[test]
    fn snapshot_resets_on_success() {
        let mut snapshot = TelemetrySnapshot::default();
        snapshot.record_failure();
        snapshot.record_failure();
        snapshot.record_success(42.0);
        assert!(!snapshot.is_offline());
        assert_eq!(snapshot.consecutive_failures, 0);
    }

    #[tokio::test]
    async fn store_save_and_load() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("telemetry.json");

        let mut store = TelemetryStore::default();
        store.buffer_mut("device1").push(100.0);
        store.buffer_mut("device1").push(200.0);
        store.buffer_mut("device2").push(50.0);

        store.save_to(&path).await.unwrap();

        let loaded = TelemetryStore::load_from(&path);
        assert_eq!(loaded.buffer("device1").unwrap().values().len(), 2);
        assert_eq!(loaded.buffer("device2").unwrap().values().len(), 1);
        assert!(loaded.buffer("device3").is_none());
    }

    #[test]
    fn store_load_missing_returns_empty() {
        let store = TelemetryStore::load_from(std::path::Path::new("/nonexistent/telemetry.json"));
        assert!(store.buffers.is_empty());
    }

    #[tokio::test]
    async fn save_to_leaves_no_tmp_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("telemetry.json");

        let mut store = TelemetryStore::default();
        store.buffer_mut("a").push(1.0);
        store.save_to(&path).await.unwrap();

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
    async fn save_to_overwrites_existing_file_atomically() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("telemetry.json");

        let mut first = TelemetryStore::default();
        first.buffer_mut("a").push(1.0);
        first.save_to(&path).await.unwrap();

        let mut second = TelemetryStore::default();
        second.buffer_mut("b").push(2.0);
        second.save_to(&path).await.unwrap();

        let loaded = TelemetryStore::load_from(&path);
        assert!(loaded.buffer("a").is_none());
        assert_eq!(loaded.buffer("b").unwrap().values(), vec![2.0]);
    }
}
