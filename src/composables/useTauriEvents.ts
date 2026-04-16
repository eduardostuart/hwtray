import { onMounted, onUnmounted } from 'vue'
import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { useDevicesStore } from '@/stores/devices'
import { useSettingsStore } from '@/stores/settings'
import type { TelemetryUpdate, AppSettings, SavedDevice, MeasurementData } from '@/types/device'

interface InitialState {
  devices: SavedDevice[]
  settings: AppSettings
  cached_sparklines: Record<string, number[]>
  cached_home_sparklines: Record<string, number[]>
  cached_data: Record<string, MeasurementData>
}

export function useTauriEvents() {
  const devicesStore = useDevicesStore()
  const settingsStore = useSettingsStore()
  const unlisteners: UnlistenFn[] = []
  let cleanupNetworkListeners: (() => void) | null = null

  onMounted(async () => {
    cleanupNetworkListeners = devicesStore.initNetworkListeners()

    try {
      const state = await invoke<InitialState>('get_initial_state')
      devicesStore.setDevices(state.devices)
      settingsStore.loadFromSettings(state.settings)

      for (const [id, sparkline] of Object.entries(state.cached_sparklines)) {
        devicesStore.setCachedSparkline(id, sparkline)
      }
      for (const [id, sparkline] of Object.entries(state.cached_home_sparklines ?? {})) {
        devicesStore.setCachedHomeSparkline(id, sparkline)
      }
      for (const [id, data] of Object.entries(state.cached_data ?? {})) {
        devicesStore.setCachedData(id, data)
      }
    } catch (e) {
      console.warn('[hw] Failed to load initial state:', e)
    }

    unlisteners.push(
      await listen<TelemetryUpdate>('telemetry_update', (event) => {
        devicesStore.updateTelemetry(event.payload)
      }),
    )
    unlisteners.push(
      await listen<{ id: string }>('device_offline', (event) => {
        devicesStore.setOffline(event.payload.id)
      }),
    )
    unlisteners.push(
      await listen<{ id: string }>('device_online', (event) => {
        devicesStore.setOnline(event.payload.id)
      }),
    )

    await emit('frontend_ready')
  })

  onUnmounted(() => {
    unlisteners.forEach((fn) => fn())
    if (cleanupNetworkListeners) {
      cleanupNetworkListeners()
      cleanupNetworkListeners = null
    }
  })
}
