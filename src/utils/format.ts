/** Format a number with optional suffix and decimal places. Returns null if value is nullish. */
export function fmt(v: number | undefined | null, suffix = '', decimals = 1): string | null {
  if (v == null) {
    return null
  }
  const num = decimals === 0 ? Math.round(v).toString() : v.toFixed(decimals)
  return suffix ? `${num}${suffix}` : num
}
