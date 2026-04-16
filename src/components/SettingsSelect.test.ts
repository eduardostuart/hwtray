import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import SettingsSelect from './SettingsSelect.vue'

const options = [
  { value: '5', label: '5 seconds' },
  { value: '10', label: '10 seconds' },
  { value: '30', label: '30 seconds' },
]

describe('SettingsSelect', () => {
  it('renders the label', () => {
    const wrapper = mount(SettingsSelect, {
      props: { label: 'Poll interval', modelValue: '5', options },
    })
    expect(wrapper.text()).toContain('Poll interval')
  })

  it('renders the description when provided', () => {
    const wrapper = mount(SettingsSelect, {
      props: { label: 'Interval', description: 'How often to poll', modelValue: '5', options },
    })
    expect(wrapper.text()).toContain('How often to poll')
  })

  it('does not render description when not provided', () => {
    const wrapper = mount(SettingsSelect, {
      props: { label: 'Interval', modelValue: '5', options },
    })
    expect(wrapper.find('p').exists()).toBe(false)
  })

  it('renders all options', () => {
    const wrapper = mount(SettingsSelect, {
      props: { label: 'Interval', modelValue: '5', options },
    })
    const optionEls = wrapper.findAll('option')
    expect(optionEls.length).toBe(3)
    expect(optionEls[0].text()).toBe('5 seconds')
    expect(optionEls[1].text()).toBe('10 seconds')
    expect(optionEls[2].text()).toBe('30 seconds')
  })

  it('emits update:modelValue on change', async () => {
    const wrapper = mount(SettingsSelect, {
      props: { label: 'Interval', modelValue: '5', options },
    })
    await wrapper.find('select').setValue('10')
    expect(wrapper.emitted('update:modelValue')).toBeTruthy()
  })

  it('selects the current modelValue', () => {
    const wrapper = mount(SettingsSelect, {
      props: { label: 'Interval', modelValue: '10', options },
    })
    expect((wrapper.find('select').element as HTMLSelectElement).value).toBe('10')
  })
})
