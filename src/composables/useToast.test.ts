import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { useToast, _resetToasts } from './useToast'

describe('useToast', () => {
  beforeEach(() => {
    vi.useFakeTimers()
    _resetToasts()
  })
  afterEach(() => {
    vi.useRealTimers()
  })

  it('error() pushes a toast with error kind', () => {
    const { toasts, error } = useToast()
    error('Boom')
    expect(toasts.value).toHaveLength(1)
    expect(toasts.value[0].message).toBe('Boom')
    expect(toasts.value[0].kind).toBe('error')
  })

  it('auto-dismisses after 4s', () => {
    const { toasts, error } = useToast()
    error('Boom')
    vi.advanceTimersByTime(4001)
    expect(toasts.value).toHaveLength(0)
  })

  it('dismiss(id) removes the toast immediately', () => {
    const { toasts, error, dismiss } = useToast()
    error('Boom')
    dismiss(toasts.value[0].id)
    expect(toasts.value).toHaveLength(0)
  })
})
