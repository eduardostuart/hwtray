import { computed } from 'vue'
import { useDevicesStore } from '@/stores/devices'
import { isPowerDevice } from '@/types/products'
import { fmt } from '@/utils/format'
import type { DeviceState, MeasurementData } from '@/types/device'

const SEPARATOR = ' \u00B7 ' // middle dot: ·
const CUBIC_METERS = 'm\u00B3' // m³
const GAS_TIMESTAMP_LENGTH = 12

// Parses HomeWizard gas_timestamp (YYMMDDhhmmss format) into "HH:MM".
function parseGasTimestamp(ts: number | undefined): string | null {
  if (ts == null) {
    return null
  }
  const raw = String(ts)
  if (raw.length < GAS_TIMESTAMP_LENGTH) {
    return null
  }
  return `${raw.slice(6, 8)}:${raw.slice(8, 10)}`
}

// Sums a numeric field across an array of device states.
function sumField(
  devices: DeviceState[],
  getter: (data: MeasurementData) => number | undefined,
): number {
  return devices.reduce((acc, s) => {
    if (!s.data) {
      return acc
    }
    return acc + (getter(s.data) ?? 0)
  }, 0)
}

/**
 * Centralized access to HomeWizard device data.
 *
 * Provides reactive aggregates (total power, import/export, gas),
 * per-device capability checks, and display helpers.
 * All views should use this instead of computing values locally.
 *
 * @returns
 * - `deviceList` — all monitored devices
 * - `allOffline` — true when all devices are unreachable
 * - `networkOnline` — browser network status (ref)
 * - `powerDevices` — devices that report watts (excludes watermeter)
 * - `hasPowerDevice` — true if any power device exists
 * - `totalPower` — sum of active_power_w across power devices (number)
 * - `isExporting` — true when totalPower is negative (solar surplus)
 * - `totalImport` / `totalExport` — cumulative kWh from/to grid (number)
 * - `mainSparkline` — historical power readings for chart
 * - `activeTariff` — current tariff number (1 = peak, 2 = off-peak)
 * - `gasDevice` — device state reporting gas, or null
 * - `totalGas` — total gas consumption in m3
 * - `gasTimestamp` — last gas reading time as "HH:MM"
 * - `isThreePhase(data)` — true if L2 phase data exists
 * - `hasPower(data)` / `hasEnergy(data)` / `hasExport(data)` — capability checks
 * - `hasVoltage(data)` / `hasGas(data)` / `hasWater(data)` / `hasPowerFails(data)`
 * - `powerSubtitle(data)` — e.g. "3-phase · 231V · 6.1A · 0.98PF"
 * - `primaryMetric(state)` — main display value (watts or L/min)
 * - `primaryUnit(state)` — "W" or "L/min"
 * - `secondaryInfo(state)` — e.g. "3-phase · 231V" or "827.2 m³ total"
 */
