<template>
  <div class="dashboard-bg flex h-full flex-col">
    <header
      v-if="$slots.title || $slots.status || $slots.actions"
      class="relative flex items-center justify-center px-5 pt-4 pb-3"
    >
      <div class="absolute left-5 flex items-center gap-2">
        <slot name="status" />
      </div>
      <h1 class="text-sm font-semibold tracking-wide text-neutral-300">
        <slot name="title" />
      </h1>
      <div class="absolute right-5 flex items-center gap-3">
        <slot name="actions" />
      </div>
    </header>
    <main class="flex-1 overflow-y-auto px-5 pt-4 pb-5 space-y-3">
      <slot />
    </main>
    <footer
      class="dashboard-footer flex items-center justify-between px-5 py-2 border-t border-[rgba(255,255,255,0.03)]"
    >
      <span class="text-[10px] text-neutral-700 tracking-wider">HWTray</span>
      <div class="flex items-center gap-3">
        <span class="text-[10px] text-neutral-700 tabular-nums">v{{ appVersion }}</span>
        <slot name="footer" />
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { getVersion } from '@tauri-apps/api/app'

const appVersion = ref('—')

onMounted(async () => {
  try {
    appVersion.value = await getVersion()
  } catch {
    // Fallback for dev/test
  }
})
</script>

<style scoped>
.dashboard-bg {
  background: linear-gradient(180deg, #0a0f0d 0%, #0d1117 40%, #0a0e12 100%);
}
.dashboard-footer {
  background: linear-gradient(180deg, transparent, rgba(16, 185, 129, 0.02));
}
</style>
