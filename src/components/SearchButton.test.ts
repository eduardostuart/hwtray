import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import SearchButton from './SearchButton.vue'

describe('SearchButton', () => {
  it('is not disabled by default', () => {
    const wrapper = mount(SearchButton, { slots: { default: 'Go' } })
    expect(wrapper.find('button').attributes('disabled')).toBeUndefined()
  })

  it('is disabled when disabled prop is true', () => {
    const wrapper = mount(SearchButton, {
      props: { disabled: true },
      slots: { default: 'Go' },
    })
    expect(wrapper.find('button').attributes('disabled')).toBeDefined()
  })

  it('applies disabled variant class instead of primary when disabled', () => {
    const wrapper = mount(SearchButton, {
      props: { disabled: true },
      slots: { default: 'Go' },
    })
    expect(wrapper.find('button').classes()).toContain('hw-btn-disabled')
    expect(wrapper.find('button').classes()).not.toContain('hw-btn-primary')
  })

  it('applies the correct variant class based on prop', () => {
    const danger = mount(SearchButton, {
      props: { variant: 'danger' },
      slots: { default: 'Delete' },
    })
    expect(danger.find('button').classes()).toContain('hw-btn-danger')

    const ghost = mount(SearchButton, {
      props: { variant: 'ghost' },
      slots: { default: 'Cancel' },
    })
    expect(ghost.find('button').classes()).toContain('hw-btn-ghost')
  })

  it('emits click event when clicked', async () => {
    const wrapper = mount(SearchButton, { slots: { default: 'Go' } })
    await wrapper.find('button').trigger('click')
    expect(wrapper.emitted('click')).toHaveLength(1)
  })
})
