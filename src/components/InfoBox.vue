<template>
  <div :class="['info-box rounded-xl px-4 py-3 flex gap-3', variantClass]">
    <div class="flex-shrink-0 mt-0.5">
      <div
        :class="[
          'w-4 h-4 rounded-full flex items-center justify-center text-[9px] font-bold',
          iconClass,
        ]"
      >
        <Info v-if="variant === 'info'" :size="10" />
        <AlertTriangle v-else-if="variant === 'warning'" :size="10" />
        <AlertCircle v-else-if="variant === 'error'" :size="10" />
      </div>
    </div>
    <p class="text-[12px] leading-relaxed" :class="textClass">
      <slot />
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Info, AlertTriangle, AlertCircle } from 'lucide-vue-next'

const props = withDefaults(
  defineProps<{
    variant?: 'info' | 'warning' | 'error'
  }>(),
  { variant: 'info' },
)

const variantClass = computed(() => `info-box--${props.variant}`)
const iconClass = computed(() => {
  const map = {
    info: 'bg-[rgba(139,92,246,0.2)] text-[#a78bfa]',
    warning: 'bg-[rgba(245,158,11,0.2)] text-[#f59e0b]',
    error: 'bg-[rgba(239,68,68,0.2)] text-[#ef4444]',
  }
  return map[props.variant]
})
const textClass = computed(() => {
  const map = {
    info: 'text-[#c4b5fd]',
    warning: 'text-[#fcd34d]',
    error: 'text-[#fca5a5]',
  }
  return map[props.variant]
})
</script>

<style scoped>
.info-box--info {
  background: rgba(139, 92, 246, 0.06);
  border: 1px solid rgba(139, 92, 246, 0.12);
}
.info-box--warning {
  background: rgba(245, 158, 11, 0.06);
  border: 1px solid rgba(245, 158, 11, 0.12);
}
.info-box--error {
  background: rgba(239, 68, 68, 0.06);
  border: 1px solid rgba(239, 68, 68, 0.12);
}
</style>
