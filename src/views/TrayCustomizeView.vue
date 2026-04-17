<template>
  <WindowLayout>
    <template #title>Menu Bar</template>

    <InfoBox>
      Show live device readings in the macOS menu bar. Each metric appears as an independent item
      next to the tray icon.
    </InfoBox>

    <TrayMetricEditor
      v-for="(metric, idx) in metrics"
      :key="idx"
      :index="idx"
      :metric="metric"
      :devices="devices"
      :field-options="fieldsForDevice(metric.device_id)"
      :placeholder="deviceName(metric.device_id)"
      @change-device="onDeviceChange(idx, $event)"
      @change-field="updateField(idx, $event)"
      @change-label="updateLabel(idx, $event)"
      @remove="remove(idx)"
    />

    <AddRowButton v-if="devices.length > 0" @click="add">+ Add metric</AddRowButton>
  </WindowLayout>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import WindowLayout from '@/components/WindowLayout.vue'
import InfoBox from '@/components/InfoBox.vue'
import TrayMetricEditor from '@/components/TrayMetricEditor.vue'
import AddRowButton from '@/components/AddRowButton.vue'
import { useSettingsStore } from '@/stores/settings'
import { ProductType } from '@/types/products'
import type { SavedDevice, TrayMetricField, AppSettings } from '@/types/device'

const settingsStore = useSettingsStore()
const devices = ref<SavedDevice[]>([])

onMounted(async () => {
  try {
    const state = await invoke<{ devices: SavedDevice[]; settings: AppSettings }>(
      'get_initial_state',
    )
    devices.value = state.devices
    settingsStore.loadFromSettings(state.settings)
  } catch {
    // Initial state not available yet
  }
})

const metrics = computed(() => settingsStore.trayMetrics)

interface FieldOption {
  value: TrayMetricField
  label: string
}

function deviceName(id: string): string {
  return devices.value.find((d) => d.id === id)?.name ?? '—'
}

function fieldsForDevice(deviceId: string): FieldOption[] {
  const device = devices.value.find((d: SavedDevice) => d.id === deviceId)
  if (!device) {
    return [{ value: 'active_power', label: 'Power (W)' }]
  }
  switch (device.product_type) {
    case ProductType.WATERMETER:
      return [{ value: 'water_flow', label: 'Water (L/min)' }]
    case ProductType.GAS:
      return [{ value: 'total_gas', label: 'Gas (m³)' }]
    case ProductType.P1_METER:
      return [
        { value: 'active_power', label: 'Power (W)' },
        { value: 'total_gas', label: 'Gas (m³)' },
      ]
    default:
      return [{ value: 'active_power', label: 'Power (W)' }]
  }
}

function defaultFieldForDevice(deviceId: string): TrayMetricField {
  return fieldsForDevice(deviceId)[0].value
}

async function save() {
  try {
    await invoke('update_settings', { settings: settingsStore.toSettings() })
  } catch {}
}

function add() {
  const first = devices.value[0]
  if (!first) {
    return
  }
  settingsStore.trayMetrics = [
    ...metrics.value,
    { device_id: first.id, field: defaultFieldForDevice(first.id), label: '' },
  ]
  save()
}

function remove(idx: number) {
  settingsStore.trayMetrics = metrics.value.filter((_, i) => i !== idx)
  save()
}

function onDeviceChange(idx: number, deviceId: string) {
  const updated = [...metrics.value]
  updated[idx] = { device_id: deviceId, field: defaultFieldForDevice(deviceId), label: '' }
  settingsStore.trayMetrics = updated
  save()
}

function updateField(idx: number, field: string) {
  const updated = [...metrics.value]
  updated[idx] = { ...updated[idx], field: field as TrayMetricField }
  settingsStore.trayMetrics = updated
  save()
}

function updateLabel(idx: number, label: string) {
  const updated = [...metrics.value]
  updated[idx] = { ...updated[idx], label: label.slice(0, 12) }
  settingsStore.trayMetrics = updated
  save()
}
</script>
