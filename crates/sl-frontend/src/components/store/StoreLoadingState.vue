<script setup lang="ts">
import { StoreContentCategory, StoreSource } from '@/types/store';
import { Loader2 } from 'lucide-vue-next';

defineProps<{
    storeSource: StoreSource
    category: StoreContentCategory
}>();
</script>

<template>
    <div class="flex flex-col items-center justify-center py-16 px-6">
        <div class="relative mb-4">
            <Loader2 class="animate-spin w-8 h-8 text-secondary-accent-foreground" />
            <div class="absolute inset-0 w-8 h-8 border-2 border-accent/20 rounded-full animate-pulse"></div>
        </div>
        <h3 class="text-lg font-semibold text-white mb-2">Loading Content</h3>
        <p class="text-neutral-400 text-center">
            Searching {{ storeSource }} for the best {{ category }}...
        </p>
    </div>
</template>

