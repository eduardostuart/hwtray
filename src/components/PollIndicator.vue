<template>
  <svg class="opacity-70" width="16" height="16" viewBox="0 0 20 20">
    <circle cx="10" cy="10" r="8" fill="none" stroke="rgba(255,255,255,0.06)" stroke-width="2" />
    <circle
      class="transition-none"
      cx="10"
      cy="10"
      r="8"
      fill="none"
      stroke="#10B981"
      stroke-width="2"
      stroke-linecap="round"
      :stroke-dasharray="circumference"
      :stroke-dashoffset="offset"
      transform="rotate(-90 10 10)"
    />
  </svg>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

const props = defineProps<{ intervalSecs: number }>()

const circumference = 2 * Math.PI * 8
const progress = ref(1)
let startTime = Date.now()
let raf = 0

const offset = computed(() => circumference * (1 - progress.value))

function tick() {
  const elapsed = (Date.now() - startTime) / 1000
  const intervalMs = props.intervalSecs
  progress.value = Math.max(0, 1 - (elapsed % intervalMs) / intervalMs)
  raf = requestAnimationFrame(tick)
}

onMounted(() => {
  startTime = Date.now()
  raf = requestAnimationFrame(tick)
})

onUnmounted(() => cancelAnimationFrame(raf))
</script>
