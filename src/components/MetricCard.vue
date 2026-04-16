<template>
  <div
    class="rounded-xl px-3 py-2.5 bg-[linear-gradient(135deg,rgba(255,255,255,0.03),rgba(255,255,255,0.01))] border border-[rgba(255,255,255,0.05)]"
  >
    <div class="text-[9px] uppercase tracking-widest text-neutral-500 font-medium">
      {{ label }}
    </div>
    <div class="mt-1 flex items-baseline gap-1">
      <span class="text-[17px] font-semibold text-neutral-100 tabular-nums">
        {{ value ?? '—' }}
      </span>
      <span v-if="value != null" class="text-[10px] text-neutral-500 font-medium">{{ unit }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  label: string
  value: string | number | null
  unit: string
}>()
</script>
