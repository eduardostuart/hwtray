import { ref, watch } from 'vue'

export function useAnimatedNumber(source: () => number, duration = 500) {
  const display = ref(source())
  let raf = 0

  watch(source, (to) => {
    const from = display.value
    const start = performance.now()
    cancelAnimationFrame(raf)

    function tick(now: number) {
      const t = Math.min((now - start) / duration, 1)
      const ease = t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t
      display.value = Math.round(from + (to - from) * ease)
      if (t < 1) {
        raf = requestAnimationFrame(tick)
      }
    }
    raf = requestAnimationFrame(tick)
  })

  return display
}
