import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import DeviceListItem from './DeviceListItem.vue'

const baseProps = {
  name: 'Energy Socket',
  productType: 'HWE-SKT',
  online: true,
  metricValue: '342',
  metricUnit: 'W',
}

describe('DeviceListItem', () => {
  it('renders the device name', () => {
    const wrapper = mount(DeviceListItem, { props: baseProps })
    expect(wrapper.text()).toContain('Energy Socket')
  })

  it('renders the metric value', () => {
    const wrapper = mount(DeviceListItem, { props: baseProps })
    expect(wrapper.text()).toContain('342')
  })

  it('renders the metric unit', () => {
    const wrapper = mount(DeviceListItem, { props: baseProps })
    expect(wrapper.text()).toContain('W')
  })

  it('shows em dash when metricValue is null', () => {
    const wrapper = mount(DeviceListItem, {
      props: { ...baseProps, metricValue: null },
    })
    expect(wrapper.text()).toContain('\u2014')
  })

  it('renders secondary text when provided', () => {
    const wrapper = mount(DeviceListItem, {
      props: { ...baseProps, secondary: 'Living room' },
    })
    expect(wrapper.text()).toContain('Living room')
  })

  it('does not render secondary text when not provided', () => {
    const wrapper = mount(DeviceListItem, { props: baseProps })
    const spans = wrapper.findAll('span')
    const secondarySpan = spans.filter((s) => s.classes().includes('text-neutral-600'))
    expect(secondarySpan.length).toBe(0)
  })

  it('shows drag handle when draggable is true', () => {
    const wrapper = mount(DeviceListItem, {
      props: { ...baseProps, draggable: true },
    })
    expect(wrapper.find('.drag-handle').exists()).toBe(true)
  })

  it('hides drag handle when draggable is false', () => {
    const wrapper = mount(DeviceListItem, {
      props: { ...baseProps, draggable: false },
    })
    expect(wrapper.find('.drag-handle').exists()).toBe(false)
  })
})
