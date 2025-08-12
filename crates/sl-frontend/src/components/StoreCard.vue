<template>
    <div class="flex gap-4 p-4 bg-[#202227] rounded-lg">
        <img :src="icon" alt="Store Card Icon" class="w-24 h-24 rounded-md">

        <div class="flex flex-col gap-2">
            <h1 class="text-white text-2xl font-bold">
                {{ title }}

                <span class="text-neutral-400 text-sm font-normal">
                    by {{ author }}
                </span>
            </h1>

            <p class="text-gray-400 text-sm mt-1 line-clamp-2">{{ description }}</p>

            <div class="flex gap-1 text-gray-400 text-sm items-center">
                <DownloadIcon class="w-4 h-4" />
                <span>{{ downloads }} downloads</span>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { DownloadIcon } from 'lucide-vue-next';

defineProps<{
    icon?: any,
    title: string,
    author: string,
    description: string,
    downloads: number,
}>();
</script>
