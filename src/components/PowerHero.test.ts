import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'

vi.mock('vue-chartjs', () => ({
  Line: {
    name: 'Line',
    props: ['data', 'options'],
    template: '<canvas />',
  },
}))

vi.mock('@/composables/useAnimatedNumber', async () => {
  const { ref } = await vi.importActual<typeof import('vue')>('vue')
  return {
    useAnimatedNumber: (fn: () => number) => ref(fn()),
  }
})

import PowerHero from './PowerHero.vue'

describe('PowerHero', () => {
  const baseProps = {
    value: '450',
    unit: 'W',
    sparkline: [100, 200, 300],
  }

  it('shows "Current usage" for positive value, "Exporting to grid" for negative', () => {
    const positive = mount(PowerHero, { props: baseProps })
    expect(positive.text()).toContain('Current usage')

    const negative = mount(PowerHero, { props: { ...baseProps, value: '-200' } })
    expect(negative.text()).toContain('Exporting to grid')
  })

  it('renders subtitle only when provided', () => {
    const withSub = mount(PowerHero, {
      props: { ...baseProps, subtitle: 'Solar panel' },
    })
    expect(withSub.text()).toContain('Solar panel')

    const withoutSub = mount(PowerHero, { props: baseProps })
    expect(withoutSub.find('.text-neutral-600').exists()).toBe(false)
  })

  it('renders import and export kWh when provided', () => {
    const wrapper = mount(PowerHero, {
      props: { ...baseProps, importKwh: '123.4', exportKwh: '56.7' },
    })
    expect(wrapper.text()).toContain('Import')
    expect(wrapper.text()).toContain('123.4')
    expect(wrapper.text()).toContain('Export')
    expect(wrapper.text()).toContain('56.7')
  })
})
