<template>
  <div ref="containerRef">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, nextTick } from 'vue'
import Sortable from 'sortablejs'

const props = defineProps<{
  modelValue: string[]
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string[]]
}>()

const containerRef = ref<HTMLElement | null>(null)
let sortable: Sortable | null = null
let dragging = false

function initSortable() {
  if (!containerRef.value || dragging) {
    return
  }
  if (sortable) {
    sortable.destroy()
  }

  sortable = Sortable.create(containerRef.value, {
    animation: 250,
    handle: '.drag-handle',
    ghostClass: 'drag-ghost',
    chosenClass: 'drag-chosen',
    dragClass: 'drag-active',
    easing: 'cubic-bezier(0.25, 1, 0.5, 1)',
    forceFallback: true,
    fallbackClass: 'drag-fallback',
    fallbackOnBody: false,
    fallbackTolerance: 3,
    onStart() {
      dragging = true
    },
    onEnd(evt) {
      dragging = false

      if (evt.oldIndex === undefined || evt.newIndex === undefined) {
        return
      }
      if (evt.oldIndex === evt.newIndex) {
        return
      }

      if (!containerRef.value) {
        return
      }
      const children = Array.from(containerRef.value.children)
      const newOrder = children
        .map((el) => (el as HTMLElement).dataset.id)
        .filter((id): id is string => !!id)

      emit('update:modelValue', newOrder)
    },
  })
}

onMounted(() => {
  nextTick(initSortable)
})

watch(
  () => props.modelValue,
  () => {
    nextTick(initSortable)
  },
)
</script>

<style>
.drag-ghost {
  opacity: 0.15;
  border-radius: 1rem;
  outline: 2px dashed rgba(16, 185, 129, 0.3);
  outline-offset: -2px;
}

.drag-chosen {
  z-index: 50;
}

.drag-active {
  opacity: 0 !important;
}

.drag-fallback {
  opacity: 1 !important;
  background: #0d1117;
  border-radius: 1rem;
  overflow: hidden;
  z-index: 100;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.6);
  width: calc(100% - 2.5rem) !important;
}
</style>
