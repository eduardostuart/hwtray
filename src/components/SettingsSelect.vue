<template>
  <div
    class="rounded-xl px-4 py-3 bg-[linear-gradient(135deg,rgba(255,255,255,0.03),rgba(255,255,255,0.01))] border border-[rgba(255,255,255,0.06)]"
  >
    <label class="flex items-center justify-between">
      <div>
        <div class="text-[12px] font-medium text-neutral-300">{{ label }}</div>
        <p v-if="description" class="text-[9px] text-neutral-600 mt-0.5">{{ description }}</p>
      </div>
      <BaseSelect
        :model-value="modelValue"
        class="ml-3 flex-shrink-0 !w-auto !text-[11px] !py-1.5"
        @update:model-value="$emit('update:modelValue', $event)"
      >
        <option v-for="opt in options" :key="opt.value" :value="opt.value">
          {{ opt.label }}
        </option>
      </BaseSelect>
    </label>
  </div>
</template>

<script setup lang="ts">
import BaseSelect from './BaseSelect.vue'

defineProps<{
  label: string
  description?: string
  modelValue: string | number
  options: Array<{ value: string | number; label: string }>
}>()
defineEmits<{ 'update:modelValue': [value: string | number] }>()
</script>
