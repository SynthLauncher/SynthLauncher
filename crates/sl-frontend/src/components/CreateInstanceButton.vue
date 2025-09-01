<script setup lang="ts">
import { Plus } from 'lucide-vue-next';
import Dialog from './ui/Dialog.vue';
import { ref } from 'vue';

const isOpen = ref(false)
</script>

<template>
    <Dialog v-model:isOpen="isOpen">
        <template #trigger>
            <button class="bg-gray-800/50 hover:bg-sky-300/20
        hover:cursor-pointer rounded-lg h-full p-4 border-3
        border-dashed border-gray-700 hover:border-sky-600/50
        transition-colors flex items-center justify-center group">
                <div
                    className="shrink-0 w-12 h-12 rounded-full bg-gray-700 flex items-center justify-center transition-colors group-hover:bg-gray-600">
                    <Plus class="text-gray-400 group-hover:text-gray-300 size-8" />
                </div>
            </button>
        </template>

        <template #content>
             
        </template>
    </Dialog>
</template>