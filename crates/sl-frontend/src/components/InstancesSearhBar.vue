<template>
  <div class="relative w-full">
    <span class="absolute left-3 top-1/2 -translate-y-1/2 text-neutral-400 pointer-events-none">
      <Search class="w-4 h-4" />
    </span>

    <input type="text" :value="searchQuery" :placeholder="`Search instances...`" maxlength="50"
      @input="emit('update:searchQuery', ($event.target as HTMLInputElement).value)" class="text-sm w-full bg-neutral-700/50 
           text-white border-2 border-neutral-700
           placeholder-neutral-300 focus:border-sky-500 
           focus:bg-neutral-800/70 focus:outline-none 
             transition-all duration-300 h-10 rounded-full 
             pl-10 pr-12" />

    <button v-if="searchQuery" @click="emit('update:searchQuery', '')" class="absolute right-3 top-1/2 -translate-y-1/2 w-6 h-6 rounded-full 
             bg-neutral-600 hover:bg-neutral-500 flex items-center 
             justify-center text-neutral-300 hover:text-white transition-colors">
      <X class="w-3.5 h-3.5" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { Search, X } from "lucide-vue-next";

defineProps<{
  searchQuery: string;
}>();

const emit = defineEmits<{
  (e: "update:searchQuery", value: string): void;
}>();
</script>
