//! Adaptive polling loop for HomeWizard devices.
//!
//! Fetches measurement data from all configured devices in parallel,
//! records telemetry, emits frontend events, and updates tray metrics.
//! Waits for a `frontend_ready` signal before starting to ensure the
//! UI is ready to receive events.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tauri::{AppHandle, Emitter, Listener, Manager};
use tauri_plugin_notification::NotificationExt;
use tracing::{debug, info, warn};

use homewizard_api::client::HwClient;
use homewizard_api::types::MeasurementData;
use homewizard_core::config::SavedDevice;
use homewizard_core::telemetry::{TelemetrySnapshot, TelemetryStore};

use crate::events::warn_on_err;
use crate::state::AppState;
use crate::tray_metrics;
use crate::types::{DeviceEvent, TelemetryUpdate};

static POLLING_STARTED: AtomicBool = AtomicBool::new(false);

/// Result of a single device poll, computed while holding the telemetry lock
/// but emitted to the frontend after the lock is released.
#[derive(Debug)]
enum PollOutcome {
    Success(Box<SuccessOutcome>),
    Failure {
        id: String,
        device_name: String,
        became_offline: bool,
    },
}

#[derive(Debug)]
struct SuccessOutcome {
    id: String,
    update: TelemetryUpdate,
    became_online: bool,
}

/// Per-device polling state. Tracks the HTTP client, connection health,
/// and online/offline transitions.
struct DevicePoller {
    client: HwClient,
    ip: String,
    port: u16,
    snapshot: TelemetrySnapshot,
    /// Set to true after the first successful poll.
    /// Offline notifications are suppressed until this is true.
    seen_online: bool,
    is_offline: bool,
}

impl DevicePoller {
    fn new(device: &SavedDevice) -> Self {
        Self {
            client: HwClient::new(&device.ip, device.port),
            ip: device.ip.clone(),
            port: device.port,
            snapshot: TelemetrySnapshot::default(),
            seen_online: false,
            is_offline: false,
        }
    }

    /// Recreate the HTTP client if the device address changed (e.g. after re-discovery).
    fn update_endpoint(&mut self, device: &SavedDevice) {
        if self.ip != device.ip || self.port != device.port {
            self.client = HwClient::new(&device.ip, device.port);
            self.ip.clone_from(&device.ip);
            self.port = device.port;
        }
    }

    /// Handle a successful measurement. Updates the device's state snapshot
    /// and telemetry buffers, returning a [`PollOutcome::Success`] for the
    /// caller to emit after the telemetry lock is released.
    fn on_success(
        &mut self,
        id: String,
        data: MeasurementData,
        telemetry: &mut TelemetryStore,
    ) -> PollOutcome {
        let primary = data.active_power_w.or(data.active_liter_lpm).unwrap_or(0.0);
        self.snapshot.record_success(primary);

        let home_key = format!("{id}:home");
        record_telemetry(&id, &home_key, &data, telemetry, primary);

        let became_online = self.is_offline;
        self.seen_online = true;
        self.is_offline = false;

        let update = TelemetryUpdate {
            sparkline: sparkline(telemetry, &id),
            home_sparkline: sparkline(telemetry, &home_key),
            id: id.clone(),
            data,
            online: true,
        };

        PollOutcome::Success(Box::new(SuccessOutcome {
            id,
            update,
            became_online,
        }))
    }

    /// Handle a failed poll. Bumps the failure counter and flags the offline
    /// transition if the threshold has just been crossed. No events are
    /// emitted here — the caller emits after the lock is released.
    fn on_failure(&mut self, id: String, device_name: String) -> PollOutcome {
        self.snapshot.record_failure();

        let became_offline = self.snapshot.is_offline() && !self.is_offline && self.seen_online;
        if became_offline {
            self.is_offline = true;
        }

        PollOutcome::Failure {
            id,
            device_name,
            became_offline,
        }
    }
}

/// Push measurement values into the telemetry ring buffers.
/// For 3-phase devices, also tracks "home consumption" (sum of positive phases).
fn record_telemetry(
    id: &str,
    home_key: &str,
    data: &MeasurementData,
    telemetry: &mut TelemetryStore,
    primary: f64,
) {
    telemetry.buffer_mut(id).push(primary);

    if data.active_power_l2_w.is_some() {
        let home: f64 = [
            data.active_power_l1_w,
            data.active_power_l2_w,
            data.active_power_l3_w,
        ]
        .iter()
        .filter_map(|v| *v)
        .filter(|v| *v > 0.0)
        .sum();
        telemetry.buffer_mut(home_key).push(home);
    }

    telemetry.set_last_data(id, data.clone());
}

