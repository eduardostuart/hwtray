import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import Sparkline from './Sparkline.vue'

describe('Sparkline', () => {
  it('renders polyline and area path with sufficient data', () => {
    const wrapper = mount(Sparkline, { props: { data: [10, 20, 30, 40] } })
    expect(wrapper.find('polyline').exists()).toBe(true)
    expect(wrapper.find('path').exists()).toBe(true)
  })

  it('renders nothing with insufficient data (< 2 points)', () => {
    const one = mount(Sparkline, { props: { data: [10] } })
    expect(one.find('polyline').exists()).toBe(false)
    expect(one.find('path').exists()).toBe(false)

    const empty = mount(Sparkline, { props: { data: [] } })
    expect(empty.find('polyline').exists()).toBe(false)
  })

  it('generates correct coordinate points from data', () => {
    const wrapper = mount(Sparkline, {
      props: { data: [0, 50, 100], width: 120, height: 32 },
    })
    const polyline = wrapper.find('polyline')
    const points = polyline.attributes('points')!
    // 3 data points should produce 3 coordinate pairs
    const coords = points.split(' ')
    expect(coords).toHaveLength(3)

    // First point should start at x=0, last at x=120
    expect(coords[0]).toMatch(/^0,/)
    expect(coords[2]).toMatch(/^120,/)
  })

  it('uses absolute values and adjusts color for negative data', () => {
    // When data contains negatives and last value is negative, line should be green (export)
    const wrapper = mount(Sparkline, {
      props: { data: [100, -50, -200] },
    })
    const polyline = wrapper.find('polyline')
    expect(polyline.attributes('stroke')).toBe('#10B981')
  })

  it('uses purple color when last value is positive with mixed data', () => {
    const wrapper = mount(Sparkline, {
      props: { data: [-100, 50, 200] },
    })
    const polyline = wrapper.find('polyline')
    expect(polyline.attributes('stroke')).toBe('#A78BFA')
  })
})
