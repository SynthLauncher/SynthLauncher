<script setup lang="ts">
import { ref } from 'vue'
import { storeManager } from '@/lib/managers/store'

const open = ref(false)

function selectInstance(version: string) {
  open.value = false
}
</script>

<template>
  <div class="relative w-64">
    <button
      class="w-full flex justify-between items-center rounded-xl border border-gray-300 bg-white px-4 py-2
             shadow-sm text-sm text-gray-800
             focus:ring-2 focus:ring-indigo-400 focus:outline-none"
      @click="open = !open"
    >
      {{ "Select a version" }}
      <svg class="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>

    <ul
      v-if="open"
      class="absolute z-10 mt-1 w-full rounded-xl bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600
             shadow-lg max-h-60 overflow-auto text-sm"
    >
      <li
        v-for="version in storeManager.selectedContent.versions"
        :key="version.name"
        class="px-4 py-2 cursor-pointer hover:bg-indigo-100 dark:hover:bg-indigo-700"
      >
        {{ version.name }}
      </li>
    </ul>
  </div>
</template>
