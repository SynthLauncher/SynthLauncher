<script setup lang="ts">
import { SkinView3d } from 'vue-skinview3d';
</script>

<template>
    <main class="w-80 shrink-0 h-full border-neutral-700 border-t-2">
        <SkinView3d :width="300" :height="400"
            skin-url="https://textures.minecraft.net/texture/ebff9a96d423d5d74c96905b0493a4c9b4e3a48cdffed145ab53a7c5487061d4"
            :global-light="3" />
    </main>
</template>