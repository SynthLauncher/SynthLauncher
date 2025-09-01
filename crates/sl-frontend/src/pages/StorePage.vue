<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import StoreCard from '../components/StoreCard.vue';
import StoreSearchBar from '../components/StoreSearchBar.vue';
import { StoreCategoryType, StoreSearch, StoreType } from '../types/store';
import { fetchStoreSearch } from '../lib/commands/store';
import StoreLoadingState from '../components/StoreLoadingState.vue';
import OptionSelector from '../components/OptionSelector.vue';

const storeType = ref<StoreType>('modrinth');
const storeCategory = ref<StoreCategoryType>("modpacks");

const searchQuery = ref("");
const storeItems = ref<StoreSearch>();

const storePage = ref(1);
const loading = ref(false);

const loadStoreItems = async () => {
  loading.value = true;
  try {
    const result = await fetchStoreSearch(searchQuery.value, storeType.value, storeCategory.value, storePage.value);
    storeItems.value = result;
  } catch (e) {
    console.error(e);
    storeItems.value = undefined;
  } finally {
    loading.value = false;
  }
};

onMounted(loadStoreItems);
watch([searchQuery, storeCategory, storeType], loadStoreItems)

const combinedItems = computed(() => {
  if (!storeItems.value) return [];

  if ('hits' in storeItems.value) {
    return storeItems.value.hits.map(item => ({
      id: item.id,
      title: item.title,
      slug: item.slug,
      description: item.description,
      downloads: item.downloads,
      author: item.author,
      icon: item.icon_url || 'https://cdn.modrinth.com/placeholder.svg'
    }));
  }
  if ('data' in storeItems.value) {
    return storeItems.value.data.map(item => ({
      id: item.id,
      title: item.name,
      slug: item.slug,
      description: item.summary,
      downloads: 0,
      author: item.authors[0].name,
      icon: item.logo?.url || 'https://cdn.modrinth.com/placeholder.svg'
    }));
  }
  return [];
});
</script>

<template>
  <main class="flex flex-col gap-3 p-6 pb-18 overflow-y-auto overflow-x-hidden">
    <div class="flex gap-2">
      <OptionSelector v-model:selectedValue="storeType" :values="['modrinth', 'curseforge']" />
      <OptionSelector v-model:selectedValue="storeCategory" :values="['modpacks', 'mods', 'shaderpacks', 'resourcepacks']" />
    </div>
    
    <StoreSearchBar v-model:searchQuery="searchQuery" :category="storeCategory" />

    <StoreLoadingState v-if="loading" :storeType="storeType" :category="storeCategory" />

    <div v-else class="flex flex-col gap-3">
      <StoreCard v-for="item in combinedItems" :key="item.id" :title="item.title" :description="item.description"
        :downloads="item.downloads" :author="item.author" :icon="item.icon" :slug="item.slug" />
    </div>
  </main>
</template>
