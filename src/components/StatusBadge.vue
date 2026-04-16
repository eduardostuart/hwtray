<template>
  <span class="inline-flex items-center">
    <span
      :class="['w-2 h-2 rounded-full', online ? 'bg-hw-green animate-pulse-green' : 'bg-hw-red']"
    />
  </span>
</template>

<script setup lang="ts">
defineProps<{ online: boolean }>()
</script>
