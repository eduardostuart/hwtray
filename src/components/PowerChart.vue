<template>
  <div v-if="data.length > 1" class="mt-3 -mx-4 -mb-4">
    <Line :data="chartData" :options="chartOptions" class="h-[80px]" />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Line } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Filler,
  Tooltip,
} from 'chart.js'
import type { ScriptableContext, TooltipItem } from 'chart.js'

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Filler, Tooltip)

const props = defineProps<{
  data: number[]
}>()

const MAX_POINTS = 20

function downsample(arr: number[]): number[] {
  if (arr.length <= MAX_POINTS) {
    return arr
  }
  const step = arr.length / MAX_POINTS
  return Array.from({ length: MAX_POINTS }, (_, i) => arr[Math.floor(i * step)])
}

const sampled = computed(() => downsample(props.data))
const hasExport = computed(() => sampled.value.some((v) => v < 0))

const gridData = computed(() => sampled.value.map((v) => (v > 0 ? v : 0)))
const surplusData = computed(() => sampled.value.map((v) => (v < 0 ? v : 0)))

const chartData = computed(() => ({
  labels: sampled.value.map(() => ''),
  datasets: [
    {
      label: 'Grid',
      data: gridData.value,
      borderColor: '#A78BFA',
      borderWidth: 1.5,
      backgroundColor: (ctx: ScriptableContext<'line'>) => {
        const chart = ctx.chart
        const { ctx: c, chartArea } = chart
        if (!chartArea) {
          return 'transparent'
        }
        const g = c.createLinearGradient(0, chartArea.top, 0, chartArea.bottom)
        g.addColorStop(0, 'rgba(167,139,250,0.25)')
        g.addColorStop(1, 'rgba(167,139,250,0)')
        return g
      },
      fill: 'origin',
      tension: 0.3,
      pointRadius: 0,
      pointHitRadius: 8,
      order: 1,
    },
    // Surplus export (green) — below zero
    ...(hasExport.value
      ? [
          {
            label: 'Surplus',
            data: surplusData.value,
            borderColor: '#10B981',
            borderWidth: 1.5,
            backgroundColor: (ctx: ScriptableContext<'line'>) => {
              const chart = ctx.chart
              const { ctx: c, chartArea } = chart
              if (!chartArea) {
                return 'transparent'
              }
              const g = c.createLinearGradient(0, chartArea.top, 0, chartArea.bottom)
              g.addColorStop(0, 'rgba(16,185,129,0)')
              g.addColorStop(1, 'rgba(16,185,129,0.25)')
              return g
            },
            fill: 'origin',
            tension: 0.3,
            pointRadius: 0,
            pointHitRadius: 8,
            order: 1,
          },
        ]
      : []),
  ],
}))

const chartOptions = computed(() => ({
  responsive: true,
  maintainAspectRatio: false,
  animation: { duration: 600, easing: 'easeOutCubic' as const },
  transitions: { active: { animation: { duration: 200 } } },
  interaction: { intersect: false, mode: 'index' as const },
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: '#1a1a1a',
      bodyColor: '#f5f5f5',
      bodyFont: { family: 'JetBrains Mono', size: 11, weight: 'normal' as const },
      borderColor: '#333',
      borderWidth: 1,
      cornerRadius: 6,
      padding: 8,
      displayColors: true,
      boxWidth: 8,
      boxHeight: 8,
      callbacks: {
        title: () => '',
        label: (ctx: TooltipItem<'line'>) => {
          const v = ctx.parsed.y ?? 0
          if (v === 0 && ctx.dataset.label !== 'Home') {
            return ''
          }
          return ` ${ctx.dataset.label}: ${Math.abs(v).toFixed(0)} W`
        },
      },
      filter: (item: TooltipItem<'line'>) => {
        if (item.dataset.label === 'Home') {
          return (item.raw as number) > 0
        }
        return item.raw !== 0
      },
    },
  },
  scales: {
    x: { display: false },
    y: {
      display: true,
      position: 'right' as const,
      grid: { color: 'rgba(255,255,255,0.03)', drawTicks: false },
      border: { display: false },
      // Always include zero so surplus (negative) goes below the line
      suggestedMin: hasExport.value ? undefined : 0,
      suggestedMax: hasExport.value ? undefined : undefined,
      ticks: {
        color: '#525252',
        font: { family: 'JetBrains Mono', size: 8 },
        padding: 6,
        maxTicksLimit: 5,
        callback: (value: string | number) => `${value}`,
      },
    },
  },
}))
</script>
