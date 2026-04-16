<template>
  <button
    :disabled="disabled"
    :class="[
      'hw-btn w-full font-semibold text-white disabled:opacity-40 disabled:cursor-not-allowed',
      small ? 'text-[11px] py-2 rounded-lg' : 'text-[12px] py-2.5 rounded-lg',
      variantClass,
    ]"
    @click.stop="$emit('click')"
  >
    <slot />
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(
  defineProps<{
    disabled?: boolean
    variant?: 'primary' | 'danger' | 'ghost'
    small?: boolean
  }>(),
  {
    variant: 'primary',
    small: false,
  },
)
defineEmits<{ click: [] }>()

const variantClass = computed(() => {
  if (props.disabled) {
    return 'hw-btn-disabled'
  }
  return `hw-btn-${props.variant}`
})
</script>

<!-- Button press animation with transform + box-shadow on :hover/:active:not(:disabled) requires CSS -->
<style scoped>
.hw-btn {
  cursor: pointer;
  transform: translateY(0);
  transition:
    transform 0.1s ease,
    box-shadow 0.1s ease;
  border: none;
}
.hw-btn:hover:not(:disabled) {
  transform: translateY(-1px);
}
.hw-btn:active:not(:disabled) {
  transform: translateY(2px);
  box-shadow: 0 1px 0 rgba(0, 0, 0, 0.2) !important;
}
.hw-btn-primary {
  background: #10b981;
  box-shadow: 0 3px 0 #065f46;
}
.hw-btn-danger {
  background: #6b7280;
  box-shadow: 0 3px 0 #374151;
}
.hw-btn-ghost {
  background: rgba(255, 255, 255, 0.06);
  box-shadow: 0 3px 0 rgba(255, 255, 255, 0.03);
}
.hw-btn-disabled {
  background: #1a1a1a;
  box-shadow: none;
}
</style>
