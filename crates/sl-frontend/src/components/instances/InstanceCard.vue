<script lang="ts" setup>
import { PlayIcon } from 'lucide-vue-next';
import { instancesManager } from '@/lib/managers/instances';
import { ModLoader } from '@/types/instances';

defineProps<{
  icon?: string,
  name: string,
  mc_version: string,
  mod_loader: ModLoader,
  mod_loader_version: string
}>();
</script>

<template>
  <div class="flex items-center justify-between hover:bg-[#26546c]/50 hover:border-sky-500 bg-[#1b1d21] border-2 border-[#2c2f34] py-3 px-4 rounded-lg transition-colors duration-300 hover:cursor-pointer">

    <div class="flex items-center gap-4">
      <img :src="icon" class="object-cover size-16 rounded-md" alt="Instance Icon">

      <div>
        <h1 class="text-white text-lg">{{ name }}</h1>
        <p class="text-gray-400 text-sm">Version: {{ mc_version }}</p>
        <p class="text-gray-400 text-sm">Mod Loader: {{ mod_loader }} {{ mod_loader_version }}</p>
      </div>
    </div>

    <button
    @click="async () => await instancesManager.launch_instance(name)" 
    class="cursor-pointer flex items-center
 justify-center size-10 rounded-full bg-sky-500 hover:bg-sky-600 transition-colors">
      <PlayIcon class="size-5 text-white" />
    </button>
  </div>
</template>
