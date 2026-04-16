import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import MetricCard from './MetricCard.vue'

describe('MetricCard', () => {
  it('renders label and value', () => {
    const wrapper = mount(MetricCard, {
      props: { label: 'Active Power', value: '543', unit: 'W' },
    })
    expect(wrapper.text()).toContain('Active Power')
    expect(wrapper.text()).toContain('543')
    expect(wrapper.text()).toContain('W')
  })

  it('shows dash when value is null', () => {
    const wrapper = mount(MetricCard, {
      props: { label: 'Gas', value: null, unit: 'm³' },
    })
    expect(wrapper.text()).toContain('—')
  })
})
