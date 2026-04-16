<template>
  <label
    :class="[
      'flex items-center gap-3 rounded-2xl px-4 py-3.5 cursor-pointer transition-all duration-200',
      selected ? 'checkbox-card-selected' : 'checkbox-card',
    ]"
  >
    <div
      :class="[
        'flex h-4 w-4 items-center justify-center rounded-md transition-all duration-200 border',
        selected
          ? 'bg-[#10b981] border-[#10b981]'
          : 'bg-transparent border-[rgba(255,255,255,0.2)]',
      ]"
    >
      <Check v-if="selected" :size="10" color="white" :stroke-width="3" />
    </div>
    <DeviceIcon :product-type="device.product_type" />
    <div class="flex-1 min-w-0">
      <div class="truncate text-[13px] font-medium text-neutral-200">{{ device.name }}</div>
      <div class="text-[10px] text-neutral-600 font-mono">{{ device.ip }}:{{ device.port }}</div>
      <div v-if="subtitle" class="text-[9px] text-neutral-500 mt-0.5">{{ subtitle }}</div>
    </div>
    <input type="checkbox" :checked="selected" class="sr-only" @change="$emit('toggle')" />
  </label>
</template>

<script setup lang="ts">
import { Check } from 'lucide-vue-next'
import DeviceIcon from './DeviceIcon.vue'

withDefaults(
  defineProps<{
    device: { name: string; ip: string; port: number; product_type: string }
    selected: boolean
    subtitle?: string
  }>(),
  { subtitle: undefined },
)
defineEmits<{ toggle: [] }>()
</script>

<style scoped>
.checkbox-card {
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.03), rgba(255, 255, 255, 0.01));
  border: 1px solid rgba(255, 255, 255, 0.06);
}
.checkbox-card-selected {
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.08), rgba(16, 185, 129, 0.03));
  border: 1px solid rgba(16, 185, 129, 0.2);
}
</style>
