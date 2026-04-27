import { ref, readonly } from 'vue'
import { check, type Update } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'

export type UpdateStatus = 'idle' | 'checking' | 'available' | 'downloading' | 'ready' | 'error'

const status = ref<UpdateStatus>('idle')
const progress = ref(0)
const error = ref<string | null>(null)
const availableUpdate = ref<Update | null>(null)

export function useUpdater() {
  async function checkForUpdates(): Promise<boolean> {
    status.value = 'checking'
    error.value = null
    progress.value = 0

    try {
      const update = await check()
      if (update) {
        availableUpdate.value = update
        status.value = 'available'
        return true
      }
      status.value = 'idle'
      return false
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to check for updates'
      status.value = 'error'
      return false
    }
  }

  async function downloadAndInstall(): Promise<void> {
    if (!availableUpdate.value) {
      return
    }

    status.value = 'downloading'
    error.value = null

    try {
      await availableUpdate.value.downloadAndInstall((event) => {
        if (event.event === 'Started' && event.data.contentLength) {
          progress.value = 0
        } else if (event.event === 'Progress') {
          progress.value = event.data.chunkLength
        } else if (event.event === 'Finished') {
          status.value = 'ready'
        }
      })
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to download update'
      status.value = 'error'
    }
  }

  async function restartApp(): Promise<void> {
    await relaunch()
  }

  function reset(): void {
    status.value = 'idle'
    progress.value = 0
    error.value = null
    availableUpdate.value = null
  }

  return {
    status: readonly(status),
    progress: readonly(progress),
    error: readonly(error),
    version: () => availableUpdate.value?.version ?? null,
    checkForUpdates,
    downloadAndInstall,
    restartApp,
    reset,
  }
}
