<template>
  <DashboardLayout>
    <template #status>
      <StatusDot :online="online" />
    </template>
    <template #title>
      {{ title }}
    </template>
    <template #actions>
      <BackButton @click="$emit('back')" />
    </template>

    <slot name="hero" />
    <slot name="details" />

    <MetricSection v-if="ssid" title="Network">
      <MetricCard label="SSID" :value="ssid" unit="" />
      <MetricCard v-if="signal != null" label="Signal" :value="signal" unit="%" />
    </MetricSection>
  </DashboardLayout>
</template>

<script setup lang="ts">
import DashboardLayout from './DashboardLayout.vue'
import StatusDot from './StatusDot.vue'
import BackButton from './BackButton.vue'
import MetricSection from './MetricSection.vue'
import MetricCard from './MetricCard.vue'

defineProps<{
  title: string
  online: boolean
  ssid?: string | null
  signal?: number | null
}>()

defineEmits<{ back: [] }>()
</script>
