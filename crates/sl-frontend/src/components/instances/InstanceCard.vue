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
  <div
    class="flex items-center justify-between hover:bg-primary/50 hover:border-border-accent bg-background border-2 border-border py-3 px-4 rounded-lg transition-colors duration-300 hover:cursor-pointer">

    <div class="flex items-center gap-4 whitespace-nowrap min-w-0">
      <img :src="icon" class="shrink-0 object-cover size-16 rounded-md border-2 border-border bg-[#2a2a31]"
        alt="Instance Icon">

      <div class="leading-tight min-w-0">
        <h1 class="text-white text-lg whitespace-nowrap truncate">{{ name }}</h1>
        <p class="text-gray-400 text-sm whitespace-nowrap truncate">{{ mc_version }} {{ mod_loader }} {{
          mod_loader_version }}</p>
      </div>
    </div>

    <button @click.stop="async () => await instancesManager.launch_instance(name)" class="cursor-pointer flex items-center
        justify-center size-10 rounded-full bg-accent hover:bg-hover-accent transition-colors shrink-0 ml-2">
      <PlayIcon class="size-5 text-white" />
    </button>
  </div>
</template>
