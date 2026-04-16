<template>
  <div class="flex flex-col items-center justify-center flex-1 min-h-[300px]">
    <div class="mb-5 relative">
      <svg width="48" height="48" viewBox="0 0 48 48" fill="none">
        <circle
          cx="24"
          cy="24"
          r="20"
          stroke="#EF4444"
          stroke-width="0.75"
          stroke-dasharray="3 3"
          opacity="0.3"
        />
        <path
          d="M16 16l16 16M32 16L16 32"
          stroke="#EF4444"
          stroke-width="1.5"
          stroke-linecap="round"
          opacity="0.5"
        />
        <circle cx="24" cy="24" r="6" stroke="#EF4444" stroke-width="1" opacity="0.4" />
      </svg>
    </div>
    <p class="text-[12px] font-medium text-neutral-400">All devices offline</p>
    <p class="text-[10px] text-neutral-600 mt-1">
      {{ message }}
    </p>
  </div>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    message?: string
  }>(),
  {
    message: 'Check your network connection and device status.',
  },
)
</script>
