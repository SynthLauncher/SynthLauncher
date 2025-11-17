<script setup lang="ts">
import { computed, ref } from 'vue'
import { storeManager } from '@/lib/managers/store'
import { StoreContentVersion } from '@/types/store'

const open = ref(false)
const hasVersions = computed(() => {
  return storeManager.selectedContent?.versions?.length > 0
})

function selectVersion(version: StoreContentVersion) {
  open.value = false
  storeManager.selectedContents.set(storeManager.selectedContent.slug, version)
}
</script>

<template>
  <div class="relative p-1">
    <button
      class="w-full flex justify-between items-center rounded-full bg-[#262729] px-4 py-3
             shadow-sm text-sm text-white"
      :disabled="!hasVersions"
      @click="open = !open"
    >
      {{ storeManager.selectedContent.versions.length != 0 ? storeManager.selectedContents.get(storeManager.selectedContent.slug)?.name : "No versions" }}
      <svg class="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>

    <ul
      v-if="open"
      class="translate-y-1 absolute z-10 mt-1 w-full rounded-xl bg-[#262729] border-2 border-[#333334] text-white max-h-60 overflow-auto text-sm"
    >
      <li
        v-for="version in storeManager.selectedContent.versions"
        :key="version.name"
        @click="selectVersion(version)"
        class="px-4 py-2 cursor-pointer hover:bg-[#6a88c4]/20"
      >
        {{ version.name }}
      </li>
    </ul>
  </div>
</template>
