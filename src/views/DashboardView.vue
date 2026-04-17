<template>
  <DashboardLayout>
    <template #footer>
      <PollIndicator v-if="showPollIndicator" :interval-secs="pollIntervalSecs" />
      <SettingsLink />
    </template>

    <template v-if="hw.deviceList.value.length === 0">
      <AppVersion />
      <SearchButton @click="router.push({ name: 'discovery' })">
        Search for devices...
      </SearchButton>
    </template>
    <template v-else-if="hw.allOffline.value">
      <OfflineState />
    </template>
    <template v-else>
      <PowerSummary
        v-if="hw.hasPowerDevice.value"
        :total-power="hw.totalPower.value"
        :total-import="hw.totalImport.value"
        :total-export="hw.totalExport.value"
        :sparkline="hw.mainSparkline.value"
        :active-tariff="hw.activeTariff.value"
      />

      <DraggableList v-model="currentOrder" @update:model-value="onReorder">
        <div v-for="item in visibleItems" :key="item.id" :data-id="item.id" class="mb-2">
          <DeviceListItem
            :id="item.id"
            :name="item.name"
            :product-type="item.productType"
            :online="item.online"
            :metric-value="item.metricValue"
            :metric-unit="item.metricUnit"
            :secondary="item.secondary"
            :editing="editingId === item.id"
            :pulsing="devicesStore.identifyingIds.has(item.id)"
            draggable
            hideable
            @click="
              editingId === item.id
                ? null
                : router.push({ name: item.route, params: item.routeParams })
            "
            @identify="onIdentify(item.id)"
            @rename-start="onRenameStart(item.id)"
            @hide="hideItem(item.id)"
            @rename="(newName: string) => onRenameSave(item.id, newName)"
            @rename-cancel="onRenameCancel"
          />
        </div>
      </DraggableList>

      <HiddenDevicesToggle
        :count="hiddenCount"
        :expanded="showHidden"
        @toggle="showHidden = !showHidden"
      />

      <template v-if="showHidden">
        <DeviceListItem
          v-for="item in hiddenItems"
          :id="item.id"
          :key="item.id"
          :name="item.name"
          :product-type="item.productType"
          :online="item.online"
          :metric-value="item.metricValue"
          :metric-unit="item.metricUnit"
          :secondary="item.secondary"
          @click="unhideItem(item.id)"
        />
      </template>
    </template>
  </DashboardLayout>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { useHomeWizard } from '@/composables/useHomeWizard'
import { useDeviceActions } from '@/composables/useDeviceActions'
import { useToast } from '@/composables/useToast'
import { useSettingsStore } from '@/stores/settings'
import { useDevicesStore } from '@/stores/devices'
import DashboardLayout from '@/components/DashboardLayout.vue'
import PollIndicator from '@/components/PollIndicator.vue'
import SettingsLink from '@/components/SettingsLink.vue'
import DraggableList from '@/components/DraggableList.vue'
import DeviceListItem from '@/components/DeviceListItem.vue'
import PowerSummary from '@/components/PowerSummary.vue'
import SearchButton from '@/components/SearchButton.vue'
import AppVersion from '@/components/AppVersion.vue'
import OfflineState from '@/components/OfflineState.vue'
import HiddenDevicesToggle from '@/components/HiddenDevicesToggle.vue'

const router = useRouter()
const settingsStore = useSettingsStore()
const devicesStore = useDevicesStore()
const hw = useHomeWizard()
const { identifyDevice } = useDeviceActions()
const { error: toastError } = useToast()

const editingId = ref<string | null>(null)

function onRenameStart(id: string) {
  editingId.value = id
}

function onIdentify(id: string) {
  identifyDevice(id)
}

function onRenameSave(id: string, name: string) {
  editingId.value = null
  devicesStore.renameDevice(id, name).catch(() => {
    toastError('Não foi possível renomear o dispositivo')
  })
}

function onRenameCancel() {
  editingId.value = null
}

// Ordering: respect user-defined order, append new items at the end
const orderedItems = computed(() => {
  const items = hw.dashboardItems.value
  const order = settingsStore.dashboardOrder
  if (order.length === 0) {
    return items
  }
  const byId = new Map(items.map((i) => [i.id, i]))
  const ordered = []
  for (const id of order) {
    const item = byId.get(id)
    if (item) {
      ordered.push(item)
      byId.delete(id)
    }
  }
  for (const item of byId.values()) {
    ordered.push(item)
  }
  return ordered
})

const visibleItems = computed(() => orderedItems.value.filter((i) => !settingsStore.isHidden(i.id)))
const hiddenItems = computed(() => orderedItems.value.filter((i) => settingsStore.isHidden(i.id)))
const hiddenCount = computed(() => hiddenItems.value.length)
const showHidden = ref(false)
const currentOrder = computed(() => orderedItems.value.map((i) => i.id))

const showPollIndicator = computed(
  () =>
    hw.dashboardItems.value.length > 0 &&
    settingsStore.showPollIndicator &&
    settingsStore.pollIntervalMs >= 5000,
)
const pollIntervalSecs = computed(() => settingsStore.pollIntervalMs / 1000)

async function saveSettings() {
  try {
    await invoke('update_settings', { settings: settingsStore.toSettings() })
  } catch {
    /* noop */
  }
}

async function onReorder(order: string[]) {
  settingsStore.dashboardOrder = order
  await saveSettings()
}

async function hideItem(id: string) {
  settingsStore.toggleHidden(id)
  await saveSettings()
}

async function unhideItem(id: string) {
  settingsStore.toggleHidden(id)
  await saveSettings()
}
</script>
