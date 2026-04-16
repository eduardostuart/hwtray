import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { SavedDevice } from '@/types/device'

export interface DiscoveredDevice {
  id: string
  name: string
  ip: string
  port: number
  serial: string
  product_type: string
  api_enabled: boolean
}

export const useDiscoveryStore = defineStore('discovery', () => {
  const isSearching = ref(false)
  const foundDevices = ref<DiscoveredDevice[]>([])
  const selectedIds = ref<Set<string>>(new Set())

  function addDevice(device: DiscoveredDevice) {
    const existing = foundDevices.value.findIndex((d) => d.id === device.id)
    if (existing !== -1) {
      // Update IP/port in case it changed
      foundDevices.value[existing] = device
    } else {
      foundDevices.value.push(device)
    }
  }

  function loadExisting(devices: SavedDevice[]) {
    for (const d of devices) {
      if (!foundDevices.value.find((f) => f.id === d.id)) {
        foundDevices.value.push({
          id: d.id,
          name: d.name,
          ip: d.ip,
          port: d.port,
          serial: d.id,
          product_type: d.product_type,
          api_enabled: true,
        })
      }
      selectedIds.value.add(d.id)
    }
  }

  function toggleSelection(id: string) {
    if (selectedIds.value.has(id)) {
      selectedIds.value.delete(id)
    } else {
      selectedIds.value.add(id)
    }
  }

  function reset() {
    foundDevices.value = []
    selectedIds.value.clear()
    isSearching.value = false
  }

  function selectedAsDevices(): SavedDevice[] {
    return foundDevices.value
      .filter((d) => selectedIds.value.has(d.id))
      .map((d) => ({
        id: d.id,
        name: d.name,
        product_type: d.product_type,
        ip: d.ip,
        port: d.port,
      }))
  }

  return {
    isSearching,
    foundDevices,
    selectedIds,
    addDevice,
    loadExisting,
    toggleSelection,
    reset,
    selectedAsDevices,
  }
})
