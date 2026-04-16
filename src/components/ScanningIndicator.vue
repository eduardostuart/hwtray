<template>
  <div class="flex flex-col items-center py-8">
    <div class="relative w-16 h-16 mb-4">
      <!-- Rotating ring -->
      <svg class="w-16 h-16 animate-spin-slow" viewBox="0 0 64 64" fill="none">
        <circle cx="32" cy="32" r="28" stroke="rgba(255,255,255,0.06)" stroke-width="1.5" />
        <path
          d="M32 4a28 28 0 0 1 28 28"
          stroke="#10B981"
          stroke-width="1.5"
          stroke-linecap="round"
        />
      </svg>
      <!-- Pulsing radar circles -->
      <div class="absolute inset-0 flex items-center justify-center">
        <div class="absolute w-8 h-8 rounded-full border border-hw-green/20 animate-radar-1" />
        <div class="absolute w-8 h-8 rounded-full border border-hw-green/20 animate-radar-2" />
      </div>
      <!-- Center bolt -->
      <div class="absolute inset-0 flex items-center justify-center">
        <Zap :size="20" color="#10B981" />
      </div>
    </div>
    <p class="text-[12px] font-medium text-neutral-400 tabular-nums">
      Scanning network... {{ countdown }}s
    </p>
    <p class="text-[10px] text-neutral-600 mt-1">
      {{ found }} device{{ found === 1 ? '' : 's' }} found
    </p>
  </div>
</template>

<script setup lang="ts">
import { Zap } from 'lucide-vue-next'

defineProps<{
  countdown: number
  found: number
}>()
</script>

<style scoped>
@keyframes spin-slow {
  to {
    transform: rotate(360deg);
  }
}
.animate-spin-slow {
  animation: spin-slow 3s linear infinite;
}

@keyframes radar {
  0% {
    transform: scale(1);
    opacity: 0.4;
  }
  100% {
    transform: scale(3);
    opacity: 0;
  }
}
.animate-radar-1 {
  animation: radar 2s ease-out infinite;
}
.animate-radar-2 {
  animation: radar 2s ease-out infinite 1s;
}
</style>
