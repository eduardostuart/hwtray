import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import DevicePageLayout from './DevicePageLayout.vue'

describe('DevicePageLayout', () => {
  it('emits back event when back button clicked', async () => {
    const wrapper = mount(DevicePageLayout, {
      props: { title: 'P1 Meter', online: true },
    })
    await wrapper.find('button').trigger('click')
    expect(wrapper.emitted('back')).toHaveLength(1)
  })

  it('renders network section only when ssid is provided', () => {
    const withSsid = mount(DevicePageLayout, {
      props: { title: 'P1 Meter', online: true, ssid: 'MyWifi' },
    })
    expect(withSsid.text()).toContain('Network')
    expect(withSsid.text()).toContain('MyWifi')

    const withoutSsid = mount(DevicePageLayout, {
      props: { title: 'P1 Meter', online: true },
    })
    expect(withoutSsid.text()).not.toContain('Network')
  })
})
