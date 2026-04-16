import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useDiscoveryStore, type DiscoveredDevice } from '@/stores/discovery'
import { useDevicesStore } from '@/stores/devices'

const SCAN_DURATION_SECS = 60

/**
 * Manages the mDNS discovery scan lifecycle: countdown timer,
 * event listener, start/stop, and save logic.
 */
export function useDiscoveryScanner() {
  const discoveryStore = useDiscoveryStore()
  const devicesStore = useDevicesStore()

  const countdown = ref(SCAN_DURATION_SECS)
  const existingCount = ref(0)
  let timer: ReturnType<typeof setInterval> | null = null
  let unlisten: (() => void) | null = null

  const newDeviceCount = computed(() => discoveryStore.foundDevices.length - existingCount.value)

  async function startSearch() {
    if (unlisten) {
      unlisten()
      unlisten = null
    }
    if (timer) {
      clearInterval(timer)
      timer = null
    }

    discoveryStore.isSearching = true
    countdown.value = SCAN_DURATION_SECS
    unlisten = await listen<DiscoveredDevice>('device_found', (event) => {
      discoveryStore.addDevice(event.payload)
    })
    await invoke('start_discovery')
    timer = setInterval(() => {
      countdown.value--
      if (countdown.value <= 0) {
        stopSearch()
      }
    }, 1000)
  }

  async function stopSearch() {
    discoveryStore.isSearching = false
    if (timer) {
      clearInterval(timer)
    }
    if (unlisten) {
      unlisten()
    }
    try {
      await invoke('stop_discovery')
    } catch {}
  }

  async function saveSelected(): Promise<void> {
    const devices = discoveryStore.selectedAsDevices()
    await invoke('save_devices', { devices })
    devicesStore.setDevices(devices)
  }

  function loadExistingDevices() {
    discoveryStore.reset()
    const saved = Object.values(devicesStore.deviceStates).map((s) => s.device)
    discoveryStore.loadExisting(saved)
    existingCount.value = saved.length
  }

  onMounted(() => {
    loadExistingDevices()
    startSearch()
  })

  onUnmounted(() => {
    if (timer) {
      clearInterval(timer)
    }
    if (unlisten) {
      unlisten()
    }
  })

  return {
    countdown,
    newDeviceCount,
    startSearch,
    stopSearch,
    saveSelected,
  }
}
