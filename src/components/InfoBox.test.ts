import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import InfoBox from './InfoBox.vue'

describe('InfoBox', () => {
  it('renders slot content', () => {
    const wrapper = mount(InfoBox, {
      slots: { default: 'Hello world' },
    })
    expect(wrapper.text()).toContain('Hello world')
  })

  it.each([['info' as const], ['warning' as const], ['error' as const]])(
    'renders %s variant with distinct styling',
    (variant) => {
      const wrapper = mount(InfoBox, {
        props: { variant },
        slots: { default: 'Test' },
      })
      expect(wrapper.text()).toContain('Test')
      // Each variant should render a unique container class
      const html = wrapper.html()
      expect(html).toContain(`info-box--${variant}`)
    },
  )

  it('defaults to info variant', () => {
    const wrapper = mount(InfoBox, {
      slots: { default: 'Test' },
    })
    expect(wrapper.html()).toContain('info-box--info')
  })
})
