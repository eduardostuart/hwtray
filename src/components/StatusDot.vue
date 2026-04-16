<template>
  <span
    :class="[
      'inline-block w-1.5 h-1.5 rounded-full',
      online ? 'bg-hw-green animate-pulse-green' : 'bg-hw-red',
    ]"
  />
</template>

<script setup lang="ts">
defineProps<{ online: boolean }>()
</script>
