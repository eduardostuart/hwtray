<template>
  <div>
    <h3 class="text-[10px] font-semibold text-neutral-600 uppercase tracking-widest mb-2">Links</h3>
    <a
      v-for="link in links"
      :key="link.url"
      class="block text-[13px] text-hw-green py-1.5 cursor-pointer hover:underline"
      @click="$emit('open-url', link.url)"
      >{{ link.label }}</a
    >
  </div>
</template>

<script setup lang="ts">
defineEmits<{ 'open-url': [url: string] }>()

const links = [
  { label: 'HomeWizard Energy API', url: 'https://api-documentation.homewizard.com' },
  { label: 'HomeWizard Official', url: 'https://www.homewizard.com' },
]
</script>
