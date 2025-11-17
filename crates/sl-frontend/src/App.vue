<template>
    <main class="bg-[#1b1d21] h-screen w-screen flex flex-col overflow-y-hidden">
        <router-view />
    </main>
</template>

<style>
::-webkit-scrollbar {
    width: 8px;
}

::-webkit-scrollbar-thumb {
    background-color: #4e4c57;
    border-radius: 10px;
    border: 2px solid #282c32;
    opacity: 0.5;
}

::-webkit-scrollbar-track {
    background: #181a1e;
    border-radius: 10px;
}

::-webkit-scrollbar-button {
    display: none;
}
</style>
