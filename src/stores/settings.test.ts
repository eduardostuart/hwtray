import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useSettingsStore } from './settings'

describe('useSettingsStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('has sensible defaults', () => {
    const store = useSettingsStore()
    expect(store.pollIntervalMs).toBe(500)
    expect(store.theme).toBe('dark')
    expect(store.notifyOffline).toBe(true)
    expect(store.launchAtLogin).toBe(false)
    expect(store.showPollIndicator).toBe(true)
    expect(store.dashboardOrder).toEqual([])
    expect(store.trayMetrics).toEqual([])
  })

  it('loads settings from backend payload', () => {
    const store = useSettingsStore()
    store.loadFromSettings({
      poll_interval_ms: 5000,
      theme: 'light',
      notify_offline: false,
      launch_at_login: true,
      show_poll_indicator: false,
      dashboard_order: ['gas', 'device1'],
    })
    expect(store.pollIntervalMs).toBe(5000)
    expect(store.theme).toBe('light')
    expect(store.notifyOffline).toBe(false)
    expect(store.launchAtLogin).toBe(true)
    expect(store.showPollIndicator).toBe(false)
    expect(store.dashboardOrder).toEqual(['gas', 'device1'])
  })

  it('exports settings back to backend format', () => {
    const store = useSettingsStore()
    store.pollIntervalMs = 2000
    store.theme = 'light'
    store.notifyOffline = false
    const out = store.toSettings()
    expect(out.poll_interval_ms).toBe(2000)
    expect(out.theme).toBe('light')
    expect(out.notify_offline).toBe(false)
  })

  it('handles missing optional fields gracefully', () => {
    const store = useSettingsStore()
    store.loadFromSettings({
      poll_interval_ms: 1000,
      theme: 'dark',
      notify_offline: true,
      launch_at_login: false,
    })
    expect(store.showPollIndicator).toBe(true)
    expect(store.dashboardOrder).toEqual([])
    expect(store.trayMetrics).toEqual([])
  })

  it('loads and exports tray metrics', () => {
    const store = useSettingsStore()
    const metrics = [
      { device_id: 'abc', field: 'active_power' as const, label: 'Power' },
      { device_id: 'def', field: 'total_gas' as const, label: '' },
    ]
    store.loadFromSettings({
      poll_interval_ms: 500,
      theme: 'dark',
      notify_offline: true,
      launch_at_login: false,
      tray_metrics: metrics,
    })
    expect(store.trayMetrics).toEqual(metrics)
    expect(store.toSettings().tray_metrics).toEqual(metrics)
  })

  it('updates tray metrics independently', () => {
    const store = useSettingsStore()
    expect(store.trayMetrics).toEqual([])

    store.trayMetrics = [{ device_id: 'x', field: 'water_flow', label: 'H2O' }]
    const out = store.toSettings()
    expect(out.tray_metrics).toHaveLength(1)
    expect(out.tray_metrics![0].label).toBe('H2O')
  })
})
