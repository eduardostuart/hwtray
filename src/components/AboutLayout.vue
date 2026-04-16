<template>
  <WindowLayout>
    <template #title>About</template>
    <div class="flex flex-col px-2">
      <slot />
    </div>
  </WindowLayout>
</template>

<script setup lang="ts">
import WindowLayout from './WindowLayout.vue'
</script>
