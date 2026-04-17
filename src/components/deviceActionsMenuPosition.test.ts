import { describe, it, expect } from 'vitest'
import { computeMenuPosition, type Viewport } from './deviceActionsMenuPosition'

const viewport: Viewport = { top: 0, left: 0, right: 400, bottom: 600 }

function rect(top: number, left: number, w = 16, h = 16) {
  return { top, left, right: left + w, bottom: top + h }
}

describe('computeMenuPosition', () => {
  it('opens below the trigger when there is space', () => {
    const pos = computeMenuPosition(rect(100, 300), 160, 100, viewport)
    expect(pos.top).toBe(120) // 100 + 16 + 4
    expect(pos.left).toBe(156) // 316 - 160
  })

  it('flips up when not enough space below', () => {
    const pos = computeMenuPosition(rect(550, 300), 160, 100, viewport)
    expect(pos.top).toBe(446) // 550 - 4 - 100
  })

  it('pins to bottom margin when menu taller than viewport and more space below', () => {
    const pos = computeMenuPosition(rect(10, 300), 160, 700, viewport)
    expect(pos.top).toBe(8) // max(top+MARGIN, bottom-menu-MARGIN)
  })

  it('pins to top margin when menu taller than viewport and more space above', () => {
    const pos = computeMenuPosition(rect(500, 300), 160, 700, viewport)
    expect(pos.top).toBe(8) // viewport.top + MARGIN
  })

  it('flips up when below does not fit but above does', () => {
    const short: Viewport = { top: 0, left: 0, right: 400, bottom: 330 }
    const pos = computeMenuPosition(rect(300, 300), 160, 100, short)
    expect(pos.top).toBe(196) // 300 - 4 - 100
  })

  it('aligns to trigger.left when right-alignment would cut off on the left', () => {
    const pos = computeMenuPosition(rect(100, 20), 160, 100, viewport)
    expect(pos.left).toBe(20)
  })

  it('clamps to the left margin when trigger is too close to the edge', () => {
    const pos = computeMenuPosition(rect(100, 4), 160, 100, viewport)
    expect(pos.left).toBe(8)
  })

  it('clamps to right margin when right-alignment overflows on the right', () => {
    const pos = computeMenuPosition(rect(100, 10), 500, 100, viewport)
    expect(pos.left).toBe(8) // clamp to MARGIN since maxLeft is negative
  })

  it('respects the right viewport margin', () => {
    const pos = computeMenuPosition(rect(100, 390, 10), 160, 100, viewport)
    expect(pos.left).toBe(232) // 400 - 160 - 8
  })

  it('treats non-zero viewport origin correctly (content area below arrow)', () => {
    // Viewport starts at y=10 (below arrow bar), bottom at 460 (content area only).
    const content: Viewport = { top: 10, left: 0, right: 400, bottom: 460 }
    // Trigger at y=430 — not enough below, flip up
    const pos = computeMenuPosition(rect(430, 300), 160, 100, content)
    // spaceBelow = 460-446-4-8 = 2; spaceAbove = 430-10-4-8 = 408 → fits above
    expect(pos.top).toBe(326) // 430 - 4 - 100
  })
})
