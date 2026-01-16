<script setup lang="ts">
import { bytesToImageUrl } from '@/lib/utils';
import { InstanceMetadata } from '@/types/instances';
import { useRouter } from 'vue-router';

defineProps<{
    instance: InstanceMetadata
}>()

const router = useRouter()
</script>

<template>
    <button @click="router.push(`/instance/${instance.name}`)" class="flex items-center px-3 py-3 
        rounded-full cursor-pointer transition-all 
        active:scale-95 bg-transparent text-[#b0b8c7]
        hover:bg-[#9eb0b8]/20 hover:text-[#c2cbcf]">
        <div class="w-6 h-6 flex items-center justify-center overflow-visible">
            <img :src="instance.icon ? bytesToImageUrl(instance.icon as Uint8Array) : '/default_instance_icon.png'"
                class="max-w-full max-h-full object-contain scale-[135%]" />
        </div>
    </button>
</template>