<script setup lang="ts">
import { Earth, Folder, Home, Library, Plus, Shirt, Store } from 'lucide-vue-next';
import { useRoute, useRouter } from 'vue-router';
import { openSynthLauncherRootFolder } from '@/lib/commands/launcher';
import { openDiscordLink } from '@/lib/utils';
import NavigationSidebarItem from '@/components/layout/NavigationSidebarItem.vue';
import SettingsDialog from '@/components/layout/SettingsDialog.vue';
import RecentInstance from '../RecentInstance.vue';
import { instancesManager } from '@/lib/managers/instances';
import HorizontalSeparator from '../HorizontalSeparator.vue';

const router = useRouter();
const route = useRoute();

</script>

<template>
  <div class="bg-background flex flex-col items-center justify-between px-3 h-full">
    <div class="flex flex-col items-center gap-1 relative">
      <NavigationSidebarItem id="home" label="home" :icon="Home" :active="route.path === '/'"
        :onClick="() => router.push('/')" />
      <NavigationSidebarItem id="instances" label="Instances" :icon="Library" :active="route.path === '/instances'"
        :onClick="() => router.push('/instances')" />
      <NavigationSidebarItem id="store" label="Store" :icon="Store" :active="route.path === '/store'"
        :onClick="() => router.push('/store')" />
      <NavigationSidebarItem id="cosmetics" label="Cosmetics" :icon="Shirt" :active="route.path === '/cosmetics'"
        :onClick="() => router.push('/cosmetics')" />
      <NavigationSidebarItem id="servers" label="Servers" :icon="Earth" :active="route.path === '/servers'"
        :onClick="() => router.push('/servers')" />

      <HorizontalSeparator />
      
      <button :class="[
        'flex items-center gap-3 px-3 py-3 rounded-full cursor-pointer transition-all active:scale-95 select-none bg-transparent text-[#b0b8c7] hover:bg-[#9eb0b8]/20 hover:text-[#c2cbcf]',
      ]">

        <Plus :size="24" />
      </button>

      <RecentInstance 
        v-for="instance in instancesManager.instances.slice(0, 3)"
        :instance="instance" 
      />
    </div>

    <div class="flex flex-col gap-1">
      <!-- <NavigationSidebarItem id="discord" label="Discord" :icon='"/discord_icon.svg"' :onClick="openDiscordLink" /> -->
      <NavigationSidebarItem id="folder" label="Folder" :icon="Folder" :onClick="() => openSynthLauncherRootFolder()" />
      <SettingsDialog />
    </div>
  </div>
</template>