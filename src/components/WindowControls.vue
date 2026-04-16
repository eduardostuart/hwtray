<template>
  <div class="flex gap-2 py-0.5" data-tauri-drag-region>
    <button
      class="w-3 h-3 rounded-full border-none flex items-center justify-center p-0 bg-[#ff5f57] cursor-pointer text-transparent transition-colors duration-150 hover:text-[rgba(0,0,0,0.6)]"
      @click="onClose"
    >
      <X :size="8" />
    </button>
    <span
      class="w-3 h-3 rounded-full border-none flex items-center justify-center p-0 bg-[rgba(255,255,255,0.08)]"
    />
    <span
      class="w-3 h-3 rounded-full border-none flex items-center justify-center p-0 bg-[rgba(255,255,255,0.08)]"
    />
  </div>
</template>

<script setup lang="ts">
import { X } from 'lucide-vue-next'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

async function onClose() {
  try {
    await getCurrentWebviewWindow().hide()
  } catch {
    // Window may already be hidden
  }
}
</script>
