<template>
  <button
    variant="ghost"
    :aria-label="label"
    @click="onClick"
    :class="'group rounded-full p-0 flex items-center justify-center w-11 h-11 transition-all duration-300'"
  >
    <span
      class="text-white group-hover:text-black transition-all duration-300 transform group-hover:scale-105"
    >
      <component :is="icon" />
    </span>
  </button>
</template>

<script setup lang="ts">
    defineProps<{
        onClick: () => void
        icon: any
        label: string
        className?: string
    }>()
</script>
