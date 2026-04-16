import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useHomeWizard } from './useHomeWizard'
import { useDevicesStore } from '@/stores/devices'
import type { MeasurementData } from '@/types/device'

function setupDevices(
  store: ReturnType<typeof useDevicesStore>,
  devices: Array<{
    id: string
    name: string
    product_type: string
    data?: Partial<MeasurementData>
  }>,
) {
  store.setDevices(
    devices.map((d) => ({
      id: d.id,
      name: d.name,
      product_type: d.product_type,
      ip: '192.168.1.1',
      port: 80,
    })),
  )
  for (const d of devices) {
    if (d.data) {
      store.updateTelemetry({
        id: d.id,
        data: d.data as MeasurementData,
        sparkline: [],
        online: true,
      })
    }
  }
}

describe('useHomeWizard', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('returns empty state when no devices', () => {
    const hw = useHomeWizard()
    expect(hw.deviceList.value).toEqual([])
    expect(hw.hasPowerDevice.value).toBe(false)
    expect(hw.totalGas.value).toBeNull()
    expect(hw.allOffline.value).toBe(false)
  })

  it('computes totalPower from power devices', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      { id: '1', name: 'P1', product_type: 'HWE-P1', data: { active_power_w: 300 } },
    ])
    const hw = useHomeWizard()
    expect(hw.totalPower.value).toBe(300)
    expect(hw.isExporting.value).toBe(false)
  })

  it('detects exporting when totalPower is negative', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      { id: '1', name: 'P1', product_type: 'HWE-P1', data: { active_power_w: -500 } },
    ])
    const hw = useHomeWizard()
    expect(hw.totalPower.value).toBe(-500)
    expect(hw.isExporting.value).toBe(true)
  })

  it('computes totalImport and totalExport', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      {
        id: '1',
        name: 'P1',
        product_type: 'HWE-P1',
        data: {
          total_power_import_kwh: 12345.6,
          total_power_export_kwh: 6789.1,
        },
      },
    ])
    const hw = useHomeWizard()
    expect(hw.totalImport.value).toBe(12345.6)
    expect(hw.totalExport.value).toBe(6789.1)
  })

  it('excludes watermeter from power devices', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      { id: '1', name: 'P1', product_type: 'HWE-P1', data: { active_power_w: 200 } },
      { id: '2', name: 'Water', product_type: 'HWE-WTR', data: { active_liter_lpm: 3.5 } },
    ])
    const hw = useHomeWizard()
    expect(hw.powerDevices.value).toHaveLength(1)
    expect(hw.totalPower.value).toBe(200)
  })

  it('computes totalGas from any device reporting it', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      { id: '1', name: 'P1', product_type: 'HWE-P1', data: { total_gas_m3: 8878.8 } },
    ])
    const hw = useHomeWizard()
    expect(hw.totalGas.value).toBe(8878.8)
  })

  it('formats gasTimestamp from YYMMDDhhmmss', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      {
        id: '1',
        name: 'P1',
        product_type: 'HWE-P1',
        data: { total_gas_m3: 100, gas_timestamp: 260415093012 },
      },
    ])
    const hw = useHomeWizard()
    expect(hw.gasTimestamp.value).toBe('09:30')
  })

  it('returns null gasTimestamp when missing', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      { id: '1', name: 'P1', product_type: 'HWE-P1', data: { total_gas_m3: 100 } },
    ])
    const hw = useHomeWizard()
    expect(hw.gasTimestamp.value).toBeNull()
  })

  it('detects allOffline', () => {
    const store = useDevicesStore()
    store.setDevices([{ id: '1', name: 'P1', product_type: 'HWE-P1', ip: '1.1.1.1', port: 80 }])
    store.setOffline('1')
    const hw = useHomeWizard()
    expect(hw.allOffline.value).toBe(true)
  })

  it('activeTariff returns tariff number', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      { id: '1', name: 'P1', product_type: 'HWE-P1', data: { active_tariff: 2 } },
    ])
    const hw = useHomeWizard()
    expect(hw.activeTariff.value).toBe(2)
  })
})

