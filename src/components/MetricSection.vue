<template>
  <div class="space-y-2">
    <div class="flex items-center gap-2">
      <h3 class="text-[10px] uppercase tracking-widest text-neutral-600 font-semibold">
        {{ title }}
      </h3>
      <div class="flex-1 h-px bg-neutral-800/50" />
    </div>
    <div class="grid grid-cols-2 gap-2">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{ title: string }>()
</script>
