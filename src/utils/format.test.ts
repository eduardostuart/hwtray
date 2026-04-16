import { describe, it, expect } from 'vitest'
import { fmt } from './format'

describe('fmt', () => {
  it('formats number with default 1 decimal', () => {
    expect(fmt(123.456)).toBe('123.5')
  })

  it('formats with suffix', () => {
    expect(fmt(230, 'V', 0)).toBe('230V')
  })

  it('formats with custom decimals', () => {
    expect(fmt(0.98, 'PF', 2)).toBe('0.98PF')
  })

  it('returns null for null input', () => {
    expect(fmt(null)).toBeNull()
  })

  it('returns null for undefined input', () => {
    expect(fmt(undefined)).toBeNull()
  })

  it('formats zero', () => {
    expect(fmt(0)).toBe('0.0')
  })

  it('formats negative numbers', () => {
    expect(fmt(-1200, '', 0)).toBe('-1200')
  })
})
