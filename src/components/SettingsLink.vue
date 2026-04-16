<template>
  <RouterLink
    :to="{ name: 'settings' }"
    class="text-neutral-500 transition-colors duration-200 hover:text-neutral-300"
  >
    <SettingsIcon :size="16" />
  </RouterLink>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router'
import { Settings as SettingsIcon } from 'lucide-vue-next'
</script>