describe('useHomeWizard per-device helpers', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('isThreePhase detects L2 data', () => {
    const hw = useHomeWizard()
    expect(hw.isThreePhase({ active_power_l2_w: 100 } as MeasurementData)).toBe(true)
    expect(hw.isThreePhase({ active_power_l1_w: 100 } as MeasurementData)).toBe(false)
    expect(hw.isThreePhase(null)).toBe(false)
  })

  it('hasPower checks active_power_w', () => {
    const hw = useHomeWizard()
    expect(hw.hasPower({ active_power_w: 0 } as MeasurementData)).toBe(true)
    expect(hw.hasPower({} as MeasurementData)).toBe(false)
  })

  it('hasExport requires positive export kWh', () => {
    const hw = useHomeWizard()
    expect(hw.hasExport({ total_power_export_kwh: 100 } as MeasurementData)).toBe(true)
    expect(hw.hasExport({ total_power_export_kwh: 0 } as MeasurementData)).toBe(false)
    expect(hw.hasExport(null)).toBe(false)
  })

  it('hasWater checks active_liter_lpm', () => {
    const hw = useHomeWizard()
    expect(hw.hasWater({ active_liter_lpm: 0 } as MeasurementData)).toBe(true)
    expect(hw.hasWater({} as MeasurementData)).toBe(false)
  })

  it('powerSubtitle builds summary string', () => {
    const hw = useHomeWizard()
    const data = {
      active_power_l2_w: 100,
      active_voltage_l1_v: 231.5,
      active_current_l1_a: 6.1,
      active_power_factor: 0.98,
    } as MeasurementData

    const result = hw.powerSubtitle(data)
    expect(result).toContain('3-phase')
    expect(result).toContain('232V')
    expect(result).toContain('6.1A')
    expect(result).toContain('0.98PF')
  })

  it('powerSubtitle returns empty for null', () => {
    const hw = useHomeWizard()
    expect(hw.powerSubtitle(null)).toBe('')
  })

  it('primaryMetric returns watts for power devices', () => {
    const hw = useHomeWizard()
    const state = {
      device: { id: '1', name: 'P1', product_type: 'HWE-P1', ip: '', port: 80 },
      data: { active_power_w: 543.2 } as MeasurementData,
      sparkline: [],
      homeSparkline: [],
      online: true,
    }
    expect(hw.primaryMetric(state)).toBe('543')
  })

  it('primaryMetric returns L/min for watermeter', () => {
    const hw = useHomeWizard()
    const state = {
      device: { id: '2', name: 'Water', product_type: 'HWE-WTR', ip: '', port: 80 },
      data: { active_liter_lpm: 3.5 } as MeasurementData,
      sparkline: [],
      homeSparkline: [],
      online: true,
    }
    expect(hw.primaryMetric(state)).toBe('3.5')
    expect(hw.primaryUnit(state)).toBe('L/min')
  })

  it('secondaryInfo shows 3-phase and voltage', () => {
    const hw = useHomeWizard()
    const state = {
      device: { id: '1', name: 'P1', product_type: 'HWE-P1', ip: '', port: 80 },
      data: { active_power_l2_w: 100, active_voltage_l1_v: 230.5 } as MeasurementData,
      sparkline: [],
      homeSparkline: [],
      online: true,
    }
    expect(hw.secondaryInfo(state)).toBe('3-phase \u00B7 231V')
  })

  it('secondaryInfo shows water total', () => {
    const hw = useHomeWizard()
    const state = {
      device: { id: '2', name: 'Water', product_type: 'HWE-WTR', ip: '', port: 80 },
      data: { total_liter_m3: 827.2 } as MeasurementData,
      sparkline: [],
      homeSparkline: [],
      online: true,
    }
    expect(hw.secondaryInfo(state)).toBe('827.2 m\u00B3 total')
  })
})

describe('useHomeWizard dashboardItems', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('returns empty when no devices', () => {
    const hw = useHomeWizard()
    expect(hw.dashboardItems.value).toEqual([])
  })

  it('creates item for a power device with resolved props', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      { id: 'abc', name: 'P1 Meter', product_type: 'HWE-P1', data: { active_power_w: 300 } },
    ])
    const hw = useHomeWizard()
    const items = hw.dashboardItems.value

    expect(items).toHaveLength(1)
    expect(items[0].id).toBe('abc')
    expect(items[0].name).toBe('P1 Meter')
    expect(items[0].productType).toBe('HWE-P1')
    expect(items[0].online).toBe(true)
    expect(items[0].metricValue).toBe('300')
    expect(items[0].metricUnit).toBe('W')
    expect(items[0].route).toBe('device')
    expect(items[0].routeParams).toEqual({ id: 'abc' })
  })

  it('creates item for watermeter with L/min', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      { id: 'w1', name: 'Watermeter', product_type: 'HWE-WTR', data: { active_liter_lpm: 3.5 } },
    ])
    const hw = useHomeWizard()
    const items = hw.dashboardItems.value

    expect(items).toHaveLength(1)
    expect(items[0].metricValue).toBe('3.5')
    expect(items[0].metricUnit).toBe('L/min')
    expect(items[0].route).toBe('device')
  })

  it('includes gas item when P1 reports gas data', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      {
        id: 'p1',
        name: 'P1 Meter',
        product_type: 'HWE-P1',
        data: { active_power_w: 200, total_gas_m3: 8878.8 },
      },
    ])
    const hw = useHomeWizard()
    const items = hw.dashboardItems.value

    expect(items).toHaveLength(2)
    const gasItem = items.find((i) => i.id === 'gas')
    expect(gasItem).toBeDefined()
    expect(gasItem!.name).toBe('Gas')
    expect(gasItem!.productType).toBe('gas')
    expect(gasItem!.metricValue).toBe('8878.8')
    expect(gasItem!.metricUnit).toBe('m3')
    expect(gasItem!.route).toBe('gas')
    expect(gasItem!.routeParams).toBeUndefined()
  })

  it('does not include gas item when no gas data', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      { id: 'p1', name: 'P1 Meter', product_type: 'HWE-P1', data: { active_power_w: 200 } },
    ])
    const hw = useHomeWizard()
    expect(hw.dashboardItems.value.find((i) => i.id === 'gas')).toBeUndefined()
  })

  it('includes secondary info for 3-phase devices', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      {
        id: 'p1',
        name: 'P1',
        product_type: 'HWE-P1',
        data: { active_power_w: 100, active_power_l2_w: 50, active_voltage_l1_v: 231 },
      },
    ])
    const hw = useHomeWizard()
    const item = hw.dashboardItems.value[0]
    expect(item.secondary).toContain('3-phase')
    expect(item.secondary).toContain('231V')
  })

  it('gas item is always online', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      { id: 'p1', name: 'P1', product_type: 'HWE-P1', data: { total_gas_m3: 100 } },
    ])
    store.setOffline('p1')
    const hw = useHomeWizard()
    const gasItem = hw.dashboardItems.value.find((i) => i.id === 'gas')
    expect(gasItem?.online).toBe(true)
  })

  it('reflects device offline state', () => {
    const store = useDevicesStore()
    setupDevices(store, [
      { id: 'p1', name: 'P1', product_type: 'HWE-P1', data: { active_power_w: 0 } },
    ])
    store.setOffline('p1')
    const hw = useHomeWizard()
    const item = hw.dashboardItems.value.find((i) => i.id === 'p1')
    expect(item?.online).toBe(false)
  })
})
