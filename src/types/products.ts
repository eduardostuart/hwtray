/** HomeWizard product type identifiers as returned by the local API. */
export const ProductType = {
  P1_METER: 'HWE-P1',
  ENERGY_SOCKET: 'HWE-SKT',
  WATERMETER: 'HWE-WTR',
  KWH_METER_1P: 'HWE-KWH1',
  KWH_METER_3P: 'HWE-KWH3',
  SDM230: 'SDM230-wifi',
  SDM630: 'SDM630-wifi',
  GAS: 'gas',
} as const

export type ProductTypeId = (typeof ProductType)[keyof typeof ProductType]

/** Returns true if the product type is a power/energy meter (not water). */
export function isPowerDevice(productType: string): boolean {
  return productType !== ProductType.WATERMETER
}

/** Short label displayed inside the device icon for non-SVG types. */
export function productIconLabel(productType: string): string {
  switch (productType) {
    case ProductType.KWH_METER_1P:
    case ProductType.KWH_METER_3P:
    case ProductType.SDM230:
    case ProductType.SDM630:
      return 'kW'
    default:
      return '?'
  }
}

/** Accent color for each product type. */
export function productColor(productType: string): string {
  switch (productType) {
    case ProductType.WATERMETER:
      return '#3B82F6'
    case ProductType.ENERGY_SOCKET:
      return '#F59E0B'
    case ProductType.GAS:
      return '#EF4444'
    default:
      return '#10B981'
  }
}
