<template>
  <div class="glass-card-red rounded-2xl px-4 pt-4 pb-4">
    <div class="text-[11px] uppercase tracking-widest font-semibold text-hw-red/70 mb-1">
      Gas consumption
    </div>

    <div class="flex items-baseline gap-2">
      <span class="text-5xl font-bold text-neutral-50 tabular-nums leading-none">
        {{ total }}
      </span>
      <span class="text-lg text-neutral-500 font-medium">m3</span>
    </div>

    <div v-if="timestamp" class="flex items-center gap-1.5 mt-3 text-[12px]">
      <Flame :size="12" color="#EF4444" />
      <span class="text-neutral-500">Last reading at</span>
      <span class="text-neutral-200 tabular-nums font-medium">{{ timestamp }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Flame } from 'lucide-vue-next'

defineProps<{
  total: string
  timestamp?: string | null
}>()
</script>

<style scoped>
.glass-card-red {
  background: linear-gradient(135deg, rgba(239, 68, 68, 0.08), rgba(239, 68, 68, 0.02));
  border: 1px solid rgba(239, 68, 68, 0.12);
}
</style>
