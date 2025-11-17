<script setup lang="ts">
import { Folder, Library, Store } from 'lucide-vue-next';
import { useRoute, useRouter } from 'vue-router';
import { openSynthLauncherRootFolder } from '@/lib/commands/launcher';
import { openDiscordLink } from '@/lib/utils';
import NavigationSidebarItem from '@/components/layout/NavigationSidebarItem.vue';
import SettingsDialog from '@/components/layout/SettingsDialog.vue';

const router = useRouter();
const route = useRoute();

</script>

<template>
  <div class="bg-[#1b1d21] flex flex-col items-center justify-between p-2 h-full">
    <div class="flex flex-col gap-1">
      <NavigationSidebarItem id="instance" label="Instances" :icon="Library" :active="route.path === '/'"
        :onClick="() => router.push('/')" />
      <NavigationSidebarItem id="store" label="Store" :icon="Store" :active="route.path === '/store'"
        :onClick="() => router.push('/store')" />
    </div>

    <div class="flex flex-col gap-1">
      <NavigationSidebarItem id="discord" label="Discord" :icon='"/discord_icon.svg"' :onClick="openDiscordLink" />
      <NavigationSidebarItem id="folder" label="Folder" :icon="Folder" :onClick="() => openSynthLauncherRootFolder()" />
      <SettingsDialog />
    </div>
  </div>
</template>