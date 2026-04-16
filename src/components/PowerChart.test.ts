import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'

vi.mock('vue-chartjs', () => ({
  Line: {
    name: 'Line',
    props: ['data', 'options'],
    template: '<canvas data-testid="chart-canvas" />',
  },
}))

import PowerChart from './PowerChart.vue'

describe('PowerChart', () => {
  it('renders chart only when data has more than 1 point', () => {
    const enough = mount(PowerChart, { props: { data: [100, 200, 300] } })
    expect(enough.find('[data-testid="chart-canvas"]').exists()).toBe(true)

    const tooFew = mount(PowerChart, { props: { data: [100] } })
    expect(tooFew.find('[data-testid="chart-canvas"]').exists()).toBe(false)

    const empty = mount(PowerChart, { props: { data: [] } })
    expect(empty.find('[data-testid="chart-canvas"]').exists()).toBe(false)
  })
})
