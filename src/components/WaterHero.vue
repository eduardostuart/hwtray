<template>
  <div class="glass-card-water rounded-2xl px-4 pt-4 overflow-hidden">
    <div class="text-[11px] uppercase tracking-widest font-semibold text-blue-400/70 mb-1">
      Water flow
    </div>

    <div class="flex items-baseline gap-2">
      <span class="text-5xl font-bold text-neutral-50 tabular-nums leading-none">
        {{ flow }}
      </span>
      <span class="text-lg text-neutral-500 font-medium">L/min</span>
    </div>

    <div class="flex gap-4 mt-3 text-[12px]">
      <div class="flex items-center gap-1.5">
        <Droplets :size="12" color="#3B82F6" :stroke-width="2.5" />
        <span class="text-neutral-500">Total</span>
        <span class="text-neutral-200 tabular-nums font-medium">{{ total }}</span>
        <span class="text-neutral-500">m3</span>
      </div>
    </div>

    <Sparkline
      v-if="sparkline.length > 1"
      :data="sparkline"
      :width="320"
      :height="40"
      color="#3B82F6"
      class="mt-3"
    />
  </div>
</template>

<script setup lang="ts">
import { Droplets } from 'lucide-vue-next'
import Sparkline from './Sparkline.vue'

defineProps<{
  flow: string
  total: string
  sparkline: number[]
}>()
</script>

<style scoped>
.glass-card-water {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.08), rgba(59, 130, 246, 0.02));
  border: 1px solid rgba(59, 130, 246, 0.12);
}
</style>
