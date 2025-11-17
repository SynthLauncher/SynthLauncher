<script setup lang="ts">
import { computed, onMounted, watch } from 'vue';
import HorizontalOptionSelector from '@/components/ui/HorizontalOptionSelector.vue';
import { storeManager } from '@/lib/managers/store';
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
watch([() => storeManager.selectedContent.slug, () => storeManager.selectedInstance], storeManager.loadContentVersions)

const combinedItems = computed(() => {
  if (!storeManager.items) return [];

  if ('hits' in storeManager.items) {
    return storeManager.items.hits.map(item => ({
      id: item.id,
      title: item.title,
      slug: item.slug,
      description: item.description,
      downloads: item.downloads,
      author: item.author,
      icon: item.icon_url || 'https://cdn.modrinth.com/placeholder.svg'
    }));
  }
  if ('data' in storeManager.items) {
    return storeManager.items.data.map(item => ({
      id: item.id,
      title: item.name,
      slug: item.slug,
      description: item.summary,
      downloads: item.downloadCount,
      author: item.authors[0].name,
      icon: item.logo?.url || 'https://cdn.modrinth.com/placeholder.svg'
    }));
  }
  return [];
});
</script>

<template>
  <main class="flex flex-col gap-3 p-6 pb-18 overflow-y-auto overflow-x-hidden">
    <div class="flex items-center gap-2">
      <HorizontalOptionSelector v-model:selectedValue="storeManager.storeType" :values="['modrinth', 'curseforge']" />
      <HorizontalOptionSelector v-model:selectedValue="storeManager.storeCategory"
        :values="['modpacks', 'mods', 'shaderpacks', 'resourcepacks']" />
      <InstanceSelector v-show="storeManager.storeCategory != 'modpacks'" />
      <ContentVersionSelector />

      <button class="p-3 bg-white" @click="storeManager.installContent">
        Install
      </button>
    </div>

    <StoreSearchBar v-model:searchQuery="storeManager.searchQuery" :category="storeManager.storeCategory" />
    <StoreLoadingState v-if="storeManager.loading" :storeType="storeManager.storeType" :category="storeManager.storeCategory" />

    <div v-else class="flex flex-col gap-3">
      <StoreCard v-for="item in combinedItems" :key="item.id" :title="item.title" :description="item.description"
        :downloads="item.downloads" :author="item.author" :icon="item.icon" :slug="item.slug"
      />
    </div>

  </main>
</template>
