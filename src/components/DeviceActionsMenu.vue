<template>
  <button
    ref="triggerEl"
    class="ml-2 text-neutral-600 hover:text-neutral-300 cursor-pointer transition-colors"
    :class="{ 'text-neutral-300': open }"
    @click.stop="toggle"
  >
    <MoreVertical :size="14" />
  </button>

  <Teleport to="body">
    <div
      v-if="open"
      ref="menuEl"
      data-testid="device-actions-menu"
      class="fixed z-[9999] min-w-[160px] rounded-lg border border-neutral-800 bg-neutral-900 py-1 shadow-xl text-[12px] text-neutral-200"
      :style="menuStyle"
      @mousedown.stop
      @click.stop
    >
      <button
        v-if="productType === 'HWE-SKT'"
        data-action="identify"
        class="flex w-full items-center gap-2 px-3 py-1.5 text-left hover:bg-neutral-800"
        @click="fire('identify')"
      >
        <Radio :size="12" /> Identify
      </button>
      <button
        data-action="rename"
        class="flex w-full items-center gap-2 px-3 py-1.5 text-left hover:bg-neutral-800"
        @click="fire('rename')"
      >
        <Pencil :size="12" /> Rename
      </button>
      <button
        data-action="hide"
        class="flex w-full items-center gap-2 px-3 py-1.5 text-left hover:bg-neutral-800"
        @click="fire('hide')"
      >
        <EyeOff :size="12" /> Hide
      </button>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, reactive, onUnmounted, nextTick } from 'vue'
import { MoreVertical, Radio, Pencil, EyeOff } from 'lucide-vue-next'
import { computeMenuPosition } from './deviceActionsMenuPosition'

const props = defineProps<{
  deviceId: string
  productType: string
  online: boolean
}>()

const emit = defineEmits<{
  identify: []
  rename: []
  hide: []
}>()

const open = ref(false)
const triggerEl = ref<HTMLButtonElement | null>(null)
const menuEl = ref<HTMLElement | null>(null)
const menuStyle = reactive({ top: '0px', left: '0px' })

function currentViewport() {
  // Prefer the main content container (data-content-root) so the menu stays
  // inside the visible rounded area instead of the padded window buffer.
  const root = document.querySelector<HTMLElement>('[data-content-root]')
  if (root) {
    const r = root.getBoundingClientRect()
    return { top: r.top, left: r.left, right: r.right, bottom: r.bottom }
  }
  return { top: 0, left: 0, right: window.innerWidth, bottom: window.innerHeight }
}

async function positionMenu() {
  if (!triggerEl.value) {
    return
  }
  const rect = triggerEl.value.getBoundingClientRect()
  const vp = currentViewport()

  // Initial position with estimated size (avoids a visible jump).
  const estHeight = props.productType === 'HWE-SKT' ? 100 : 72
  const initial = computeMenuPosition(rect, 160, estHeight, vp)
  menuStyle.top = `${initial.top}px`
  menuStyle.left = `${initial.left}px`

  // Refine after mount using the actual rendered size.
  await nextTick()
  if (!menuEl.value || !triggerEl.value) {
    return
  }
  const refined = computeMenuPosition(
    triggerEl.value.getBoundingClientRect(),
    menuEl.value.offsetWidth,
    menuEl.value.offsetHeight,
    currentViewport(),
  )
  menuStyle.top = `${refined.top}px`
  menuStyle.left = `${refined.left}px`
}

function toggle() {
  if (open.value) {
    close()
  } else {
    open.value = true
    void positionMenu()
    document.addEventListener('keydown', onKeydown)
    document.addEventListener('mousedown', onOutside)
  }
}

function close() {
  open.value = false
  document.removeEventListener('keydown', onKeydown)
  document.removeEventListener('mousedown', onOutside)
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    close()
  }
}

function onOutside() {
  close()
}

function fire(action: 'identify' | 'rename' | 'hide') {
  if (action === 'identify') {
    emit('identify')
  } else if (action === 'rename') {
    emit('rename')
  } else {
    emit('hide')
  }
  close()
}

onUnmounted(close)
</script>
