import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { useDeviceActions } from './useDeviceActions'
import { useDevicesStore } from '@/stores/devices'
import { _resetToasts, useToast } from './useToast'

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }))

describe('useDeviceActions.identifyDevice', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.useFakeTimers()
    _resetToasts()
    vi.mocked(invoke).mockReset()
  })
  afterEach(() => {
    vi.useRealTimers()
  })

  it('adds id to identifyingIds immediately and removes after 3s', async () => {
    vi.mocked(invoke).mockResolvedValueOnce(undefined)
    const store = useDevicesStore()
    const { identifyDevice } = useDeviceActions()

    const promise = identifyDevice('abc')
    expect(store.identifyingIds.has('abc')).toBe(true)
    await promise
    expect(store.identifyingIds.has('abc')).toBe(true)
    vi.advanceTimersByTime(3001)
    expect(store.identifyingIds.has('abc')).toBe(false)
  })

  it('invokes identify_device with id', async () => {
    vi.mocked(invoke).mockResolvedValueOnce(undefined)
    const { identifyDevice } = useDeviceActions()
    await identifyDevice('abc')
    expect(invoke).toHaveBeenCalledWith('identify_device', { id: 'abc' })
  })

  it('shows error toast on IPC failure', async () => {
    vi.mocked(invoke).mockRejectedValueOnce('offline')
    const { identifyDevice } = useDeviceActions()
    const { toasts } = useToast()
    await identifyDevice('abc')
    expect(toasts.value).toHaveLength(1)
    expect(toasts.value[0].kind).toBe('error')
  })
})
