import { invoke } from '@tauri-apps/api/core'
import { useDevicesStore } from '@/stores/devices'
import { useToast } from '@/composables/useToast'

const PULSE_DURATION_MS = 3000

export function useDeviceActions() {
  const store = useDevicesStore()
  const { error } = useToast()

  async function identifyDevice(id: string) {
    store.startIdentifying(id)
    setTimeout(() => store.stopIdentifying(id), PULSE_DURATION_MS)
    try {
      await invoke('identify_device', { id })
    } catch {
      error('Não foi possível identificar o dispositivo')
    }
  }

  return { identifyDevice }
}
