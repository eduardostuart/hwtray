export interface SavedDevice {
  id: string
  name: string
  product_type: string
  ip: string
  port: number
}

export interface MeasurementData {
  wifi_ssid?: string
  wifi_strength?: number
  active_power_w?: number
  active_power_l1_w?: number
  active_power_l2_w?: number
  active_power_l3_w?: number
  total_power_import_kwh?: number
  total_power_import_t1_kwh?: number
  total_power_import_t2_kwh?: number
  total_power_export_kwh?: number
  total_power_export_t1_kwh?: number
  total_power_export_t2_kwh?: number
  active_voltage_v?: number
  active_voltage_l1_v?: number
  active_voltage_l2_v?: number
  active_voltage_l3_v?: number
  active_current_a?: number
  active_current_l1_a?: number
  active_current_l2_a?: number
  active_current_l3_a?: number
  active_frequency_hz?: number
  active_power_factor?: number
  active_tariff?: number
  total_gas_m3?: number
  gas_timestamp?: number
  active_liter_lpm?: number
  total_liter_m3?: number
  total_liter_offset_m3?: number
  smr_version?: number
  meter_model?: string
  any_power_fail_count?: number
  long_power_fail_count?: number
}

export interface TelemetryUpdate {
  id: string
  data: MeasurementData
  sparkline: number[]
  home_sparkline?: number[]
  online: boolean
}

export interface DeviceState {
  device: SavedDevice
  data: MeasurementData | null
  sparkline: number[]
  homeSparkline: number[]
  online: boolean
}

export interface AppSettings {
  poll_interval_ms: number
  theme: string
  notify_offline: boolean
  launch_at_login: boolean
  show_poll_indicator?: boolean
  always_on_top?: boolean
  dashboard_order?: string[]
  hidden_devices?: string[]
  tray_metrics?: TrayMetricConfig[]
}

export type TrayMetricField = 'active_power' | 'total_gas' | 'water_flow'

export interface TrayMetricConfig {
  device_id: string
  field: TrayMetricField
  label: string
}
