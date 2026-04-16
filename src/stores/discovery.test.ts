import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useDiscoveryStore } from './discovery'

describe('useDiscoveryStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  const device = {
    id: 'aabb',
    name: 'P1 Meter',
    ip: '192.168.1.50',
    port: 80,
    serial: 'aabb',
    product_type: 'HWE-P1',
    api_enabled: true,
  }

  it('starts empty', () => {
    const store = useDiscoveryStore()
    expect(store.foundDevices).toEqual([])
    expect(store.isSearching).toBe(false)
    expect(store.selectedIds.size).toBe(0)
  })

  it('adds a discovered device', () => {
    const store = useDiscoveryStore()
    store.addDevice(device)
    expect(store.foundDevices).toHaveLength(1)
    expect(store.foundDevices[0].id).toBe('aabb')
  })

  it('does not add duplicate devices', () => {
    const store = useDiscoveryStore()
    store.addDevice(device)
    store.addDevice(device)
    expect(store.foundDevices).toHaveLength(1)
  })

  it('toggles selection on and off', () => {
    const store = useDiscoveryStore()
    store.addDevice(device)
    store.toggleSelection('aabb')
    expect(store.selectedIds.has('aabb')).toBe(true)
    store.toggleSelection('aabb')
    expect(store.selectedIds.has('aabb')).toBe(false)
  })

  it('resets state', () => {
    const store = useDiscoveryStore()
    store.addDevice(device)
    store.toggleSelection('aabb')
    store.isSearching = true
    store.reset()
    expect(store.foundDevices).toEqual([])
    expect(store.selectedIds.size).toBe(0)
    expect(store.isSearching).toBe(false)
  })

  it('converts selected devices to SavedDevice format', () => {
    const store = useDiscoveryStore()
    store.addDevice(device)
    store.toggleSelection('aabb')
    const saved = store.selectedAsDevices()
    expect(saved).toHaveLength(1)
    expect(saved[0]).toEqual({
      id: 'aabb',
      name: 'P1 Meter',
      product_type: 'HWE-P1',
      ip: '192.168.1.50',
      port: 80,
    })
  })

  it('returns empty for unselected devices', () => {
    const store = useDiscoveryStore()
    store.addDevice(device)
    expect(store.selectedAsDevices()).toEqual([])
  })
})
