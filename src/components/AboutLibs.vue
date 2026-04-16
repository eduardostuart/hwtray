<template>
  <div>
    <h3 class="text-[10px] font-semibold text-neutral-600 uppercase tracking-widest mb-3">
      Built with
    </h3>
    <div class="flex flex-wrap justify-center gap-1.5">
      <span
        v-for="lib in libs"
        :key="lib"
        class="text-[11px] text-neutral-500 bg-white/[0.03] border border-white/[0.06] rounded-md px-2.5 py-1"
        >{{ lib }}</span
      >
    </div>
  </div>
</template>

<script setup lang="ts">
const libs = [
  'Tauri',
  'Vue',
  'Pinia',
  'Chart.js',
  'Lucide',
  'Tailwind CSS',
  'SortableJS',
  'Vite',
  'Rust',
  'Tokio',
  'Reqwest',
  'mdns-sd',
  'Serde',
  'Tracing',
  'JetBrains Mono',
]
</script>
