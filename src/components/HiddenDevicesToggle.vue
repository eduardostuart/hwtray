<template>
  <button
    v-if="count > 0"
    class="w-full text-[10px] text-neutral-600 hover:text-neutral-400 transition-colors py-2 cursor-pointer"
    @click="$emit('toggle')"
  >
    <span v-if="expanded">
      <EyeOff :size="10" class="inline -mt-0.5" />
      Hide {{ count }} hidden device{{ count > 1 ? 's' : '' }}
    </span>
    <span v-else>
      <Eye :size="10" class="inline -mt-0.5" />
      Show {{ count }} hidden device{{ count > 1 ? 's' : '' }}
    </span>
  </button>
</template>

<script setup lang="ts">
import { Eye, EyeOff } from 'lucide-vue-next'

defineProps<{
  count: number
  expanded: boolean
}>()

defineEmits<{ toggle: [] }>()
</script>
