<template>
  <div
    class="device-icon flex h-10 w-10 items-center justify-center rounded-xl"
    :style="{ '--icon-color': productColor(productType) }"
  >
    <Droplets v-if="productType === P.WATERMETER" :size="20" class="icon-stroke" />
    <Zap
      v-else-if="productType === P.ENERGY_SOCKET || productType === P.P1_METER"
      :size="20"
      class="icon-stroke"
    />
    <Flame v-else-if="productType === P.GAS" :size="20" class="icon-stroke" />
    <span v-else class="icon-text text-[11px] font-bold">{{ productIconLabel(productType) }}</span>
  </div>
</template>

<script setup lang="ts">
import { Droplets, Zap, Flame } from 'lucide-vue-next'
import { ProductType as P, productIconLabel, productColor } from '@/types/products'

defineProps<{ productType: string }>()
</script>

<style scoped>
.device-icon {
  background: linear-gradient(
    135deg,
    color-mix(in srgb, var(--icon-color) 8%, transparent),
    color-mix(in srgb, var(--icon-color) 3%, transparent)
  );
  border: 1px solid color-mix(in srgb, var(--icon-color) 12%, transparent);
  box-shadow: 0 0 12px color-mix(in srgb, var(--icon-color) 6%, transparent);
}
.icon-stroke {
  stroke: var(--icon-color);
  fill: none;
}
.icon-text {
  color: var(--icon-color);
}
</style>
