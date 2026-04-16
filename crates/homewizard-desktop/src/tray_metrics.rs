//! Formats and displays live device metrics as independent menu bar items.
//!
//! Each configured metric becomes its own `NSStatusItem` via the `tray_title` crate.
//! Power metrics show a colored arrow: green ↑ for export, red ↓ for import.

use std::sync::Arc;

use tauri::{AppHandle, Manager};

use homewizard_api::types::MeasurementData;
use homewizard_core::config::{SavedDevice, TrayMetricConfig, TrayMetricField};
use tray_title::{Color, Span};

use crate::state::AppState;

const EXPORT_GREEN: Color = Color {
    r: 0.063,
    g: 0.725,
    b: 0.506,
};
const IMPORT_RED: Color = Color {
    r: 0.937,
    g: 0.267,
    b: 0.267,
};

type StyledSpan = (String, Option<Color>);

/// A single formatted menu bar item ready for display.
struct FormattedItem {
    id: String,
    name: String,
    spans: Vec<StyledSpan>,
}

/// Update all menu bar tray items to reflect the latest telemetry.
pub async fn update(app: &AppHandle, metrics: &[TrayMetricConfig], devices: &[SavedDevice]) {
    let items = {
        let state: tauri::State<Arc<AppState>> = app.state();
        let telemetry = state.telemetry.lock().await;

        metrics
            .iter()
            .map(|m| FormattedItem {
                id: format!("{}:{}", m.device_id, m.field.as_str()),
                name: resolve_label(m, devices),
                spans: format_value(telemetry.get_last_data(&m.device_id), m.field),
            })
            .collect::<Vec<_>>()
    };

    let _ = app.run_on_main_thread(move || render(items));
}

/// Send formatted items to the native menu bar via `tray_title`.
fn render(items: Vec<FormattedItem>) {
    let span_vecs: Vec<Vec<Span<'_>>> = items
        .iter()
        .map(|item| {
            item.spans
                .iter()
                .map(|(text, color)| Span {
                    text,
                    color: *color,
                })
                .collect()
        })
        .collect();

    let tray_items: Vec<tray_title::TrayItem<'_>> = items
        .iter()
        .zip(span_vecs.iter())
        .map(|(item, spans)| tray_title::TrayItem {
            id: &item.id,
            name: &item.name,
            value_spans: spans,
        })
        .collect();

    tray_title::update_items(&tray_items);
}

/// Resolve the display label: use custom label if set, otherwise device name.
fn resolve_label(metric: &TrayMetricConfig, devices: &[SavedDevice]) -> String {
    if !metric.label.is_empty() {
        return metric.label.clone();
    }
    devices
        .iter()
        .find(|d| d.id == metric.device_id)
        .map(|d| d.name.clone())
        .unwrap_or_else(|| "—".to_string())
}

/// Format a measurement value as styled text spans.
fn format_value(data: Option<&MeasurementData>, field: TrayMetricField) -> Vec<StyledSpan> {
    let Some(data) = data else {
        return vec![("—".to_string(), None)];
    };
    match field {
        TrayMetricField::ActivePower => {
            let w = data.active_power_w.unwrap_or(0.0);
            let (arrow, color) = if w < 0.0 {
                ("↑ ", EXPORT_GREEN)
            } else {
                ("↓ ", IMPORT_RED)
            };
            vec![
                (arrow.to_string(), Some(color)),
                (format!("{} W", w.abs().round() as i64), None),
            ]
        }
        TrayMetricField::TotalGas => {
            let v = data.total_gas_m3.unwrap_or(0.0);
            vec![(format!("{v:.1} m³"), None)]
        }
        TrayMetricField::WaterFlow => {
            let v = data.active_liter_lpm.unwrap_or(0.0);
            vec![(format!("{v:.1} L/min"), None)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_value_shows_dash_when_no_data() {
        let spans = format_value(None, TrayMetricField::ActivePower);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].0, "—");
        assert!(spans[0].1.is_none());
    }

    #[test]
    fn format_value_import_shows_red_down_arrow() {
        let data = MeasurementData {
            active_power_w: Some(500.0),
            ..Default::default()
        };
        let spans = format_value(Some(&data), TrayMetricField::ActivePower);
        assert_eq!(spans[0].0, "↓ ");
        assert!(spans[0].1.is_some());
        assert_eq!(spans[1].0, "500 W");
    }

    #[test]
    fn format_value_export_shows_green_up_arrow() {
        let data = MeasurementData {
            active_power_w: Some(-1200.0),
            ..Default::default()
        };
        let spans = format_value(Some(&data), TrayMetricField::ActivePower);
        assert_eq!(spans[0].0, "↑ ");
        assert_eq!(spans[1].0, "1200 W");
    }

    #[test]
    fn format_value_gas() {
        let data = MeasurementData {
            total_gas_m3: Some(8879.4),
            ..Default::default()
        };
        let spans = format_value(Some(&data), TrayMetricField::TotalGas);
        assert_eq!(spans[0].0, "8879.4 m³");
    }

    #[test]
    fn format_value_water() {
        let data = MeasurementData {
            active_liter_lpm: Some(3.2),
            ..Default::default()
        };
        let spans = format_value(Some(&data), TrayMetricField::WaterFlow);
        assert_eq!(spans[0].0, "3.2 L/min");
    }

    #[test]
    fn resolve_label_uses_custom_label() {
        let metric = TrayMetricConfig {
            device_id: "abc".to_string(),
            field: TrayMetricField::ActivePower,
            label: "Solar".to_string(),
        };
        assert_eq!(resolve_label(&metric, &[]), "Solar");
    }

    #[test]
    fn resolve_label_falls_back_to_device_name() {
        let metric = TrayMetricConfig {
            device_id: "abc".to_string(),
            field: TrayMetricField::ActivePower,
            label: String::new(),
        };
        let devices = vec![SavedDevice {
            id: "abc".to_string(),
            name: "P1 Meter".to_string(),
            product_type: "HWE-P1".to_string(),
            ip: "1.1.1.1".to_string(),
            port: 80,
        }];
        assert_eq!(resolve_label(&metric, &devices), "P1 Meter");
    }

    #[test]
    fn resolve_label_dash_for_unknown_device() {
        let metric = TrayMetricConfig {
            device_id: "unknown".to_string(),
            field: TrayMetricField::ActivePower,
            label: String::new(),
        };
        assert_eq!(resolve_label(&metric, &[]), "—");
    }
}
