<template>
  <div class="flex flex-col items-center text-center pt-6">
    <img :src="logoUrl" class="w-20 h-20 rounded-2xl" alt="HWTray app icon" />
    <h1 class="text-xl font-bold text-white mt-4">HWTray</h1>
    <p class="text-sm text-neutral-500 mt-1">v0.1.0</p>
    <p class="text-[12px] text-neutral-600 mt-3">Unofficial app, not affiliated with HomeWizard</p>
    <p class="text-[13px] text-neutral-400 mt-1">
      by
      <a
        class="text-hw-green cursor-pointer hover:underline"
        @click="$emit('open-url', 'https://s.tuart.dev')"
        >Eduardo Stuart</a
      >
    </p>
  </div>
</template>

<script setup lang="ts">
import logoUrl from '@/assets/app-icon.png'

defineEmits<{ 'open-url': [url: string] }>()
</script>
