<template>
  <div class="flex flex-col items-center justify-center py-14">
    <div class="mb-6 relative">
      <svg width="56" height="56" viewBox="0 0 56 56" fill="none">
        <circle
          cx="28"
          cy="28"
          r="24"
          stroke="#10B981"
          stroke-width="0.75"
          stroke-dasharray="3 3"
          opacity="0.25"
        />
        <!-- Lightning bolt -->
        <path d="M31 14L21 30h8l-3 12 13-18h-9l3-10z" fill="#10B981" opacity="0.7" />
      </svg>
      <div class="absolute inset-0 rounded-full icon-glow" />
    </div>
    <p class="text-[11px] text-neutral-500 font-medium tracking-wide">HWTray</p>
    <p class="text-[10px] text-neutral-600 mt-0.5">for HomeWizard devices</p>
  </div>
</template>

<style scoped>
.icon-glow {
  background: radial-gradient(circle, rgba(16, 185, 129, 0.1) 0%, transparent 70%);
}
</style>
