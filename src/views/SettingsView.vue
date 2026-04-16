<template>
  <DashboardLayout>
    <template #title> Settings </template>
    <template #actions>
      <BackButton @click="router.push({ name: 'dashboard' })" />
    </template>

    <SettingsSelect
      label="Poll interval"
      description="How often to fetch data from devices. Min 500ms."
      :model-value="settingsStore.pollIntervalMs"
      :options="pollOptions"
      @update:model-value="updatePollInterval"
    />

    <SettingsToggle
      label="Launch at login"
      description="Start automatically when you log in."
      :model-value="settingsStore.launchAtLogin"
      @update:model-value="updateLaunchAtLogin"
    />

    <SettingsToggle
      label="Always on top"
      description="Keep the window above other windows. When off, clicking outside will close it."
      :model-value="settingsStore.alwaysOnTop"
      @update:model-value="updateAlwaysOnTop"
    />

    <SettingsToggle
      label="Poll indicator"
      description="Show countdown circle in the footer. Only visible at 5s+ intervals."
      :model-value="settingsStore.showPollIndicator"
      @update:model-value="updateShowPollIndicator"
    />

    <SearchButton variant="ghost" small @click="router.push({ name: 'discovery' })">
      Re-discover devices
    </SearchButton>
  </DashboardLayout>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { enable, disable } from '@tauri-apps/plugin-autostart'
import { useSettingsStore } from '@/stores/settings'
import DashboardLayout from '@/components/DashboardLayout.vue'
import BackButton from '@/components/BackButton.vue'
import SettingsSelect from '@/components/SettingsSelect.vue'
import SettingsToggle from '@/components/SettingsToggle.vue'
import SearchButton from '@/components/SearchButton.vue'

const router = useRouter()
const settingsStore = useSettingsStore()

const pollOptions = [
  { value: 500, label: '500ms' },
  { value: 1000, label: '1 second' },
  { value: 2000, label: '2 seconds' },
  { value: 5000, label: '5 seconds' },
  { value: 10000, label: '10 seconds' },
]

async function saveSettings() {
  try {
    await invoke('update_settings', { settings: settingsStore.toSettings() })
  } catch {}
}
async function updatePollInterval(v: string | number) {
  settingsStore.pollIntervalMs = Number(v)
  await saveSettings()
}
async function updateLaunchAtLogin(v: boolean) {
  settingsStore.launchAtLogin = v
  try {
    if (v) {
      await enable()
    } else {
      await disable()
    }
  } catch {}
  await saveSettings()
}
async function updateAlwaysOnTop(v: boolean) {
  settingsStore.alwaysOnTop = v
  try {
    await getCurrentWebviewWindow().setAlwaysOnTop(v)
  } catch {}
  await saveSettings()
}
async function updateShowPollIndicator(v: boolean) {
  settingsStore.showPollIndicator = v
  await saveSettings()
}
</script>
