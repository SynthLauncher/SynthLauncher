<script setup lang="ts">
import { computed, ref } from 'vue'
import { storeManager } from '@/lib/managers/store'
import { StoreContentVersion } from '@/types/store'

const open = ref(false)

const hasVersions = computed(() => {
  return storeManager.selectedContent?.versions?.length > 0
})

const selectedVersion = computed(() => {
  return storeManager.selectedContents.get(storeManager.selectedContent?.slug)
})

function selectVersion(version: StoreContentVersion) {
  open.value = false
  storeManager.selectedContents.set(storeManager.selectedContent.slug, version)
}
</script>

<template>
  <div class="relative">
    <button
      class="flex items-center gap-2 rounded-full bg-background px-4 py-2 text-sm font-medium text-white hover:bg-opacity-80 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
      @click="open = !open"
    >
      <span class="text-gray-400">Version:</span>
      <span class="max-w-[200px] truncate">
        {{ selectedVersion?.name || 'Select version' }}
      </span>
      <svg 
        class="w-4 h-4 transition-transform flex-shrink-0" 
        :class="{ 'rotate-180': open }"
        fill="none" 
        stroke="currentColor" 
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>

    <transition
      enter-active-class="transition ease-out duration-100"
      enter-from-class="opacity-0 scale-95"
      enter-to-class="opacity-100 scale-100"
      leave-active-class="transition ease-in duration-75"
      leave-from-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-95"
    >
      <ul
        v-if="open"
        class="absolute z-10 mt-2 w-full min-w-[250px] rounded-xl bg-background border border-gray-700 shadow-lg max-h-60 overflow-auto"
        @click.stop
      >
        <li
          v-for="version in storeManager.selectedContent.versions"
          :key="version.name"
          @click="selectVersion(version)"
          class="px-4 py-2.5 cursor-pointer hover:bg-gray-700 text-white text-sm font-medium transition-colors first:rounded-t-xl last:rounded-b-xl"
          :class="{ 'bg-gray-700': selectedVersion?.name === version.name }"
        >
          <div class="flex items-center justify-between gap-2">
            <span class="truncate">{{ version.name }}</span>
            <svg 
              v-if="selectedVersion?.name === version.name"
              class="w-4 h-4 text-green-400 flex-shrink-0" 
              fill="currentColor" 
              viewBox="0 0 20 20"
            >
              <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
            </svg>
          </div>
        </li>
      </ul>
    </transition>
  </div>
</template>