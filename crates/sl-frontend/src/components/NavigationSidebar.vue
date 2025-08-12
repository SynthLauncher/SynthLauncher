<template>
  <div class="bg-[#1b1d21] flex flex-col items-center justify-between p-2">
    <div class="flex flex-col gap-1">
      <NavigationSidebarItem id="home" label="Home" :icon="Home" :active="route.path === '/'"
        :onClick="() => router.push('/')" />
      <NavigationSidebarItem id="instance" label="Instances" :icon="Library" :active="route.path === '/instances'"
        :onClick="() => router.push('/instances')" />
      <NavigationSidebarItem id="store" label="Store" :icon="Store" :active="route.path === '/store'"
        :onClick="() => router.push('/store')" />
    </div>

    <div class="flex flex-col gap-1">
      <NavigationSidebarItem id="folder" label="Folder" :icon="Folder" :onClick="() => openSynthLauncherRootFolder()" />
      <NavigationSidebarItem id="settings" label="Settings" :icon="Settings" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { Folder, Home, Library, Settings, Store } from 'lucide-vue-next';
import { useRoute, useRouter } from 'vue-router';
import { openSynthLauncherRootFolder } from '../lib/commands/launcher';
import NavigationSidebarItem from './NavigationSidebarItem.vue';

const router = useRouter();
const route = useRoute();
</script>
