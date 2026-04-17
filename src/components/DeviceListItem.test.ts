import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import DeviceListItem from './DeviceListItem.vue'

const baseProps = {
  id: 'abc',
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

  it('renders an input when editing is true', async () => {
    const wrapper = mount(DeviceListItem, { props: { ...baseProps, editing: true } })
    expect(wrapper.find('input[type="text"]').exists()).toBe(true)
  })

  it('input is pre-filled with current name', () => {
    const wrapper = mount(DeviceListItem, { props: { ...baseProps, editing: true } })
    const input = wrapper.get('input[type="text"]')
    expect((input.element as HTMLInputElement).value).toBe('Energy Socket')
  })

  it('Enter emits rename with trimmed new value', async () => {
    const wrapper = mount(DeviceListItem, { props: { ...baseProps, editing: true } })
    const input = wrapper.get('input[type="text"]')
    await input.setValue('  Living Room  ')
    await input.trigger('keydown.enter')
    expect(wrapper.emitted('rename')?.[0]).toEqual(['Living Room'])
  })

  it('Enter with empty value emits rename-cancel and not rename', async () => {
    const wrapper = mount(DeviceListItem, { props: { ...baseProps, editing: true } })
    const input = wrapper.get('input[type="text"]')
    await input.setValue('   ')
    await input.trigger('keydown.enter')
    expect(wrapper.emitted('rename')).toBeUndefined()
    expect(wrapper.emitted('rename-cancel')).toBeTruthy()
  })

  it('Escape emits rename-cancel', async () => {
    const wrapper = mount(DeviceListItem, { props: { ...baseProps, editing: true } })
    await wrapper.get('input[type="text"]').trigger('keydown.escape')
    expect(wrapper.emitted('rename-cancel')).toBeTruthy()
  })

  it('blur with valid value emits rename (like Finder)', async () => {
    const wrapper = mount(DeviceListItem, { props: { ...baseProps, editing: true } })
    const input = wrapper.get('input[type="text"]')
    await input.setValue('Novo')
    await input.trigger('blur')
    expect(wrapper.emitted('rename')?.[0]).toEqual(['Novo'])
  })

  it('blur with empty value emits rename-cancel', async () => {
    const wrapper = mount(DeviceListItem, { props: { ...baseProps, editing: true } })
    const input = wrapper.get('input[type="text"]')
    await input.setValue('')
    await input.trigger('blur')
    expect(wrapper.emitted('rename')).toBeUndefined()
    expect(wrapper.emitted('rename-cancel')).toBeTruthy()
  })

  it('renders name div (not input) when editing is false', () => {
    const wrapper = mount(DeviceListItem, { props: baseProps })
    expect(wrapper.find('input[type="text"]').exists()).toBe(false)
    expect(wrapper.text()).toContain('Energy Socket')
  })

  it('passes pulsing prop to DeviceIcon', () => {
    const wrapper = mount(DeviceListItem, { props: { ...baseProps, pulsing: true } })
    const iconEl = wrapper.find('.device-icon')
    expect(iconEl.exists()).toBe(true)
    expect(iconEl.classes()).toContain('pulsing')
  })
})
