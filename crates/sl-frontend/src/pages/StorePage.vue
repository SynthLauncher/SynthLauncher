<script setup lang="ts">
import { onMounted, watch } from 'vue';
import { storeManager } from '@/lib/managers/store';
import HorizontalOptionSelector from '@/components/ui/HorizontalOptionSelector.vue';
import StoreCard from '@/components/store/StoreCard.vue';
import StoreSearchBar from '@/components/store/StoreSearchBar.vue';
import StoreLoadingState from '@/components/store/StoreLoadingState.vue';
import InstanceSelector from '@/components/store/InstanceSelector.vue';
import ContentVersionSelector from '@/components/store/ContentVersionSelector.vue';

onMounted(storeManager.loadSearch);

watch([
  () => storeManager.searchQuery, 
  () => storeManager.storeCategory, 
  () => storeManager.storeType
], storeManager.loadSearch)
// watch([() => storeManager.selectedContent.slug, () => storeManager.selectedInstance], storeManager.loadContentVersions)
</script>

<template>
  <main class="flex flex-col gap-3 p-6 pb-18 overflow-y-auto overflow-x-hidden">
    <div class="flex items-center gap-2">
      <HorizontalOptionSelector v-model:selectedValue="storeManager.storeType" :values="['Modrinth', 'Curseforge']" />
      <HorizontalOptionSelector v-model:selectedValue="storeManager.storeCategory"
        :values="['modpacks', 'mods', 'shaderpacks', 'resourcepacks']" />
      <InstanceSelector v-show="storeManager.storeCategory != 'modpacks'" />
      <ContentVersionSelector />

      <!-- <button class="p-3 bg-white" @click="storeManager.installContent">
        Install
      </button> -->
    </div>

    <StoreSearchBar v-model:searchQuery="storeManager.searchQuery" :category="storeManager.storeCategory" />
    <StoreLoadingState v-if="storeManager.loading" :storeSource="storeManager.storeType" :category="storeManager.storeCategory" />

    <div v-else class="flex flex-col gap-3">
      <StoreCard 
        v-if="storeManager.items != undefined" 
        v-for="item in storeManager?.items.hits" 
        :key="item.name" :title="item.name" :description="item.description"
        :downloads="item.downloads" :author="item.author" :icon="item.icon_url" :slug="item.slug"
      />
    </div>

  </main>
</template>
