export interface Rect {
  top: number
  bottom: number
  left: number
  right: number
}

/** Absolute-coords rect of the area the menu must stay inside. */
export type Viewport = Rect

export interface Position {
  top: number
  left: number
}

const MARGIN = 8
const GAP = 4

export function computeMenuPosition(
  trigger: Rect,
  menuWidth: number,
  menuHeight: number,
  viewport: Viewport,
): Position {
  const spaceBelow = viewport.bottom - trigger.bottom - GAP - MARGIN
  const spaceAbove = trigger.top - viewport.top - GAP - MARGIN

  let top: number
  if (menuHeight <= spaceBelow) {
    top = trigger.bottom + GAP
  } else if (menuHeight <= spaceAbove) {
    top = trigger.top - GAP - menuHeight
  } else if (spaceBelow >= spaceAbove) {
    top = Math.max(viewport.top + MARGIN, viewport.bottom - menuHeight - MARGIN)
  } else {
    top = viewport.top + MARGIN
  }

  let left = trigger.right - menuWidth
  if (left < viewport.left + MARGIN) {
    left = trigger.left
  }
  const maxLeft = viewport.right - menuWidth - MARGIN
  if (left > maxLeft) {
    left = maxLeft
  }
  if (left < viewport.left + MARGIN) {
    left = viewport.left + MARGIN
  }

  return { top, left }
}
