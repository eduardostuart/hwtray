import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import SettingsToggle from './SettingsToggle.vue'

describe('SettingsToggle', () => {
  it('renders label and optional description', () => {
    const withDesc = mount(SettingsToggle, {
      props: { label: 'Auto-start', description: 'Start on boot', modelValue: false },
    })
    expect(withDesc.text()).toContain('Auto-start')
    expect(withDesc.text()).toContain('Start on boot')

    const withoutDesc = mount(SettingsToggle, {
      props: { label: 'Auto-start', modelValue: false },
    })
    expect(withoutDesc.find('p').exists()).toBe(false)
  })

  it('emits update:modelValue with negated value on click', async () => {
    const off = mount(SettingsToggle, {
      props: { label: 'Toggle', modelValue: false },
    })
    await off.find('div').trigger('click')
    expect(off.emitted('update:modelValue')?.[0]).toEqual([true])

    const on = mount(SettingsToggle, {
      props: { label: 'Toggle', modelValue: true },
    })
    await on.find('div').trigger('click')
    expect(on.emitted('update:modelValue')?.[0]).toEqual([false])
  })
})
