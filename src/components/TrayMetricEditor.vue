<template>
  <div
    class="rounded-xl px-4 py-3 space-y-3 bg-[linear-gradient(135deg,rgba(255,255,255,0.03),rgba(255,255,255,0.01))] border border-[rgba(255,255,255,0.06)]"
  >
    <div class="flex items-center justify-between">
      <div class="text-[10px] text-neutral-500 uppercase tracking-wider">
        Metric {{ index + 1 }}
      </div>
      <button
        class="text-[10px] text-neutral-600 hover:text-hw-red cursor-pointer transition-colors"
        @click="$emit('remove')"
      >
        Remove
      </button>
    </div>

    <div class="grid grid-cols-2 gap-2">
      <div>
        <div class="text-[9px] text-neutral-600 mb-1">Device</div>
        <BaseSelect
          :model-value="metric.device_id"
          @update:model-value="$emit('change-device', String($event))"
        >
          <option v-for="d in devices" :key="d.id" :value="d.id">{{ d.name }}</option>
        </BaseSelect>
      </div>
      <div>
        <div class="text-[9px] text-neutral-600 mb-1">Field</div>
        <BaseSelect
          :model-value="metric.field"
          @update:model-value="$emit('change-field', String($event))"
        >
          <option v-for="f in fieldOptions" :key="f.value" :value="f.value">
            {{ f.label }}
          </option>
        </BaseSelect>
      </div>
    </div>

    <div>
      <div class="text-[9px] text-neutral-600 mb-1">Label</div>
      <input
        :value="metric.label"
        :placeholder="placeholder"
        maxlength="12"
        class="w-full rounded-lg bg-neutral-800/50 px-3 py-2 text-[12px] text-neutral-300 outline-none placeholder-neutral-600 border border-[rgba(255,255,255,0.05)]"
        @input="$emit('change-label', ($event.target as HTMLInputElement).value)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import BaseSelect from './BaseSelect.vue'
import type { SavedDevice, TrayMetricConfig, TrayMetricField } from '@/types/device'

defineProps<{
  index: number
  metric: TrayMetricConfig
  devices: SavedDevice[]
  fieldOptions: Array<{ value: TrayMetricField; label: string }>
  placeholder: string
}>()

defineEmits<{
  'change-device': [deviceId: string]
  'change-field': [field: string]
  'change-label': [label: string]
  remove: []
}>()
</script>
