<template>
  <AboutLayout>
    <AboutHeader @open-url="openUrl" />
    <AboutDivider />
    <AboutLinks @open-url="openUrl" />
    <AboutDivider />
    <AboutLibs />
  </AboutLayout>
</template>

<script setup lang="ts">
import { openUrl as tauriOpen } from '@tauri-apps/plugin-opener'
import AboutLayout from '@/components/AboutLayout.vue'
import AboutHeader from '@/components/AboutHeader.vue'
import AboutLinks from '@/components/AboutLinks.vue'
import AboutLibs from '@/components/AboutLibs.vue'
import AboutDivider from '@/components/AboutDivider.vue'

async function openUrl(url: string) {
  try {
    await tauriOpen(url)
  } catch {
    window.open(url, '_blank')
  }
}
</script>
