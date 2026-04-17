<template>
  <Teleport to="body">
    <div class="pointer-events-none fixed bottom-4 right-4 z-[9999] flex flex-col gap-2">
      <div
        v-for="t in toasts"
        :key="t.id"
        class="pointer-events-auto rounded-lg border border-red-900/60 bg-red-950/90 px-3 py-2 text-[12px] text-red-100 shadow-xl"
      >
        {{ t.message }}
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { useToast } from '@/composables/useToast'
const { toasts } = useToast()
</script>
