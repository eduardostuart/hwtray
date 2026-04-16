<template>
  <div
    class="rounded-xl px-4 py-3 cursor-pointer group bg-[linear-gradient(135deg,rgba(255,255,255,0.03),rgba(255,255,255,0.01))] border border-[rgba(255,255,255,0.06)]"
    @click="$emit('update:modelValue', !modelValue)"
  >
    <label class="flex items-center justify-between pointer-events-none">
      <div>
        <div
          class="text-[12px] font-medium text-neutral-300 group-hover:text-neutral-200 transition-colors"
        >
          {{ label }}
        </div>
        <p v-if="description" class="text-[9px] text-neutral-600 mt-0.5">{{ description }}</p>
      </div>
      <div
        :class="[
          'relative inline-flex h-5 w-9 items-center rounded-full transition-colors duration-200 flex-shrink-0 ml-3',
          modelValue ? 'bg-hw-green' : 'bg-neutral-700',
        ]"
      >
        <span
          :class="[
            'inline-block h-3.5 w-3.5 rounded-full bg-white transition-transform duration-200 shadow-sm',
            modelValue ? 'translate-x-[18px]' : 'translate-x-[3px]',
          ]"
        />
      </div>
    </label>
  </div>
</template>

<script setup lang="ts">
defineProps<{ label: string; description?: string; modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [value: boolean] }>()
</script>