/// Spawn the polling loop. Safe to call multiple times — only the first
/// call starts the loop, subsequent calls are ignored.
pub fn start_polling(app: AppHandle) {
    if POLLING_STARTED.swap(true, Ordering::Relaxed) {
        warn!("Polling already started, ignoring duplicate call");
        return;
    }
    tauri::async_runtime::spawn(poll_loop(app));
}

async fn poll_loop(app: AppHandle) {
    let state: tauri::State<Arc<AppState>> = app.state();
    let mut pollers: HashMap<String, DevicePoller> = HashMap::new();
    let mut polls_since_save: u32 = 0;

    wait_for_frontend(&app).await;

    loop {
        let (devices, interval_ms, notify_offline, tray_metrics_config) = {
            let config = state.config.lock().await;
            (
                config.devices.clone(),
                config.settings.poll_interval_ms.max(500),
                config.settings.notify_offline,
                config.settings.tray_metrics.clone(),
            )
        };

        if !devices.is_empty() {
            sync_pollers(&mut pollers, &devices);

            let results = fetch_all(&pollers, &devices).await;

            let outcomes: Vec<PollOutcome> = {
                let mut telemetry = state.telemetry.lock().await;
                results
                    .into_iter()
                    .filter_map(|(id, result)| {
                        let device = devices.iter().find(|d| d.id == id)?;
                        let poller = pollers.get_mut(&id)?;
                        Some(match result {
                            Ok(data) => poller.on_success(id, data, &mut telemetry),
                            Err(e) => {
                                debug!("Poll failed for {id}: {e}");
                                poller.on_failure(id, device.name.clone())
                            }
                        })
                    })
                    .collect()
            };

            emit_outcomes(&app, outcomes, notify_offline);
            tray_metrics::update(&app, &tray_metrics_config, &devices).await;

            polls_since_save += 1;
            if should_save(polls_since_save, interval_ms) {
                polls_since_save = 0;
                warn_on_err("save telemetry", state.save_telemetry().await);
            }
        }

        tokio::select! {
            _ = tokio::time::sleep(Duration::from_millis(interval_ms)) => {}
            _ = state.settings_changed.notified() => {
                debug!("Settings changed, restarting poll cycle");
            }
        }
    }
}

/// Block until the frontend emits `frontend_ready`, or the timeout expires.
async fn wait_for_frontend(app: &AppHandle) {
    let (tx, rx) = oneshot_signal();
    app.once("frontend_ready", move |_| tx());
    wait_with_timeout(rx, Duration::from_secs(2)).await;
}

/// Create a one-shot signal pair: a trigger closure and a receiver future.
fn oneshot_signal() -> (impl FnOnce(), tokio::sync::oneshot::Receiver<()>) {
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let trigger = move || {
        let _ = tx.send(());
    };
    (trigger, rx)
}

async fn wait_with_timeout(rx: tokio::sync::oneshot::Receiver<()>, timeout: Duration) -> bool {
    tokio::time::timeout(timeout, rx).await.is_ok()
}

/// Persist telemetry to disk roughly every 30 seconds.
fn should_save(polls: u32, interval_ms: u64) -> bool {
    let save_every = (30_000 / interval_ms.max(100)) as u32;
    polls >= save_every.max(1)
}

/// Sync poller map with the current device list: remove stale entries,
/// create new ones, and update endpoints for existing devices.
fn sync_pollers(pollers: &mut HashMap<String, DevicePoller>, devices: &[SavedDevice]) {
    pollers.retain(|id, _| devices.iter().any(|d| d.id == *id));
    for device in devices {
        pollers
            .entry(device.id.clone())
            .and_modify(|p| p.update_endpoint(device))
            .or_insert_with(|| DevicePoller::new(device));
    }
}

/// Fire HTTP measurement requests to all devices concurrently.
async fn fetch_all(
    pollers: &HashMap<String, DevicePoller>,
    devices: &[SavedDevice],
) -> Vec<(
    String,
    Result<MeasurementData, homewizard_api::error::ApiError>,
)> {
    let futures: Vec<_> = devices
        .iter()
        .filter_map(|d| {
            let client = pollers.get(&d.id)?.client.clone();
            let id = d.id.clone();
            Some(async move { (id, client.measurement().await) })
        })
        .collect();

    futures::future::join_all(futures).await
}

fn sparkline(telemetry: &TelemetryStore, key: &str) -> Vec<f64> {
    telemetry
        .buffer(key)
        .map(|b| b.values())
        .unwrap_or_default()
}

