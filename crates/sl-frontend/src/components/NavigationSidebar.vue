<template>
  <div class="bg-[#1b1d21] flex flex-col items-center justify-between p-2 h-full">
    <div class="flex flex-col gap-1">
      <NavigationSidebarItem id="instance" label="Instances" :icon="Library" :active="route.path === '/'"
        :onClick="() => router.push('/')" />
      <NavigationSidebarItem id="store" label="Store" :icon="Store" :active="route.path === '/store'"
        :onClick="() => router.push('/store')" />
    </div>

    <div class="flex flex-col gap-1">
      <NavigationSidebarItem id="discord" label="Discord" :icon='"/discord_icon.svg"' :onClick="() => openDiscordLink()" />
      <NavigationSidebarItem id="folder" label="Folder" :icon="Folder" :onClick="() => openSynthLauncherRootFolder()" />
      <SettingsDialog />
    </div>
  </div>
</template>

<script setup lang="ts">
import { Folder, Library, Store } from 'lucide-vue-next';
import { useRoute, useRouter } from 'vue-router';
import { openSynthLauncherRootFolder } from '../lib/commands/launcher';
import NavigationSidebarItem from './NavigationSidebarItem.vue';
import SettingsDialog from './SettingsDialog.vue';

const router = useRouter();
const route = useRoute();

import { open } from '@tauri-apps/plugin-shell';

async function openDiscordLink() {
  try {
    await open('https://discord.gg/ajZux2Uy9E')
  } catch (e) {
    console.error('Failed to open link:', e)
  }
}
</script>
