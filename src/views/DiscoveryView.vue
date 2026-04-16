<template>
  <DashboardLayout>
    <template #title> Discover Devices </template>
    <template #actions>
      <BackButton @click="goBack" />
    </template>

    <ScanningIndicator
      v-if="discoveryStore.isSearching"
      :countdown="countdown"
      :found="newDeviceCount"
    />

    <DeviceCheckboxItem
      v-for="device in discoveryStore.foundDevices"
      :key="device.id"
      :device="device"
      :selected="discoveryStore.selectedIds.has(device.id)"
      :subtitle="deviceSubtitle(device)"
      @toggle="discoveryStore.toggleSelection(device.id)"
    />

    <ButtonRow v-if="discoveryStore.foundDevices.length > 0 && discoveryStore.isSearching">
      <SearchButton :disabled="discoveryStore.selectedIds.size === 0" @click="saveAndReturn">
        Save selected
      </SearchButton>
      <SearchButton variant="danger" @click="stopSearch"> Stop scanning </SearchButton>
    </ButtonRow>

    <ButtonRow v-else-if="discoveryStore.foundDevices.length > 0">
      <SearchButton :disabled="discoveryStore.selectedIds.size === 0" @click="saveAndReturn">
        Save selected
      </SearchButton>
      <SearchButton variant="ghost" @click="startSearch"> Scan again </SearchButton>
    </ButtonRow>
  </DashboardLayout>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useDiscoveryStore, type DiscoveredDevice } from '@/stores/discovery'
import { useDiscoveryScanner } from '@/composables/useDiscoveryScanner'
import { ProductType } from '@/types/products'
import DashboardLayout from '@/components/DashboardLayout.vue'
import ButtonRow from '@/components/ButtonRow.vue'
import SearchButton from '@/components/SearchButton.vue'
import BackButton from '@/components/BackButton.vue'
import ScanningIndicator from '@/components/ScanningIndicator.vue'
import DeviceCheckboxItem from '@/components/DeviceCheckboxItem.vue'

const router = useRouter()
const discoveryStore = useDiscoveryStore()
const { countdown, newDeviceCount, startSearch, stopSearch, saveSelected } = useDiscoveryScanner()

async function saveAndReturn() {
  await saveSelected()
  router.push({ name: 'dashboard' })
}

function deviceSubtitle(device: DiscoveredDevice): string | undefined {
  if (device.product_type === ProductType.P1_METER) {
    return 'Includes gas meter if connected'
  }
  return undefined
}

function goBack() {
  stopSearch()
  router.push({ name: 'dashboard' })
}
</script>
