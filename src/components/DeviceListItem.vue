<template>
  <div class="hw-device-item group flex items-center rounded-2xl py-3.5 pr-4 cursor-pointer">
    <div
      v-if="draggable"
      class="drag-handle flex items-center justify-center w-7 flex-shrink-0 cursor-grab active:cursor-grabbing self-stretch"
    >
      <GripVertical
        :size="14"
        class="text-neutral-600 group-hover:text-neutral-400 transition-colors"
      />
    </div>
    <div v-else class="w-4 flex-shrink-0" />
    <DeviceIcon :product-type="productType" class="mr-3" />
    <div class="flex-1 min-w-0">
      <div class="truncate text-[13px] font-medium text-neutral-200">
        {{ name }}
      </div>
      <div class="flex items-center gap-2 mt-0.5">
        <StatusBadge :online="online" />
        <span v-if="secondary" class="text-[9px] text-neutral-600">{{ secondary }}</span>
      </div>
    </div>
    <div class="text-right min-w-[48px]">
      <div class="text-xl font-semibold text-neutral-50 tabular-nums animate-glow">
        {{ metricValue ?? '—' }}
      </div>
      <div class="text-[9px] font-medium text-neutral-500 uppercase tracking-wider">
        {{ metricUnit }}
      </div>
    </div>
    <button
      v-if="hideable"
      class="ml-2 opacity-0 group-hover:opacity-100 transition-opacity text-neutral-700 hover:text-neutral-400 cursor-pointer"
      @click.stop="$emit('hide')"
    >
      <EyeOff :size="12" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { GripVertical, EyeOff } from 'lucide-vue-next'
import DeviceIcon from './DeviceIcon.vue'
import StatusBadge from './StatusBadge.vue'

withDefaults(
  defineProps<{
    name: string
    productType: string
    online: boolean
    metricValue: string | number | null
    metricUnit: string
    secondary?: string | null
    draggable?: boolean
    hideable?: boolean
  }>(),
  {
    secondary: undefined,
    draggable: false,
    hideable: false,
  },
)

defineEmits<{ hide: [] }>()
</script>

<style scoped>
.hw-device-item {
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.03), rgba(255, 255, 255, 0.01));
  border: 1px solid rgba(255, 255, 255, 0.06);
  transition:
    background 0.2s,
    border-color 0.2s;
}
.hw-device-item:hover {
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.06), rgba(255, 255, 255, 0.02));
  border-color: rgba(255, 255, 255, 0.1);
}
</style>
