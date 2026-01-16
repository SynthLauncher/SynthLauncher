<script setup lang="ts">
import { Download, User } from 'lucide-vue-next';

defineProps<{
    modpack_banner_url: string,
    modpack_icon_url: string,
    source_icon_url: string,
    modpack_title: string,
    modpack_description: string,
    modpack_downloads: string,
    modpack_author: string
}>();
</script>

<template>
    <div class="flex-1 bg-[#1f1f22] rounded-xl overflow-hidden flex flex-col w-96">
        <div class="h-48 relative">
            <div class="relative w-full h-full">
                <img :src="modpack_banner_url" alt="Banner" class="w-full h-full object-cover" />
                <div class="absolute inset-0 bg-black/20"></div>
            </div>

            <img :src="modpack_icon_url" alt="icon" class="absolute bottom-2 left-2" :width="45" :height="45">
            <div class="absolute top-2 right-2 bg-black/60 p-1 rounded-xl">
                <img :src="source_icon_url" alt="Modrinth Icon" :width="35" :height="35">
            </div>
        </div>

        <div class="p-4 flex flex-col gap-2">
            <h1 class="text-xl font-semibold text-white">
                {{ modpack_title }}
            </h1>

            <p class="text-sm text-[#8d8d91] line-clamp-2">
                {{ modpack_description }}
            </p>

            <div class="flex justify-between mt-2">
                <div class="flex items-center gap-1 text-[#7b7b83]">
                    <User :size="18" />
                    <span class="text-sm">{{ modpack_author }}</span>
                </div>

                <div class="flex items-center gap-1 text-[#7b7b83]">
                    <Download :size="18" />
                    <span class="text-sm">{{ modpack_downloads }}</span>
                </div>
            </div>
        </div>
    </div>
</template>