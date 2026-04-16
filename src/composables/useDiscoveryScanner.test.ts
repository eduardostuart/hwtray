import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { defineComponent, nextTick } from 'vue'
import { mount } from '@vue/test-utils'

const listenCalls: Array<{ event: string; unlisten: ReturnType<typeof vi.fn> }> = []

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockImplementation(async (event: string) => {
    const unlisten = vi.fn()
    listenCalls.push({ event, unlisten })
    return unlisten
  }),
}))

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue(undefined),
}))

import { useDiscoveryScanner } from './useDiscoveryScanner'

const Harness = defineComponent({
  setup() {
    return useDiscoveryScanner()
  },
  render: () => null,
})

describe('useDiscoveryScanner', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    listenCalls.length = 0
    vi.clearAllMocks()
  })

  it('calling startSearch twice cleans up the previous listener', async () => {
    const wrapper = mount(Harness)
    await nextTick()
    await nextTick()

    // onMounted triggers the first startSearch
    expect(listenCalls).toHaveLength(1)
    const firstUnlisten = listenCalls[0].unlisten

    // A second manual call should dispose the first listener
    await wrapper.vm.startSearch()

    expect(listenCalls.length).toBeGreaterThanOrEqual(2)
    expect(firstUnlisten).toHaveBeenCalledTimes(1)
  })
})
