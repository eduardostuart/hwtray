//! Data types returned by the HomeWizard local API.

use serde::{Deserialize, Serialize};

/// HomeWizard device product types as returned by the local API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductType {
    /// P1 meter (`HWE-P1`) -- reads the Dutch smart meter P1 port.
    #[serde(rename = "HWE-P1")]
    P1Meter,
    /// Energy Socket (`HWE-SKT`) -- smart plug with power monitoring.
    #[serde(rename = "HWE-SKT")]
    EnergySocket,
    /// Water Meter (`HWE-WTR`) -- measures water flow and consumption.
    #[serde(rename = "HWE-WTR")]
    Watermeter,
    /// kWh Meter 1-phase (`HWE-KWH1`, alias `SDM230-wifi`).
    #[serde(alias = "SDM230-wifi")]
    #[serde(rename = "HWE-KWH1")]
    KwhMeter1Phase,
    /// kWh Meter 3-phase (`HWE-KWH3`, alias `SDM630-wifi`).
    #[serde(alias = "SDM630-wifi")]
    #[serde(rename = "HWE-KWH3")]
    KwhMeter3Phase,
}

/// Response from `GET /api` -- basic device identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// The product type identifier (e.g. `HWE-P1`).
    pub product_type: ProductType,
    /// Human-readable product name (e.g. "P1 Meter").
    pub product_name: String,
    /// Device serial number (unique per device).
    pub serial: String,
    /// Current firmware version.
    pub firmware_version: String,
    /// API version string (e.g. "v1").
    pub api_version: String,
}

/// Response from `GET /api/v1/data` -- real-time measurement data.
///
/// All fields are optional because the API only returns fields applicable to
/// the specific device type (P1, Socket, Water, kWh meter).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeasurementData {
    // WiFi
    /// SSID of the Wi-Fi network the device is connected to.
    pub wifi_ssid: Option<String>,
    /// Wi-Fi signal strength in percent (0–100).
    pub wifi_strength: Option<i32>,

    // Power
    /// Total active power in watts (sum of all phases).
    pub active_power_w: Option<f64>,
    /// Active power on phase 1 in watts.
    pub active_power_l1_w: Option<f64>,
    /// Active power on phase 2 in watts.
    pub active_power_l2_w: Option<f64>,
    /// Active power on phase 3 in watts.
    pub active_power_l3_w: Option<f64>,

    // Energy
    /// Total energy imported in kWh (all tariffs combined).
    pub total_power_import_kwh: Option<f64>,
    /// Energy imported on tariff 1 in kWh.
    pub total_power_import_t1_kwh: Option<f64>,
    /// Energy imported on tariff 2 in kWh.
    pub total_power_import_t2_kwh: Option<f64>,
    /// Energy imported on tariff 3 in kWh.
    pub total_power_import_t3_kwh: Option<f64>,
    /// Energy imported on tariff 4 in kWh.
    pub total_power_import_t4_kwh: Option<f64>,
    /// Total energy exported in kWh (all tariffs combined).
    pub total_power_export_kwh: Option<f64>,
    /// Energy exported on tariff 1 in kWh.
    pub total_power_export_t1_kwh: Option<f64>,
    /// Energy exported on tariff 2 in kWh.
    pub total_power_export_t2_kwh: Option<f64>,
    /// Energy exported on tariff 3 in kWh.
    pub total_power_export_t3_kwh: Option<f64>,
    /// Energy exported on tariff 4 in kWh.
    pub total_power_export_t4_kwh: Option<f64>,

    // Voltage & current
    /// Active voltage in volts (single-phase devices).
    pub active_voltage_v: Option<f64>,
    /// Voltage on phase 1 in volts.
    pub active_voltage_l1_v: Option<f64>,
    /// Voltage on phase 2 in volts.
    pub active_voltage_l2_v: Option<f64>,
    /// Voltage on phase 3 in volts.
    pub active_voltage_l3_v: Option<f64>,
    /// Total active current in amps.
    pub active_current_a: Option<f64>,
    /// Current on phase 1 in amps.
    pub active_current_l1_a: Option<f64>,
    /// Current on phase 2 in amps.
    pub active_current_l2_a: Option<f64>,
    /// Current on phase 3 in amps.
    pub active_current_l3_a: Option<f64>,

    // Power quality
    /// Grid frequency in hertz.
    pub active_frequency_hz: Option<f64>,
    /// Power factor (0.0–1.0).
    pub active_power_factor: Option<f64>,
    /// Reactive power in volt-amperes reactive.
    pub active_reactive_power_var: Option<f64>,
    /// Apparent power in volt-amperes.
    pub active_apparent_power_va: Option<f64>,

    // Tariff (P1)
    /// Currently active tariff (1, 2, 3, or 4).
    pub active_tariff: Option<u32>,

    // Voltage sags (P1)
    /// Number of voltage sags detected on phase 1.
    pub voltage_sag_l1_count: Option<u64>,
    /// Number of voltage sags detected on phase 2.
    pub voltage_sag_l2_count: Option<u64>,
    /// Number of voltage sags detected on phase 3.
    pub voltage_sag_l3_count: Option<u64>,

    // Voltage swells (P1)
    /// Number of voltage swells detected on phase 1.
    pub voltage_swell_l1_count: Option<u64>,
    /// Number of voltage swells detected on phase 2.
    pub voltage_swell_l2_count: Option<u64>,
    /// Number of voltage swells detected on phase 3.
    pub voltage_swell_l3_count: Option<u64>,

    // Gas (P1)
    /// Total gas consumption in cubic meters.
    pub total_gas_m3: Option<f64>,
    /// Timestamp of the last gas reading (seconds since epoch).
    pub gas_timestamp: Option<u64>,
    /// Unique identifier of the connected gas meter.
    pub gas_unique_id: Option<String>,

    // External devices (P1)
    /// External device readings attached to the P1 meter (e.g. gas, water).
    pub external: Option<serde_json::Value>,

    // Water
    /// Current water flow in liters per minute.
    pub active_liter_lpm: Option<f64>,
    /// Total water consumption in cubic meters.
    pub total_liter_m3: Option<f64>,
    /// User-configured offset for the water meter in cubic meters.
    pub total_liter_offset_m3: Option<f64>,

    // P1 specific
    /// Smart Meter Requirements version (e.g. 50 for DSMR 5.0).
    pub smr_version: Option<u32>,
    /// Model identifier of the connected smart meter.
    pub meter_model: Option<String>,
    /// Unique identifier of the smart meter.
    pub unique_id: Option<String>,
    /// Monthly power peak in watts. The API spells it "montly" (sic).
    #[serde(rename = "montly_power_peak_w")]
    pub monthly_power_peak_w: Option<f64>,
    /// Timestamp of the monthly power peak. The API spells it "montly" (sic).
    #[serde(rename = "montly_power_peak_timestamp")]
    pub monthly_power_peak_timestamp: Option<u64>,
    /// Total number of power failures detected.
    pub any_power_fail_count: Option<u64>,
    /// Number of long power failures detected.
    pub long_power_fail_count: Option<u64>,
    /// Average active power in watts (rolling window).
    pub active_power_average_w: Option<f64>,
}

