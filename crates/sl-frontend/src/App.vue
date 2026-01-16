<script lang="ts" setup>
</script>

<template>
    <main class="bg-background h-screen w-screen flex flex-col overflow-y-hidden">
        <router-view :key="$route.fullPath" />
    </main>
</template>

<style>
::-webkit-scrollbar {
    width: 10px;
}

::-webkit-scrollbar-thumb {
    background-color: #5c5870;
    border-radius: 8px;
    opacity: 0.5;
}

::-webkit-scrollbar-track {
    background: transparent;
    border-radius: 10px;
}

::-webkit-scrollbar-button {
    display: none;
}
</style>
