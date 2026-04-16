import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import DeviceIcon from './DeviceIcon.vue'

describe('DeviceIcon', () => {
  it('renders Droplets icon for water meter', () => {
    const wrapper = mount(DeviceIcon, { props: { productType: 'HWE-WTR' } })
    expect(wrapper.find('.device-icon').exists()).toBe(true)
    expect(wrapper.find('.device-icon').attributes('style')).toContain('#3B82F6')
  })

  it('renders Zap icon for energy socket', () => {
    const wrapper = mount(DeviceIcon, { props: { productType: 'HWE-SKT' } })
    expect(wrapper.find('.device-icon').attributes('style')).toContain('#F59E0B')
  })

  it('renders Flame icon for gas', () => {
    const wrapper = mount(DeviceIcon, { props: { productType: 'gas' } })
    expect(wrapper.find('.device-icon').attributes('style')).toContain('#EF4444')
  })

  it('renders Zap icon for P1 meter', () => {
    const wrapper = mount(DeviceIcon, { props: { productType: 'HWE-P1' } })
    expect(wrapper.find('.device-icon').attributes('style')).toContain('#10B981')
  })

  it('renders kW text for kWh meters', () => {
    const wrapper = mount(DeviceIcon, { props: { productType: 'HWE-KWH1' } })
    expect(wrapper.text()).toBe('kW')
    expect(wrapper.find('.device-icon').attributes('style')).toContain('#10B981')
  })

  it('renders kW text for SDM230-wifi', () => {
    const wrapper = mount(DeviceIcon, { props: { productType: 'SDM230-wifi' } })
    expect(wrapper.text()).toBe('kW')
  })

  it('renders ? for unknown product types', () => {
    const wrapper = mount(DeviceIcon, { props: { productType: 'UNKNOWN' } })
    expect(wrapper.text()).toBe('?')
    expect(wrapper.find('.device-icon').attributes('style')).toContain('#10B981')
  })
})
