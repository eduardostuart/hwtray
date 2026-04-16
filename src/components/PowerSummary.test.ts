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

import PowerSummary from './PowerSummary.vue'

describe('PowerSummary', () => {
  const baseProps = {
    totalPower: 500,
    totalImport: 1234,
    totalExport: 567,
    sparkline: [100, 200, 300],
  }

  it('hides export section when totalExport is 0', () => {
    const wrapper = mount(PowerSummary, {
      props: { ...baseProps, totalExport: 0 },
    })
    expect(wrapper.text()).not.toContain('Export')
  })

  it('shows export section when totalExport > 0', () => {
    const wrapper = mount(PowerSummary, { props: baseProps })
    expect(wrapper.text()).toContain('Export')
    expect(wrapper.text()).toContain('567')
  })

  it('shows "Current usage" for positive power, "Exporting to grid" for negative', () => {
    const positive = mount(PowerSummary, { props: baseProps })
    expect(positive.text()).toContain('Current usage')

    const negative = mount(PowerSummary, {
      props: { ...baseProps, totalPower: -200 },
    })
    expect(negative.text()).toContain('Exporting to grid')
  })

  it('renders active tariff when provided', () => {
    const wrapper = mount(PowerSummary, {
      props: { ...baseProps, activeTariff: 2 },
    })
    expect(wrapper.text()).toContain('T2')
  })
})
