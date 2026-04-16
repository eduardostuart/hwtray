<template>
  <button
    class="flex items-center gap-1 text-[11px] font-medium text-neutral-500 hover:text-neutral-300 transition-colors duration-200 cursor-pointer"
    @click="$emit('click')"
  >
    <ArrowLeft :size="14" />
    Back
  </button>
</template>

<script setup lang="ts">
import { ArrowLeft } from 'lucide-vue-next'
defineEmits<{ click: [] }>()
</script>
