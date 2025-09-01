<template>
    <div class="flex flex-col items-center justify-center py-16 px-6">
        <div class="relative mb-4">
            <Loader2 class="animate-spin w-8 h-8 text-blue-400" />
            <div class="absolute inset-0 w-8 h-8 border-2 border-blue-400/20 rounded-full animate-pulse"></div>
        </div>
        <h3 class="text-lg font-semibold text-white mb-2">Loading Content</h3>
        <p class="text-neutral-400 text-center">
            Searching {{ storeType }} for the best {{ category }}...
        </p>
    </div>
</template>

<script setup lang="ts">
import { Loader2 } from 'lucide-vue-next';
import { StoreCategoryType, StoreType } from '../types/store';

defineProps<{
    storeType: StoreType
    category: StoreCategoryType
}>();
</script>