/// Emit Tauri events and notifications for a batch of poll outcomes.
/// Runs *after* the telemetry lock is released so downstream listeners
/// never contend with the poll loop for shared state.
fn emit_outcomes(app: &AppHandle, outcomes: Vec<PollOutcome>, notify_offline: bool) {
    for outcome in outcomes {
        match outcome {
            PollOutcome::Success(success) => {
                let SuccessOutcome {
                    id,
                    update,
                    became_online,
                } = *success;
                if became_online {
                    info!("Device {id} back online");
                    warn_on_err(
                        "emit device_online",
                        app.emit("device_online", DeviceEvent { id: id.clone() }),
                    );
                }
                warn_on_err(
                    "emit telemetry_update",
                    app.emit("telemetry_update", update),
                );
            }
            PollOutcome::Failure {
                id,
                device_name,
                became_offline,
            } => {
                if became_offline {
                    warn!("Device {id} is offline");
                    warn_on_err(
                        "emit device_offline",
                        app.emit("device_offline", DeviceEvent { id: id.clone() }),
                    );
                    if notify_offline {
                        if let Err(e) = app
                            .notification()
                            .builder()
                            .title("Device Offline")
                            .body(format!("{device_name} is not responding"))
                            .show()
                        {
                            warn!("Failed to show offline notification: {e}");
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device(id: &str, ip: &str) -> SavedDevice {
        SavedDevice {
            id: id.to_string(),
            name: id.to_string(),
            product_type: "HWE-P1".to_string(),
            ip: ip.to_string(),
            port: 80,
        }
    }

    fn measurement(power: f64) -> MeasurementData {
        MeasurementData {
            active_power_w: Some(power),
            ..Default::default()
        }
    }

    fn three_phase_measurement(l1: f64, l2: f64, l3: f64) -> MeasurementData {
        MeasurementData {
            active_power_w: Some(l1 + l2 + l3),
            active_power_l1_w: Some(l1),
            active_power_l2_w: Some(l2),
            active_power_l3_w: Some(l3),
            ..Default::default()
        }
    }

    #[test]
    fn should_save_at_correct_interval() {
        // At 500ms interval, save every 60 polls (30s / 0.5s)
        assert!(!should_save(59, 500));
        assert!(should_save(60, 500));

        // At 1000ms, save every 30 polls
        assert!(!should_save(29, 1000));
        assert!(should_save(30, 1000));

        // At 100ms, save every 300 polls
        assert!(!should_save(299, 100));
        assert!(should_save(300, 100));

        // Always saves at least every poll for very long intervals
        assert!(should_save(1, 60000));
    }

    #[test]
    fn sync_pollers_adds_new_devices() {
        let mut pollers = HashMap::new();
        let devices = vec![device("a", "1.1.1.1"), device("b", "2.2.2.2")];

        sync_pollers(&mut pollers, &devices);

        assert_eq!(pollers.len(), 2);
        assert!(pollers.contains_key("a"));
        assert!(pollers.contains_key("b"));
    }

    #[test]
    fn sync_pollers_removes_stale_devices() {
        let mut pollers = HashMap::new();
        let all = vec![device("a", "1.1.1.1"), device("b", "2.2.2.2")];
        sync_pollers(&mut pollers, &all);

        let only_a = vec![device("a", "1.1.1.1")];
        sync_pollers(&mut pollers, &only_a);

        assert_eq!(pollers.len(), 1);
        assert!(pollers.contains_key("a"));
        assert!(!pollers.contains_key("b"));
    }

    #[test]
    fn sync_pollers_updates_changed_endpoint() {
        let mut pollers = HashMap::new();
        sync_pollers(&mut pollers, &[device("a", "1.1.1.1")]);
        assert_eq!(pollers["a"].ip, "1.1.1.1");

        sync_pollers(&mut pollers, &[device("a", "9.9.9.9")]);
        assert_eq!(pollers["a"].ip, "9.9.9.9");
    }

    #[test]
    fn sync_pollers_preserves_state_on_same_endpoint() {
        let mut pollers = HashMap::new();
        sync_pollers(&mut pollers, &[device("a", "1.1.1.1")]);
        pollers.get_mut("a").unwrap().seen_online = true;

        sync_pollers(&mut pollers, &[device("a", "1.1.1.1")]);
        assert!(pollers["a"].seen_online);
    }

    #[test]
    fn record_telemetry_stores_primary_value() {
        let mut store = TelemetryStore::default();
        let data = measurement(500.0);

        record_telemetry("dev1", "dev1:home", &data, &mut store, 500.0);

        assert_eq!(sparkline(&store, "dev1"), vec![500.0]);
        assert!(store.get_last_data("dev1").is_some());
    }

    #[test]
    fn record_telemetry_tracks_home_for_3phase() {
        let mut store = TelemetryStore::default();
        let data = three_phase_measurement(200.0, 150.0, -50.0);

        record_telemetry("dev1", "dev1:home", &data, &mut store, 300.0);

        // Home = sum of positive phases only: 200 + 150 = 350
        assert_eq!(sparkline(&store, "dev1:home"), vec![350.0]);
    }

    #[test]
    fn record_telemetry_skips_home_for_single_phase() {
        let mut store = TelemetryStore::default();
        let data = measurement(500.0);

        record_telemetry("dev1", "dev1:home", &data, &mut store, 500.0);

        assert!(sparkline(&store, "dev1:home").is_empty());
    }

    #[test]
    fn record_telemetry_accumulates_values() {
        let mut store = TelemetryStore::default();

        record_telemetry("d", "d:home", &measurement(100.0), &mut store, 100.0);
        record_telemetry("d", "d:home", &measurement(200.0), &mut store, 200.0);
        record_telemetry("d", "d:home", &measurement(300.0), &mut store, 300.0);

        assert_eq!(sparkline(&store, "d"), vec![100.0, 200.0, 300.0]);
        // last_data should be the most recent
        assert_eq!(
            store.get_last_data("d").unwrap().active_power_w,
            Some(300.0)
        );
    }

    #[test]
    fn new_poller_starts_not_online_not_offline() {
        let poller = DevicePoller::new(&device("a", "1.1.1.1"));
        assert!(!poller.seen_online);
        assert!(!poller.is_offline);
    }

    #[test]
    fn on_success_returns_success_outcome_with_sparkline() {
        let mut poller = DevicePoller::new(&device("a", "1.1.1.1"));
        let mut telemetry = TelemetryStore::default();

        let outcome = poller.on_success("a".to_string(), measurement(123.0), &mut telemetry);

        match outcome {
            PollOutcome::Success(success) => {
                assert_eq!(success.id, "a");
                assert_eq!(success.update.sparkline, vec![123.0]);
                assert!(success.update.online);
                assert!(
                    !success.became_online,
                    "first success should not flag transition"
                );
            }
            other => panic!("expected Success, got {other:?}"),
        }
    }

    #[test]
    fn on_success_after_offline_flags_online_transition() {
        let mut poller = DevicePoller::new(&device("a", "1.1.1.1"));
        let mut telemetry = TelemetryStore::default();

        poller.on_success("a".into(), measurement(1.0), &mut telemetry);
        for _ in 0..3 {
            poller.on_failure("a".into(), "A".into());
        }
        assert!(poller.is_offline);

        let outcome = poller.on_success("a".into(), measurement(2.0), &mut telemetry);
        match outcome {
            PollOutcome::Success(success) => assert!(success.became_online),
            other => panic!("expected Success, got {other:?}"),
        }
    }

    #[test]
    fn on_failure_before_first_success_never_flags_offline() {
        let mut poller = DevicePoller::new(&device("a", "1.1.1.1"));
        for _ in 0..10 {
            let outcome = poller.on_failure("a".into(), "A".into());
            match outcome {
                PollOutcome::Failure { became_offline, .. } => {
                    assert!(
                        !became_offline,
                        "should never transition to offline before seen_online"
                    );
                }
                other => panic!("expected Failure, got {other:?}"),
            }
        }
    }

    #[test]
    fn on_failure_flags_offline_exactly_once_at_threshold() {
        let mut poller = DevicePoller::new(&device("a", "1.1.1.1"));
        let mut telemetry = TelemetryStore::default();
        poller.on_success("a".into(), measurement(1.0), &mut telemetry);

        let mut transitions = 0;
        for _ in 0..5 {
            let outcome = poller.on_failure("a".into(), "A".into());
            if let PollOutcome::Failure { became_offline, .. } = outcome {
                if became_offline {
                    transitions += 1;
                }
            }
        }
        assert_eq!(
            transitions, 1,
            "offline transition should fire exactly once"
        );
    }

    #[test]
    fn should_save_handles_zero_interval() {
        // interval 0 is clamped to 100 internally, so save_every = 300
        assert!(!should_save(299, 0));
        assert!(should_save(300, 0));
    }

    #[tokio::test]
    async fn wait_completes_on_signal() {
        let (trigger, rx) = oneshot_signal();
        trigger();
        assert!(wait_with_timeout(rx, Duration::from_millis(100)).await);
    }

    #[tokio::test]
    async fn wait_times_out_without_signal() {
        let (_trigger, rx) = oneshot_signal();
        assert!(!wait_with_timeout(rx, Duration::from_millis(50)).await);
    }
}
