<template>
  <select
    :value="modelValue"
    class="base-select appearance-none rounded-lg bg-neutral-800/80 w-full px-3 py-2 pr-7 text-[12px] font-medium text-neutral-200 outline-none cursor-pointer transition-colors hover:bg-neutral-700/80"
    @change="$emit('update:modelValue', ($event.target as HTMLSelectElement).value)"
  >
    <slot />
  </select>
</template>

<script setup lang="ts">
defineProps<{
  modelValue: string | number
}>()
defineEmits<{ 'update:modelValue': [value: string | number] }>()
</script>

<style scoped>
.base-select {
  border: 1px solid rgba(255, 255, 255, 0.08);
  background-image: url('data:image/svg+xml,%3Csvg width=%2710%27 height=%276%27 viewBox=%270 0 10 6%27 fill=%27none%27 xmlns=%27http://www.w3.org/2000/svg%27%3E%3Cpath d=%27M1 1l4 4 4-4%27 stroke=%27%23737373%27 stroke-width=%271.5%27 stroke-linecap=%27round%27 stroke-linejoin=%27round%27/%3E%3C/svg%3E');
  background-repeat: no-repeat;
  background-position: right 8px center;
}
</style>
