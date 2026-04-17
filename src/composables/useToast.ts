import { ref } from 'vue'

export type ToastKind = 'error'
export interface Toast {
  id: number
  message: string
  kind: ToastKind
}

const toasts = ref<Toast[]>([])
let nextId = 1
const TTL_MS = 4000

function dismiss(id: number) {
  const idx = toasts.value.findIndex((t) => t.id === id)
  if (idx >= 0) {
    toasts.value.splice(idx, 1)
  }
}

function push(message: string, kind: ToastKind) {
  const id = nextId++
  toasts.value.push({ id, message, kind })
  setTimeout(() => dismiss(id), TTL_MS)
}

export function useToast() {
  return {
    toasts,
    error: (msg: string) => push(msg, 'error'),
    dismiss,
  }
}

/** Test-only helper to reset module-level state between tests. */
export function _resetToasts() {
  toasts.value.splice(0, toasts.value.length)
  nextId = 1
}