export function useHomeWizard() {
  const store = useDevicesStore()

  // All monitored devices, regardless of type.
  const deviceList = computed(() => Object.values(store.deviceStates))

  // True when devices exist but none are responding.
  const allOffline = computed(
    () => deviceList.value.length > 0 && deviceList.value.every((s) => !s.online),
  )

  // Devices that report power data (excludes watermeter).
  const powerDevices = computed(() =>
    deviceList.value.filter((s) => isPowerDevice(s.device.product_type)),
  )

  // True if at least one power device is configured.
  const hasPowerDevice = computed(() => powerDevices.value.length > 0)

  // Total active power across all power devices (W). Negative means exporting.
  const totalPower = computed(() => sumField(powerDevices.value, (d) => d.active_power_w))

  // True when the household is exporting energy to the grid (solar surplus).
  const isExporting = computed(() => totalPower.value < 0)

  // Cumulative energy imported from the grid (kWh).
  const totalImport = computed(
    () => Math.round(sumField(powerDevices.value, (d) => d.total_power_import_kwh) * 10) / 10,
  )

  // Cumulative energy exported to the grid (kWh).
  const totalExport = computed(
    () => Math.round(sumField(powerDevices.value, (d) => d.total_power_export_kwh) * 10) / 10,
  )

  // Sparkline data from the first power device that has readings.
  const mainSparkline = computed(() => {
    const device = powerDevices.value.find((s) => s.sparkline.length > 0)
    return device?.sparkline ?? []
  })

  // Current electricity tariff (T1 = peak, T2 = off-peak).
  const activeTariff = computed(() => {
    const device = powerDevices.value.find((s) => s.data?.active_tariff != null)
    return device?.data?.active_tariff ?? null
  })

  // The device state that reports gas data, or null.
  const gasDevice = computed(
    () => deviceList.value.find((s) => s.data?.total_gas_m3 != null) ?? null,
  )

  // Total gas consumption (m3).
  const totalGas = computed(() => gasDevice.value?.data?.total_gas_m3 ?? null)

  // Last gas meter reading time formatted as HH:MM.
  const gasTimestamp = computed(() => parseGasTimestamp(gasDevice.value?.data?.gas_timestamp))

  // True if the device has L2 phase data, indicating a 3-phase installation.
  function isThreePhase(data: MeasurementData | null): boolean {
    return data?.active_power_l2_w != null
  }

  // True if the device reports active power (W).
  function hasPower(data: MeasurementData | null): boolean {
    return data?.active_power_w != null
  }

  // True if the device reports cumulative energy import.
  function hasEnergy(data: MeasurementData | null): boolean {
    return data?.total_power_import_kwh != null
  }

  // True if the device has exported any energy to the grid.
  function hasExport(data: MeasurementData | null): boolean {
    return (data?.total_power_export_kwh ?? 0) > 0
  }

  // True if the device reports voltage readings.
  function hasVoltage(data: MeasurementData | null): boolean {
    return data?.active_voltage_v != null || data?.active_voltage_l1_v != null
  }

  // True if the device reports gas consumption.
  function hasGas(data: MeasurementData | null): boolean {
    return data?.total_gas_m3 != null
  }

  // True if the device reports water flow.
  function hasWater(data: MeasurementData | null): boolean {
    return data?.active_liter_lpm != null
  }

  // True if the device reports grid power failure counts.
  function hasPowerFails(data: MeasurementData | null): boolean {
    return data?.any_power_fail_count != null
  }

  // Summary line for the power hero card.
  // Example: "3-phase · 231V · 6.1A · 0.98PF"
  function powerSubtitle(data: MeasurementData | null): string {
    if (!data) {
      return ''
    }
    return [
      isThreePhase(data) ? '3-phase' : '1-phase',
      fmt(data.active_voltage_v ?? data.active_voltage_l1_v, 'V', 0),
      fmt(data.active_current_a ?? data.active_current_l1_a, 'A'),
      fmt(data.active_power_factor, 'PF', 2),
    ]
      .filter(Boolean)
      .join(SEPARATOR)
  }

  // The main number to display for a device: watts for power, L/min for water.
  function primaryMetric(state: DeviceState): string | null {
    if (!state.data) {
      return null
    }
    if (state.data.active_power_w != null) {
      return fmt(state.data.active_power_w, '', 0)
    }
    if (state.data.active_liter_lpm != null) {
      return fmt(state.data.active_liter_lpm)
    }
    return null
  }

  // Unit label for the primary metric.
  function primaryUnit(state: DeviceState): string {
    if (state.data?.active_liter_lpm != null) {
      return 'L/min'
    }
    return 'W'
  }

  // Extra info shown below the device name.
  // Example: "3-phase · 231V" or "827.2 m³ total"
  function secondaryInfo(state: DeviceState): string | null {
    if (!state.data) {
      return null
    }

    const parts: string[] = []
    if (isThreePhase(state.data)) {
      parts.push('3-phase')
    }
    const voltage = state.data.active_voltage_v ?? state.data.active_voltage_l1_v
    if (voltage != null) {
      parts.push(`${voltage.toFixed(0)}V`)
    }
    if (parts.length > 0) {
      return parts.join(SEPARATOR)
    }

    if (state.data.total_liter_m3 != null) {
      return `${state.data.total_liter_m3.toFixed(1)} ${CUBIC_METERS} total`
    }
    return null
  }

  // A dashboard item with all display props pre-resolved.
  // Views bind these directly to DeviceListItem without knowing about device types.
  interface DashboardItem {
    id: string
    route: string
    routeParams?: Record<string, string>
    name: string
    productType: string
    online: boolean
    metricValue: string | null
    metricUnit: string
    secondary: string | null
  }

  // Build the list of dashboard items (devices + gas if available).
  const dashboardItems = computed<DashboardItem[]>(() => {
    const items: DashboardItem[] = []

    if (totalGas.value != null) {
      items.push({
        id: 'gas',
        route: 'gas',
        name: 'Gas',
        productType: 'gas',
        online: true,
        metricValue: fmt(totalGas.value),
        metricUnit: 'm3',
        secondary: null,
      })
    }

    for (const state of deviceList.value) {
      items.push({
        id: state.device.id,
        route: 'device',
        routeParams: { id: state.device.id },
        name: state.device.name,
        productType: state.device.product_type,
        online: state.online,
        metricValue: primaryMetric(state),
        metricUnit: primaryUnit(state),
        secondary: secondaryInfo(state),
      })
    }

    return items
  })

  return {
    deviceList,
    allOffline,
    networkOnline: store.networkOnline,
    dashboardItems,

    powerDevices,
    hasPowerDevice,
    totalPower,
    isExporting,
    totalImport,
    totalExport,
    mainSparkline,
    activeTariff,

    gasDevice,
    totalGas,
    gasTimestamp,

    isThreePhase,
    hasPower,
    hasEnergy,
    hasExport,
    hasVoltage,
    hasGas,
    hasWater,
    hasPowerFails,
    powerSubtitle,
    primaryMetric,
    primaryUnit,
    secondaryInfo,
  }
}
