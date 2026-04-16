import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import BaseSelect from './BaseSelect.vue'

describe('BaseSelect', () => {
  it('renders with options and selects the correct value', () => {
    const wrapper = mount(BaseSelect, {
      props: { modelValue: 'b' },
      slots: {
        default: '<option value="a">Alpha</option><option value="b">Beta</option>',
      },
    })
    const select = wrapper.find('select')
    expect(select.exists()).toBe(true)
    expect(select.findAll('option')).toHaveLength(2)
    expect((select.element as HTMLSelectElement).value).toBe('b')
  })

  it('emits update:modelValue on change', async () => {
    const wrapper = mount(BaseSelect, {
      props: { modelValue: 'a' },
      slots: {
        default: '<option value="a">Alpha</option><option value="b">Beta</option>',
      },
    })
    await wrapper.find('select').setValue('b')
    expect(wrapper.emitted('update:modelValue')![0]).toEqual(['b'])
  })

  it('uses appearance-none for custom styling', () => {
    const wrapper = mount(BaseSelect, {
      props: { modelValue: '' },
    })
    expect(wrapper.find('select').classes()).toContain('appearance-none')
  })
})
