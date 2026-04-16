<template>
  <DevicePageLayout
    v-if="state"
    :title="state.device.name"
    :online="state.online"
    :ssid="state.data?.wifi_ssid"
    :signal="state.data?.wifi_strength"
    @back="router.push({ name: 'dashboard' })"
  >
    <template #hero>
      <PowerHero
        v-if="hw.hasPower(state.data)"
        :value="fmt(state.data?.active_power_w) ?? '—'"
        unit="W"
        :subtitle="hw.powerSubtitle(state.data)"
        :sparkline="state.sparkline"
        :active-tariff="state.data?.active_tariff"
        :import-kwh="fmt(state.data?.total_power_import_kwh)"
        :export-kwh="fmt(state.data?.total_power_export_kwh)"
      />

      <WaterHero
        v-if="hw.hasWater(state.data)"
        :flow="fmt(state.data?.active_liter_lpm) ?? '0'"
        :total="fmt(state.data?.total_liter_m3) ?? '0'"
        :sparkline="state.sparkline"
      />
    </template>

    <template #details>
      <template v-if="hw.hasPower(state.data) && hw.isThreePhase(state.data)">
        <MetricSection title="3-phase distribution">
          <MetricCard label="L1" :value="fmt(state.data?.active_power_l1_w)" unit="W" />
          <MetricCard label="L2" :value="fmt(state.data?.active_power_l2_w)" unit="W" />
          <MetricCard label="L3" :value="fmt(state.data?.active_power_l3_w)" unit="W" />
          <MetricCard
            v-if="state.data?.active_frequency_hz"
            label="Freq"
            :value="fmt(state.data?.active_frequency_hz)"
            unit="Hz"
          />
        </MetricSection>
      </template>

      <MetricSection v-if="hw.hasEnergy(state.data)" title="Import (from grid)">
        <MetricCard label="Total" :value="fmt(state.data?.total_power_import_kwh)" unit="kWh" />
        <MetricCard
          v-if="state.data?.total_power_import_t1_kwh"
          label="T1 (peak)"
          :value="fmt(state.data?.total_power_import_t1_kwh)"
          unit="kWh"
        />
        <MetricCard
          v-if="state.data?.total_power_import_t2_kwh"
          label="T2 (off-peak)"
          :value="fmt(state.data?.total_power_import_t2_kwh)"
          unit="kWh"
        />
      </MetricSection>

      <MetricSection v-if="hw.hasExport(state.data)" title="Export (to grid)">
        <MetricCard label="Total" :value="fmt(state.data?.total_power_export_kwh)" unit="kWh" />
        <MetricCard
          v-if="state.data?.total_power_export_t1_kwh"
          label="T1 (peak)"
          :value="fmt(state.data?.total_power_export_t1_kwh)"
          unit="kWh"
        />
        <MetricCard
          v-if="state.data?.total_power_export_t2_kwh"
          label="T2 (off-peak)"
          :value="fmt(state.data?.total_power_export_t2_kwh)"
          unit="kWh"
        />
      </MetricSection>

      <MetricSection
        v-if="hw.hasVoltage(state.data) && hw.isThreePhase(state.data)"
        title="Voltage per phase"
      >
        <MetricCard label="L1" :value="fmt(state.data?.active_voltage_l1_v)" unit="V" />
        <MetricCard label="L2" :value="fmt(state.data?.active_voltage_l2_v)" unit="V" />
        <MetricCard label="L3" :value="fmt(state.data?.active_voltage_l3_v)" unit="V" />
        <MetricCard
          v-if="state.data?.active_tariff"
          label="Tariff"
          :value="'T' + state.data?.active_tariff"
          unit=""
        />
      </MetricSection>

      <MetricSection
        v-if="hw.hasVoltage(state.data) && hw.isThreePhase(state.data)"
        title="Current per phase"
      >
        <MetricCard label="L1" :value="fmt(state.data?.active_current_l1_a, '', 2)" unit="A" />
        <MetricCard label="L2" :value="fmt(state.data?.active_current_l2_a, '', 2)" unit="A" />
        <MetricCard label="L3" :value="fmt(state.data?.active_current_l3_a, '', 2)" unit="A" />
        <MetricCard label="Total" :value="fmt(state.data?.active_current_a, '', 2)" unit="A" />
      </MetricSection>

      <MetricSection v-if="hw.hasVoltage(state.data) && !hw.isThreePhase(state.data)" title="Grid">
        <MetricCard
          label="Voltage"
          :value="fmt(state.data?.active_voltage_v ?? state.data?.active_voltage_l1_v)"
          unit="V"
        />
        <MetricCard
          label="Current"
          :value="fmt(state.data?.active_current_a ?? state.data?.active_current_l1_a)"
          unit="A"
        />
        <MetricCard
          v-if="state.data?.active_tariff"
          label="Tariff"
          :value="'T' + state.data?.active_tariff"
          unit=""
        />
      </MetricSection>

      <MetricSection v-if="hw.hasGas(state.data)" title="Gas">
        <MetricCard label="Total" :value="fmt(state.data?.total_gas_m3)" unit="m3" />
      </MetricSection>

      <MetricSection v-if="hw.hasWater(state.data)" title="Water details">
        <MetricCard label="Flow" :value="fmt(state.data?.active_liter_lpm)" unit="L/min" />
        <MetricCard label="Total" :value="fmt(state.data?.total_liter_m3)" unit="m3" />
        <MetricCard
          v-if="state.data?.total_liter_offset_m3 != null"
          label="Offset"
          :value="fmt(state.data?.total_liter_offset_m3)"
          unit="m3"
        />
      </MetricSection>

      <MetricSection v-if="hw.hasPowerFails(state.data)" title="Grid quality">
        <MetricCard label="Power fails" :value="state.data?.any_power_fail_count ?? null" unit="" />
        <MetricCard label="Long fails" :value="state.data?.long_power_fail_count ?? null" unit="" />
      </MetricSection>

      <MetricSection v-if="state.data?.meter_model" title="Meter">
        <MetricCard label="Model" :value="state.data.meter_model" unit="" />
        <MetricCard
          v-if="state.data?.smr_version"
          label="SMR"
          :value="state.data.smr_version"
          unit=""
        />
      </MetricSection>
    </template>
  </DevicePageLayout>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useDevicesStore } from '@/stores/devices'
import { useHomeWizard } from '@/composables/useHomeWizard'
import { fmt } from '@/utils/format'
import DevicePageLayout from '@/components/DevicePageLayout.vue'
import PowerHero from '@/components/PowerHero.vue'
import WaterHero from '@/components/WaterHero.vue'
import MetricSection from '@/components/MetricSection.vue'
import MetricCard from '@/components/MetricCard.vue'

const route = useRoute()
const router = useRouter()
const devicesStore = useDevicesStore()
const hw = useHomeWizard()
const state = computed(() => devicesStore.deviceStates[route.params.id as string])
</script>
