import { ref, onMounted, onUnmounted, type Ref } from 'vue'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { PhysicalSize } from '@tauri-apps/api/dpi'

const WINDOW_WIDTH = 400
const MIN_HEIGHT = 450
const MAX_HEIGHT = 800

// Automatically resizes the Tauri window height to match the content.
// Returns a template ref to attach to the content container.
export function useAutoResize(extraHeight = 50): Ref<HTMLElement | null> {
  const contentRef = ref<HTMLElement | null>(null)
  let observer: ResizeObserver | null = null

  async function sync() {
    if (!contentRef.value) {
      return
    }

    const height = Math.min(
      MAX_HEIGHT,
      Math.max(MIN_HEIGHT, contentRef.value.scrollHeight + extraHeight),
    )

    try {
      const win = getCurrentWebviewWindow()
      await win.setSize(
        new PhysicalSize(WINDOW_WIDTH * window.devicePixelRatio, height * window.devicePixelRatio),
      )
    } catch {
      // Outside Tauri runtime
    }
  }

  onMounted(() => {
    if (!contentRef.value) {
      return
    }
    observer = new ResizeObserver(() => sync())
    observer.observe(contentRef.value)
    sync()
  })

  onUnmounted(() => {
    observer?.disconnect()
  })

  return contentRef
}
