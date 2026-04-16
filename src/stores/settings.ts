import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { AppSettings, TrayMetricConfig } from '@/types/device'

export const useSettingsStore = defineStore('settings', () => {
  const pollIntervalMs = ref(500)
  const theme = ref<'dark' | 'light'>('dark')
  const notifyOffline = ref(true)
  const launchAtLogin = ref(false)
  const showPollIndicator = ref(true)
  const alwaysOnTop = ref(false)
  const dashboardOrder = ref<string[]>([])
  const hiddenDevices = ref<string[]>([])
  const trayMetrics = ref<TrayMetricConfig[]>([])

  function loadFromSettings(settings: AppSettings) {
    pollIntervalMs.value = settings.poll_interval_ms
    theme.value = settings.theme as 'dark' | 'light'
    notifyOffline.value = settings.notify_offline
    launchAtLogin.value = settings.launch_at_login
    showPollIndicator.value = settings.show_poll_indicator ?? true
    alwaysOnTop.value = settings.always_on_top ?? false
    dashboardOrder.value = settings.dashboard_order ?? []
    hiddenDevices.value = settings.hidden_devices ?? []
    trayMetrics.value = settings.tray_metrics ?? []
  }

  function toSettings(): AppSettings {
    return {
      poll_interval_ms: pollIntervalMs.value,
      theme: theme.value,
      notify_offline: notifyOffline.value,
      launch_at_login: launchAtLogin.value,
      show_poll_indicator: showPollIndicator.value,
      always_on_top: alwaysOnTop.value,
      dashboard_order: dashboardOrder.value,
      hidden_devices: hiddenDevices.value,
      tray_metrics: trayMetrics.value,
    }
  }

  function isHidden(id: string): boolean {
    return hiddenDevices.value.includes(id)
  }

  function toggleHidden(id: string) {
    if (isHidden(id)) {
      hiddenDevices.value = hiddenDevices.value.filter((h) => h !== id)
    } else {
      hiddenDevices.value = [...hiddenDevices.value, id]
    }
  }

  return {
    pollIntervalMs,
    theme,
    notifyOffline,
    launchAtLogin,
    showPollIndicator,
    alwaysOnTop,
    dashboardOrder,
    hiddenDevices,
    trayMetrics,
    isHidden,
    toggleHidden,
    loadFromSettings,
    toSettings,
  }
})
