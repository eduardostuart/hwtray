<template>
  <DevicePageLayout
    v-if="hw.gasDevice.value"
    title="Gas"
    :online="hw.gasDevice.value.online"
    :ssid="hw.gasDevice.value.data?.wifi_ssid"
    :signal="hw.gasDevice.value.data?.wifi_strength"
    @back="router.push({ name: 'dashboard' })"
  >
    <template #hero>
      <GasHero
        :total="fmt(hw.gasDevice.value.data?.total_gas_m3) ?? '—'"
        :timestamp="hw.gasTimestamp.value"
      />
    </template>

    <template #details />
  </DevicePageLayout>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useHomeWizard } from '@/composables/useHomeWizard'
import { fmt } from '@/utils/format'
import DevicePageLayout from '@/components/DevicePageLayout.vue'
import GasHero from '@/components/GasHero.vue'

const router = useRouter()
const hw = useHomeWizard()
</script>
