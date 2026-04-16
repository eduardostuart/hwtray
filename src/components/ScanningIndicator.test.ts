import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import ScanningIndicator from './ScanningIndicator.vue'

describe('ScanningIndicator', () => {
  it('renders countdown value in seconds', () => {
    const wrapper = mount(ScanningIndicator, {
      props: { countdown: 8, found: 0 },
    })
    expect(wrapper.text()).toContain('8s')
  })

  it('uses singular "device" when found is 1, plural otherwise', () => {
    const plural = mount(ScanningIndicator, {
      props: { countdown: 5, found: 3 },
    })
    expect(plural.text()).toContain('3 devices found')

    const singular = mount(ScanningIndicator, {
      props: { countdown: 5, found: 1 },
    })
    expect(singular.text()).toContain('1 device found')
    expect(singular.text()).not.toContain('devices')
  })
})
