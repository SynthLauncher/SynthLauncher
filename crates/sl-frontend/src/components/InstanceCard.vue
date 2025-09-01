<!--  
  FIXME: Make this responsive
-->
<template>
  <div class="flex items-center justify-between bg-[#1b1d21] border-2 border-[#2c2f34] py-3 px-4 rounded-lg">

    <div class="flex items-center gap-4">
      <img :src="icon" class="object-cover size-16 rounded-md" alt="Instance Icon">

      <div>
        <h1 class="text-white text-lg">{{ name }}</h1>
        <p class="text-gray-400 text-sm">Version: {{ mc_version }}</p>
        <p class="text-gray-400 text-sm">Mod Loader: {{ mod_loader }} {{ mod_loader_version }}</p>
      </div>
    </div>

    <button class="cursor-pointer flex items-center
 justify-center size-10 rounded-full bg-sky-500 hover:bg-sky-600 transition-colors" @click="">
      <PlayIcon class="size-5 text-white" />
    </button>
  </div>
</template>

<script lang="ts" setup>
import { PlayIcon } from 'lucide-vue-next';

defineProps<{
  icon?: string,
  name: string,
  mc_version: string,
  mod_loader: ModLoader,
  mod_loader_version: string
}>();
</script>