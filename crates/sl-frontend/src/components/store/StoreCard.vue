<script setup lang="ts">
import { DownloadIcon } from 'lucide-vue-next';
import { storeManager } from '@/lib/managers/store';

withDefaults(defineProps<{
    icon?: string,
    title: string,
    slug: string,
    author: string,
    description: string,
    downloads: number,
}>(), {
    icon: 'https://cdn.modrinth.com/placeholder.svg'
});
</script>

<template>
    <div :class="[
        'flex items-center justify-between gap-5 p-3 rounded-lg border-2 transition-colors duration-300',
        storeManager.selectedContents.has(slug) ? 'border-sky-500' : 'border-[#4e4e4e]',
        storeManager.selectedContent.slug == slug ? 'bg-[#26546c]/50' : 'bg-[#202227]',
    ]"
    @click="storeManager.selectContent(slug)"
    >
        <div class="flex gap-4">
            <img :src="icon" alt="Store Card Icon" class="bg-[#454956]/30 border-1 border-[#42444a] w-24 h-24 rounded-lg object-cover">

            <div class="flex flex-col gap-2">
                <h1 class="text-white text-2xl font-bold truncate">
                    {{ title }}

                    <span class="text-neutral-400 text-sm font-normal">
                        by {{ author }}
                    </span>
                </h1>

                <p class="text-gray-400 text-sm mb-1 line-clamp-1">{{ description }}</p>

                <div class="flex gap-1 text-gray-400 text-sm items-center">
                    <DownloadIcon class="w-4 h-4" />
                    <span>{{ downloads.toLocaleString("en-US", undefined) }} downloads</span>
                </div>
            </div>

        </div>
    
    </div>
</template>
