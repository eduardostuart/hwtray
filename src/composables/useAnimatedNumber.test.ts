import { describe, it, expect } from 'vitest'
import { ref } from 'vue'
import { useAnimatedNumber } from './useAnimatedNumber'

describe('useAnimatedNumber', () => {
  it('returns a ref', () => {
    const source = ref(42)
    const display = useAnimatedNumber(() => source.value)
    expect(display).toBeDefined()
    expect(display.value).toBe(42)
  })

  it('initial value matches source value', () => {
    const source = ref(100)
    const display = useAnimatedNumber(() => source.value)
    expect(display.value).toBe(100)
  })

  it('initial value is zero when source is zero', () => {
    const source = ref(0)
    const display = useAnimatedNumber(() => source.value)
    expect(display.value).toBe(0)
  })

  it('accepts custom duration parameter', () => {
    const source = ref(50)
    const display = useAnimatedNumber(() => source.value, 1000)
    expect(display.value).toBe(50)
  })
})
