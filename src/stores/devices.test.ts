import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useDevicesStore } from './devices'

describe('useDevicesStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('starts with empty devices', () => {
    const store = useDevicesStore()
    expect(store.deviceStates).toEqual({})
  })

  it('sets devices from saved list', () => {
    const store = useDevicesStore()
    store.setDevices([{ id: '1', name: 'P1', product_type: 'HWE-P1', ip: '192.168.1.1', port: 80 }])
    expect(store.deviceStates['1']).toBeDefined()
    expect(store.deviceStates['1'].device.name).toBe('P1')
    expect(store.deviceStates['1'].online).toBe(true)
  })

  it('updates telemetry', () => {
    const store = useDevicesStore()
    store.setDevices([{ id: '1', name: 'P1', product_type: 'HWE-P1', ip: '192.168.1.1', port: 80 }])
    store.updateTelemetry({
      id: '1',
      data: { active_power_w: 500 },
      sparkline: [400, 450, 500],
      online: true,
    })
    expect(store.deviceStates['1'].data?.active_power_w).toBe(500)
    expect(store.deviceStates['1'].sparkline).toEqual([400, 450, 500])
  })

  it('marks device offline', () => {
    const store = useDevicesStore()
    store.setDevices([{ id: '1', name: 'P1', product_type: 'HWE-P1', ip: '192.168.1.1', port: 80 }])
    store.setOffline('1')
    expect(store.deviceStates['1'].online).toBe(false)
  })

  it('marks device online', () => {
    const store = useDevicesStore()
    store.setDevices([{ id: '1', name: 'P1', product_type: 'HWE-P1', ip: '192.168.1.1', port: 80 }])
    store.setOffline('1')
    store.setOnline('1')
    expect(store.deviceStates['1'].online).toBe(true)
  })
})

describe('useDevicesStore network listeners', () => {
  let addSpy: ReturnType<typeof vi.spyOn>
  let removeSpy: ReturnType<typeof vi.spyOn>

  beforeEach(() => {
    setActivePinia(createPinia())
    addSpy = vi.spyOn(window, 'addEventListener')
    removeSpy = vi.spyOn(window, 'removeEventListener')
  })

  afterEach(() => {
    addSpy.mockRestore()
    removeSpy.mockRestore()
  })

  it('initNetworkListeners returns a cleanup function', () => {
    const store = useDevicesStore()
    const cleanup = store.initNetworkListeners()
    expect(typeof cleanup).toBe('function')
    cleanup()
  })

  it('cleanup removes the exact online/offline handlers that were added', () => {
    const store = useDevicesStore()
    const cleanup = store.initNetworkListeners()

    const addedOnline = addSpy.mock.calls.find((c) => c[0] === 'online')?.[1]
    const addedOffline = addSpy.mock.calls.find((c) => c[0] === 'offline')?.[1]
    expect(addedOnline).toBeDefined()
    expect(addedOffline).toBeDefined()

    cleanup()

    expect(removeSpy).toHaveBeenCalledWith('online', addedOnline)
    expect(removeSpy).toHaveBeenCalledWith('offline', addedOffline)
  })
})