/// Summary of a discovered or saved device -- sent to the frontend via IPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSummary {
    /// Unique device identifier (typically the serial number).
    pub id: String,
    /// Display name.
    pub name: String,
    /// Product type.
    pub product_type: ProductType,
    /// Serial number.
    pub serial: String,
    /// Local network IP address.
    pub ip: String,
    /// HTTP port (usually 80).
    pub port: u16,
    /// Whether the device is currently reachable.
    pub online: bool,
}

/// Detailed device view including measurement data -- sent to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDetail {
    /// Basic device info and status.
    pub info: DeviceSummary,
    /// Latest measurement data, if available.
    pub data: Option<MeasurementData>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_device_info() {
        let json = r#"{
            "product_type": "HWE-P1",
            "product_name": "P1 Meter",
            "serial": "aabbccddeeff",
            "firmware_version": "4.19",
            "api_version": "v1"
        }"#;
        let info: DeviceInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.product_type, ProductType::P1Meter);
        assert_eq!(info.serial, "aabbccddeeff");
    }

    #[test]
    fn deserialize_product_type_variants() {
        assert_eq!(
            serde_json::from_str::<ProductType>(r#""HWE-P1""#).unwrap(),
            ProductType::P1Meter
        );
        assert_eq!(
            serde_json::from_str::<ProductType>(r#""HWE-SKT""#).unwrap(),
            ProductType::EnergySocket
        );
        assert_eq!(
            serde_json::from_str::<ProductType>(r#""HWE-WTR""#).unwrap(),
            ProductType::Watermeter
        );
        assert_eq!(
            serde_json::from_str::<ProductType>(r#""HWE-KWH1""#).unwrap(),
            ProductType::KwhMeter1Phase
        );
        assert_eq!(
            serde_json::from_str::<ProductType>(r#""HWE-KWH3""#).unwrap(),
            ProductType::KwhMeter3Phase
        );
        assert_eq!(
            serde_json::from_str::<ProductType>(r#""SDM230-wifi""#).unwrap(),
            ProductType::KwhMeter1Phase
        );
        assert_eq!(
            serde_json::from_str::<ProductType>(r#""SDM630-wifi""#).unwrap(),
            ProductType::KwhMeter3Phase
        );
    }

    #[test]
    fn deserialize_p1_data() {
        let json = r#"{
            "wifi_ssid": "MyWiFi",
            "wifi_strength": 72,
            "active_power_w": 543.0,
            "active_power_l1_w": 200.0,
            "active_power_l2_w": 143.0,
            "active_power_l3_w": 200.0,
            "total_power_import_kwh": 1234.567,
            "total_power_export_kwh": 0.0,
            "active_voltage_l1_v": 230.1,
            "active_current_l1_a": 0.87,
            "total_gas_m3": 567.89,
            "active_tariff": 2
        }"#;
        let data: MeasurementData = serde_json::from_str(json).unwrap();
        assert_eq!(data.active_power_w, Some(543.0));
        assert_eq!(data.total_gas_m3, Some(567.89));
        assert_eq!(data.active_tariff, Some(2));
        assert_eq!(data.wifi_ssid, Some("MyWiFi".to_string()));
    }

    #[test]
    fn deserialize_watermeter_data() {
        let json = r#"{
            "wifi_ssid": "MyWiFi",
            "wifi_strength": 80,
            "active_liter_lpm": 3.5,
            "total_liter_m3": 1234.567
        }"#;
        let data: MeasurementData = serde_json::from_str(json).unwrap();
        assert_eq!(data.active_liter_lpm, Some(3.5));
        assert_eq!(data.total_liter_m3, Some(1234.567));
        assert!(data.active_power_w.is_none());
    }
}
