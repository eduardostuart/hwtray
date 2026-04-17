<template>
  <button
    class="w-full rounded-lg py-3 text-[13px] font-medium text-neutral-500 hover:text-neutral-300 cursor-pointer transition-colors border border-dashed border-[rgba(255,255,255,0.08)]"
    @click="$emit('click')"
  >
    <slot />
  </button>
</template>

<script setup lang="ts">
defineEmits<{ click: [] }>()
</script>
