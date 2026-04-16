<template>
  <div
    :class="[
      'rounded-2xl px-4 pt-4 overflow-hidden',
      isExporting ? 'glass-card-export' : 'glass-card-import',
    ]"
  >
    <div class="flex items-center justify-between mb-1">
      <div
        class="text-[11px] uppercase tracking-widest font-semibold"
        :class="isExporting ? 'text-hw-green/70' : 'text-hw-orange/70'"
      >
        {{ isExporting ? 'Exporting to grid' : 'Current usage' }}
      </div>
      <div class="flex items-center gap-2">
        <div v-if="activeTariff" class="text-[11px] text-neutral-500 font-medium tabular-nums">
          T{{ activeTariff }}
        </div>
        <div v-if="hasExportData" class="flex gap-2 text-[10px] text-neutral-500">
          <span class="flex items-center gap-1"
            ><span class="inline-block w-2 h-2 rounded-full bg-[#A78BFA]" /> Grid</span
          >
          <span class="flex items-center gap-1"
            ><span class="inline-block w-2 h-2 rounded-full bg-hw-green" /> Surplus</span
          >
        </div>
      </div>
    </div>

    <div class="flex items-baseline gap-2">
      <span class="text-5xl font-bold text-neutral-50 tabular-nums leading-none animate-glow">
        {{ animatedValue }}
      </span>
      <span class="text-lg text-neutral-500 font-medium">{{ unit }}</span>
    </div>

    <div v-if="subtitle" class="text-[10px] text-neutral-600 mt-1">
      {{ subtitle }}
    </div>

    <div class="flex gap-4 mt-3 text-[12px]">
      <div v-if="importKwh != null" class="flex items-center gap-1.5">
        <ArrowDown :size="12" color="#EF4444" :stroke-width="2.5" />
        <span class="text-neutral-500">Import</span>
        <span class="text-neutral-200 tabular-nums font-medium">{{ importKwh }}</span>
        <span class="text-neutral-500">kWh</span>
      </div>
      <div v-if="exportKwh != null && Number(exportKwh) > 0" class="flex items-center gap-1.5">
        <ArrowUp :size="12" color="#10B981" :stroke-width="2.5" />
        <span class="text-neutral-500">Export</span>
        <span class="text-neutral-200 tabular-nums font-medium">{{ exportKwh }}</span>
        <span class="text-neutral-500">kWh</span>
      </div>
    </div>

    <PowerChart :data="sparkline" />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ArrowDown, ArrowUp } from 'lucide-vue-next'
import { useAnimatedNumber } from '@/composables/useAnimatedNumber'
import PowerChart from './PowerChart.vue'

const props = defineProps<{
  value: string
  unit: string
  subtitle?: string
  sparkline: number[]
  activeTariff?: number | null
  importKwh?: string | null
  exportKwh?: string | null
}>()

const isExporting = computed(() => Number(props.value) < 0)
const animatedValue = useAnimatedNumber(() => Math.abs(Number(props.value) || 0))
const hasExportData = computed(() => props.sparkline.some((v) => v < 0))
</script>

<style scoped>
.glass-card-export {
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.06), rgba(16, 185, 129, 0.01));
  border: 1px solid rgba(16, 185, 129, 0.1);
}
.glass-card-import {
  background: linear-gradient(135deg, rgba(245, 158, 11, 0.06), rgba(245, 158, 11, 0.01));
  border: 1px solid rgba(245, 158, 11, 0.1);
}
</style>
