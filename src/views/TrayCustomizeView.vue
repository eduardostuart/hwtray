<template>
  <WindowLayout>
    <template #title>Menu Bar</template>

    <div class="space-y-4">
      <InfoBox>
        Show live device readings in the macOS menu bar. Each metric appears as an independent item
        next to the tray icon.
      </InfoBox>

      <div
        v-for="(metric, idx) in metrics"
        :key="idx"
        class="rounded-xl px-4 py-3 space-y-3 bg-[linear-gradient(135deg,rgba(255,255,255,0.03),rgba(255,255,255,0.01))] border border-[rgba(255,255,255,0.06)]"
      >
        <div class="flex items-center justify-between">
          <div class="text-[10px] text-neutral-500 uppercase tracking-wider">
            Metric {{ idx + 1 }}
          </div>
          <button
            class="text-[10px] text-neutral-600 hover:text-hw-red cursor-pointer transition-colors"
            @click="remove(idx)"
          >
            Remove
          </button>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <div>
            <div class="text-[9px] text-neutral-600 mb-1">Device</div>
            <BaseSelect
              :model-value="metric.device_id"
              @update:model-value="onDeviceChange(idx, String($event))"
            >
              <option v-for="d in devices" :key="d.id" :value="d.id">{{ d.name }}</option>
            </BaseSelect>
          </div>
          <div>
            <div class="text-[9px] text-neutral-600 mb-1">Field</div>
            <BaseSelect
              :model-value="metric.field"
              @update:model-value="updateField(idx, String($event))"
            >
              <option
                v-for="f in fieldsForDevice(metric.device_id)"
                :key="f.value"
                :value="f.value"
              >
                {{ f.label }}
              </option>
            </BaseSelect>
          </div>
        </div>

        <div>
          <div class="text-[9px] text-neutral-600 mb-1">Label</div>
          <input
            :value="metric.label"
            :placeholder="deviceName(metric.device_id)"
            maxlength="12"
            class="w-full rounded-lg bg-neutral-800/50 px-3 py-2 text-[12px] text-neutral-300 outline-none placeholder-neutral-600 border border-[rgba(255,255,255,0.05)]"
            @input="updateLabel(idx, ($event.target as HTMLInputElement).value)"
          />
        </div>
      </div>

      <button
        v-if="devices.length > 0"
        class="w-full rounded-lg py-3 text-[13px] font-medium text-neutral-500 hover:text-neutral-300 cursor-pointer transition-colors border border-dashed border-[rgba(255,255,255,0.08)]"
        @click="add"
      >
        + Add metric
      </button>
    </div>
  </WindowLayout>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import WindowLayout from '@/components/WindowLayout.vue'
import InfoBox from '@/components/InfoBox.vue'
import BaseSelect from '@/components/BaseSelect.vue'
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
