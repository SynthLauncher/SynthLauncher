<template>
    <div class="flex items-center justify-between gap-5 p-4 bg-[#202227] rounded-lg">
        <div class="flex gap-4">
            <img :src="icon" alt="Store Card Icon" class="w-24 h-24 rounded-lg object-cover">

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
                    <span>{{ downloads }} downloads</span>
                </div>
            </div>

        </div>

        <Dialog v-model:isOpen="isOpen" class="">
            <template #trigger>
                <button class="cursor-pointer text-white rounded-lg 
            border-2 border-sky-500 hover:bg-sky-500 transition-colors py-2 px-4"
                    @click='isOpen = true'>
                    Install
                </button>
            </template>

            <template #content>
                1
            </template>
        </Dialog>
    </div>
</template>

<script setup lang="ts">
import { DownloadIcon } from 'lucide-vue-next';
import Dialog from './ui/Dialog.vue';
import { ref } from 'vue';

const isOpen = ref(false);

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
