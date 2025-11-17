<script setup lang="ts">
import { useRoute } from 'vue-router'
import { bytesToImageUrl } from '@/lib/utils';
import { instancesManager } from '@/lib/managers/instances';

const route = useRoute()
const instanceName = route.params.instance_name as string
const instanceMetadata = instancesManager.get(instanceName)
</script>


<template>
  <div class="h-full w-full p-6">
    <div class="bg-[#232529] w-full flex gap-4 p-4">
      <div class="bg-neutral-700/70 p-2 rounded-lg">
        <img :src="bytesToImageUrl(instanceMetadata?.icon as Uint8Array)" class="size-32">
      </div>
      
      <div class="flex flex-col gap-2">
        <h1 class="text-white text-4xl font-semibold">
          {{ instanceMetadata?.name }}
        </h1>

        <div>
          <h2 class="text-gray-300 text-xl">
          Version: {{ instanceMetadata?.mc_version }}
          </h2>
          <h2 class="text-gray-300 text-xl">
            Mod Loader: {{ instanceMetadata?.mod_loader }} {{ instanceMetadata?.mod_loader_version }}
          </h2>
      </div>
      </div>
    </div>
  </div>
</template>
