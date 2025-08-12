<template>
  <button :class="[
    'flex items-center gap-3 px-3 py-3 rounded-full cursor-pointer transition-all active:scale-95 select-none',
    active
      ? 'bg-[#1fc0fe]/20 hover:bg-[#1fc0fe]/30 text-[#59bbfe]'
      : 'bg-transparent text-[#b0b8c7] hover:bg-[#9eb0b8]/20 hover:text-[#c2cbcf]',
  ]" @click="onClick?.()">
    <component :is="icon" :size="24" />
  </button>
</template>

<script setup lang="ts">
defineProps<{
  id: string;
  label: string;
  icon: any;
  active?: boolean;
  onClick?: () => void;
}
>();
</script>