<template>
  <div v-if="isMainWindow" class="flex flex-col">
    <div class="relative h-[10px] flex-shrink-0">
      <div class="arrow" :style="{ left: arrowLeft + 'px' }" />
    </div>
    <div ref="contentRef" class="w-full overflow-clip rounded-xl bg-[#0d1117]">
      <RouterView />
    </div>
  </div>
  <div v-else class="h-full">
    <RouterView />
  </div>
</template>

<script setup lang="ts">
import { RouterView, useRoute, useRouter } from 'vue-router'
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useAutoResize } from '@/composables/useAutoResize'
import { useTauriEvents } from '@/composables/useTauriEvents'

const route = useRoute()
const router = useRouter()
const isMainWindow = computed(() => route.path !== '/about' && route.path !== '/tray-customize')
useTauriEvents()
const contentRef = useAutoResize()

const arrowLeft = ref(200)
const unlisteners: UnlistenFn[] = []

onMounted(async () => {
  try {
    arrowLeft.value = await invoke<number>('get_arrow_offset')
  } catch {
    /* first launch before tray click */
  }

  unlisteners.push(
    await listen<number>('arrow_offset', (event) => {
      arrowLeft.value = event.payload
    }),
  )
  unlisteners.push(
    await listen<string>('navigate', (event) => {
      router.push(event.payload)
    }),
  )
})

onUnmounted(() => {
  unlisteners.forEach((fn) => fn())
  unlisteners.length = 0
})
</script>

<style scoped>
.arrow {
  position: absolute;
  top: 0;
  width: 20px;
  height: 10px;
  margin-left: -10px;
  clip-path: polygon(50% 0%, 0% 100%, 100% 100%);
  background: #0d1117;
  z-index: 10;
}
</style>
