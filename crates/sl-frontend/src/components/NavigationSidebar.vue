<script setup lang="ts">
import { Folder, Home, Library, Settings, Store } from 'lucide-vue-next';
import NavigationSidebarItem from './NavigationSidebarItem.vue';
import { useRoute, useRouter } from 'vue-router';

const router = useRouter();
const route = useRoute();
</script>

<template>
  <div class="flex flex-col items-center justify-between p-2 bg-[#1b1d21]">
    <div class="flex flex-col gap-1">
      <NavigationSidebarItem
        id="home"
        label="Home"
        :icon="Home"
        :active="route.path === '/'"
      />
      <NavigationSidebarItem
        id="instance"
        label="Instances"
        :icon="Library"
        :active="route.path === '/instances'"
      />
      <NavigationSidebarItem
        id="store"
        label="Store"
        :icon="Store"
        :active="route.path === '/store'"
      />
    </div>

    <div class="flex flex-col gap-1">
      <NavigationSidebarItem
        id="folder"
        label="Folder"
        :icon="Folder"
      />
      <NavigationSidebarItem
        id="settings"
        label="Settings"
        :icon="Settings"
      />
    </div>
  </div>
</template>
