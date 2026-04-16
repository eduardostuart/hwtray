import { defineStore } from 'pinia'
import { reactive, ref } from 'vue'
import type { SavedDevice, TelemetryUpdate, DeviceState, MeasurementData } from '@/types/device'

export const useDevicesStore = defineStore('devices', () => {
  const deviceStates = reactive<Record<string, DeviceState>>({})
  const networkOnline = ref(navigator.onLine)

  function setDevices(devices: SavedDevice[]) {
    for (const d of devices) {
      if (!deviceStates[d.id]) {
        deviceStates[d.id] = {
          device: d,
          data: null,
          sparkline: [],
          homeSparkline: [],
          online: true,
        }
      } else {
        deviceStates[d.id].device = d
      }
    }
    for (const id of Object.keys(deviceStates)) {
      if (!devices.find((d) => d.id === id)) {
        delete deviceStates[id]
      }
    }
  }

  function updateTelemetry(update: TelemetryUpdate) {
    if (deviceStates[update.id]) {
      deviceStates[update.id].data = update.data
      deviceStates[update.id].sparkline = update.sparkline
      deviceStates[update.id].homeSparkline = update.home_sparkline ?? []
      deviceStates[update.id].online = update.online
    }
  }

  function setOffline(id: string) {
    if (deviceStates[id]) {
      deviceStates[id].online = false
    }
  }

  function setOnline(id: string) {
    if (deviceStates[id]) {
      deviceStates[id].online = true
    }
  }

  function setCachedSparkline(id: string, sparkline: number[]) {
    if (deviceStates[id]) {
      deviceStates[id].sparkline = sparkline
    }
  }

  function setCachedHomeSparkline(id: string, sparkline: number[]) {
    if (deviceStates[id]) {
      deviceStates[id].homeSparkline = sparkline
    }
  }

  function setCachedData(id: string, data: MeasurementData) {
    if (deviceStates[id] && !deviceStates[id].data) {
      deviceStates[id].data = data
    }
  }

  function initNetworkListeners() {
    const onOnline = () => {
      networkOnline.value = true
    }
    const onOffline = () => {
      networkOnline.value = false
    }
    window.addEventListener('online', onOnline)
    window.addEventListener('offline', onOffline)
    return () => {
      window.removeEventListener('online', onOnline)
      window.removeEventListener('offline', onOffline)
    }
  }

  return {
    deviceStates,
    networkOnline,
    setDevices,
    updateTelemetry,
    setOffline,
    setOnline,
    setCachedSparkline,
    setCachedHomeSparkline,
    setCachedData,
    initNetworkListeners,
  }
})
