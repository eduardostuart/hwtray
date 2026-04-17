import { describe, it, expect, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import DeviceActionsMenu from './DeviceActionsMenu.vue'

const socketProps = { deviceId: 'abc', productType: 'HWE-SKT', online: true }
const p1Props = { deviceId: 'xyz', productType: 'HWE-P1', online: true }

describe('DeviceActionsMenu', () => {
  beforeEach(() => {
    document.body.innerHTML = ''
  })

  it('renders only the trigger button by default', () => {
    const wrapper = mount(DeviceActionsMenu, { props: socketProps, attachTo: document.body })
    expect(wrapper.find('button').exists()).toBe(true)
    expect(document.body.querySelector('[data-testid="device-actions-menu"]')).toBeNull()
    wrapper.unmount()
  })

  it('opens menu on trigger click (teleported to body)', async () => {
    const wrapper = mount(DeviceActionsMenu, { props: socketProps, attachTo: document.body })
    await wrapper.get('button').trigger('click')
    expect(document.body.querySelector('[data-testid="device-actions-menu"]')).not.toBeNull()
    wrapper.unmount()
  })

  it('shows all three actions for HWE-SKT', async () => {
    const wrapper = mount(DeviceActionsMenu, { props: socketProps, attachTo: document.body })
    await wrapper.get('button').trigger('click')
    const menu = document.body.querySelector('[data-testid="device-actions-menu"]')!
    expect(menu.textContent).toContain('Identify')
    expect(menu.textContent).toContain('Rename')
    expect(menu.textContent).toContain('Hide')
    wrapper.unmount()
  })

  it('hides Identify for non-socket products', async () => {
    const wrapper = mount(DeviceActionsMenu, { props: p1Props, attachTo: document.body })
    await wrapper.get('button').trigger('click')
    const menu = document.body.querySelector('[data-testid="device-actions-menu"]')!
    expect(menu.textContent).not.toContain('Identify')
    expect(menu.textContent).toContain('Rename')
    expect(menu.textContent).toContain('Hide')
    wrapper.unmount()
  })

  it('emits identify and closes on Identify click', async () => {
    const wrapper = mount(DeviceActionsMenu, { props: socketProps, attachTo: document.body })
    await wrapper.get('button').trigger('click')
    const btn = document.body.querySelector('[data-action="identify"]') as HTMLButtonElement
    btn.click()
    await flushPromises()
    expect(wrapper.emitted('identify')).toBeTruthy()
    expect(document.body.querySelector('[data-testid="device-actions-menu"]')).toBeNull()
    wrapper.unmount()
  })

  it('emits rename and closes on Rename click', async () => {
    const wrapper = mount(DeviceActionsMenu, { props: socketProps, attachTo: document.body })
    await wrapper.get('button').trigger('click')
    const btn = document.body.querySelector('[data-action="rename"]') as HTMLButtonElement
    btn.click()
    await flushPromises()
    expect(wrapper.emitted('rename')).toBeTruthy()
    wrapper.unmount()
  })

  it('emits hide and closes on Hide click', async () => {
    const wrapper = mount(DeviceActionsMenu, { props: socketProps, attachTo: document.body })
    await wrapper.get('button').trigger('click')
    const btn = document.body.querySelector('[data-action="hide"]') as HTMLButtonElement
    btn.click()
    await flushPromises()
    expect(wrapper.emitted('hide')).toBeTruthy()
    wrapper.unmount()
  })

  it('closes on Escape', async () => {
    const wrapper = mount(DeviceActionsMenu, { props: socketProps, attachTo: document.body })
    await wrapper.get('button').trigger('click')
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }))
    await flushPromises()
    expect(document.body.querySelector('[data-testid="device-actions-menu"]')).toBeNull()
    wrapper.unmount()
  })

  it('closes on outside click', async () => {
    const wrapper = mount(DeviceActionsMenu, { props: socketProps, attachTo: document.body })
    await wrapper.get('button').trigger('click')
    document.body.dispatchEvent(new MouseEvent('mousedown', { bubbles: true }))
    await flushPromises()
    expect(document.body.querySelector('[data-testid="device-actions-menu"]')).toBeNull()
    wrapper.unmount()
  })
})
