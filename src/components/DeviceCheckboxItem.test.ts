import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import DeviceCheckboxItem from './DeviceCheckboxItem.vue'

const device = {
  name: 'P1 Meter',
  ip: '192.168.1.10',
  port: 80,
  product_type: 'HWE-P1',
}

describe('DeviceCheckboxItem', () => {
  it('renders the device name and address', () => {
    const wrapper = mount(DeviceCheckboxItem, {
      props: { device, selected: false },
    })
    expect(wrapper.text()).toContain('P1 Meter')
    expect(wrapper.text()).toContain('192.168.1.10:80')
  })

  it('checkbox reflects selected prop', () => {
    const selected = mount(DeviceCheckboxItem, {
      props: { device, selected: true },
    })
    expect((selected.find('input[type="checkbox"]').element as HTMLInputElement).checked).toBe(true)

    const unselected = mount(DeviceCheckboxItem, {
      props: { device, selected: false },
    })
    expect((unselected.find('input[type="checkbox"]').element as HTMLInputElement).checked).toBe(
      false,
    )
  })

  it('emits toggle when checkbox changes', async () => {
    const wrapper = mount(DeviceCheckboxItem, {
      props: { device, selected: false },
    })
    await wrapper.find('input[type="checkbox"]').trigger('change')
    expect(wrapper.emitted('toggle')).toHaveLength(1)
  })
})
