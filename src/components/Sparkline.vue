<template>
  <svg :width="width" :height="height">
    <defs>
      <linearGradient :id="gradId" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" :stop-color="lineColor" stop-opacity="0.3" />
        <stop offset="100%" :stop-color="lineColor" stop-opacity="0" />
      </linearGradient>
    </defs>
    <path
      v-if="areaPath"
      :d="areaPath"
      :fill="`url(#${gradId})`"
      class="transition-all duration-500"
    />
    <polyline
      v-if="linePath"
      :points="linePath"
      fill="none"
      :stroke="lineColor"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
      class="transition-all duration-500"
    />
  </svg>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const MAX_POINTS = 30

const props = withDefaults(
  defineProps<{
    data: number[]
    width?: number
    height?: number
    color?: string
  }>(),
  {
    width: 120,
    height: 32,
    color: '#10B981',
  },
)

const gradId = computed(() => `sp-${Math.random().toString(36).slice(2, 8)}`)

const sampled = computed(() => {
  const d = props.data
  if (d.length <= MAX_POINTS) {
    return d
  }
  const step = d.length / MAX_POINTS
  return Array.from({ length: MAX_POINTS }, (_, i) => d[Math.floor(i * step)])
})

// For mini sparklines: use absolute values, color based on latest value
const absData = computed(() => sampled.value.map((v) => Math.abs(v)))
const lastValue = computed(() => sampled.value[sampled.value.length - 1] ?? 0)
const hasNegative = computed(() => sampled.value.some((v) => v < 0))

// Green if exporting (negative), purple if importing, default color otherwise
const lineColor = computed(() => {
  if (!hasNegative.value) {
    return props.color
  }
  return lastValue.value < 0 ? '#10B981' : '#A78BFA'
})

const coords = computed(() => {
  if (absData.value.length < 2) {
    return []
  }
  const max = Math.max(...absData.value)
  const min = Math.min(...absData.value)
  const range = max - min || 1
  const stepX = props.width / (absData.value.length - 1)
  return absData.value.map((v, i) => ({
    x: i * stepX,
    y: props.height - ((v - min) / range) * (props.height - 2) - 1,
  }))
})

const linePath = computed(() => {
  if (coords.value.length < 2) {
    return ''
  }
  return coords.value.map((c) => `${c.x},${c.y}`).join(' ')
})

const areaPath = computed(() => {
  if (coords.value.length < 2) {
    return ''
  }
  const pts = coords.value.map((c) => `${c.x},${c.y}`).join(' L ')
  return `M ${coords.value[0].x},${props.height} L ${pts} L ${coords.value[coords.value.length - 1].x},${props.height} Z`
})
</script>
