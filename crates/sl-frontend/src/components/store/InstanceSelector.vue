<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { InstanceMetadata } from '@/types/instances'
import { instancesManager } from '@/lib/managers/instances'
import { storeManager } from '@/lib/managers/store'

const open = ref(false)

function selectInstance(instance: InstanceMetadata) {
  storeManager.selectedInstance = instance
  open.value = false
}

onMounted(() => storeManager.selectedInstance = instancesManager.instances[0])
</script>

<template>
  <div class="relative">
    <button
      class="flex items-center gap-2 rounded-full bg-background px-4 py-2 text-md font-medium text-white hover:bg-opacity-80 transition-all"
      @click="open = !open"
    >
      <span class="text-gray-400">Install to:</span>
      <span>{{ storeManager.selectedInstance?.name || 'Select Instance' }}</span>
      <svg 
        class="w-4 h-4 transition-transform" 
        :class="{ 'rotate-180': open }"
        fill="none" 
        stroke="currentColor" 
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>

    <ul
      v-if="open"
      class="absolute z-10 mt-2 w-64 rounded-xl bg-background border border-border shadow-lg max-h-60 overflow-auto"
    >
      <li
        v-for="instance in instancesManager.instances"
        :key="instance.name"
        @click="selectInstance(instance)"
        class="px-4 py-2.5 cursor-pointer hover:bg-accent/30 text-white text-sm font-medium transition-colors first:rounded-t-xl last:rounded-b-xl"
        :class="{ 'bg-accent/20': storeManager.selectedInstance?.name === instance.name }"
      >
        {{ instance.name }}
      </li>
    </ul>
  </div>
</template>