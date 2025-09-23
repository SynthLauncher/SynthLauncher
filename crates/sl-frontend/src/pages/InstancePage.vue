<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { getInstance } from '../lib/commands/instances';
import { bytesToImageUrl } from '../lib/utils';
import { InstanceMetadata } from '../types/instances';

const route = useRoute()
const instanceName = route.params.instance_name as string
const instanceMetadata = ref<InstanceMetadata>();

const loadInstances = async () => {
  const result = await getInstance(instanceName);
  instanceMetadata.value = result as InstanceMetadata;
}

onMounted(async () => {
  await loadInstances()
  console.log(instanceMetadata.value)
})
</script>


<template>
  <div class="h-full w-full p-6">
    <div class="bg-[#232529] w-full flex gap-4 p-4">
      <div class="bg-neutral-700/70 p-2 rounded-lg">
        <img :src="bytesToImageUrl(instanceMetadata?.icon as Uint8Array)" alt="Instance Icon" class="size-32">
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
