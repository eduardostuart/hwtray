<template>
  <div class="window-bg h-screen flex flex-col rounded-xl overflow-hidden" data-tauri-drag-region>
    <div class="flex-shrink-0 px-5 pt-3 pb-3" data-tauri-drag-region>
      <div class="flex items-center" data-tauri-drag-region>
        <WindowControls />
        <h1
          v-if="$slots.title"
          class="flex-1 text-center text-sm font-semibold tracking-wide text-neutral-300 pointer-events-none -ml-[52px]"
        >
          <slot name="title" />
        </h1>
      </div>
    </div>
    <div class="border-t border-[rgba(255,255,255,0.06)]" />
    <div class="flex-1 overflow-y-auto px-5 py-5 space-y-4">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
import WindowControls from './WindowControls.vue'
</script>

<style scoped>
.window-bg {
  background: linear-gradient(180deg, #0a0f0d 0%, #0d1117 40%, #0a0e12 100%);
}
</style>
