<script setup lang="ts">
import Navbar from './components/Navbar.vue';
import NavigationSidebar from './components/NavigationSidebar.vue';

</script>

<template>
  <main class="bg-[#1b1d21] h-screen w-screen flex flex-col overflow-y-hidden">
    <Navbar />
    <div class="flex w-full h-full">
      <NavigationSidebar />
      <div class="w-full h-full">
        <router-view class="bg-neutral-900 w-full h-full p-2 rounded-tl-2xl border-neutral-700 border-t-2 border-l-2" />
      </div>
    </div>
  </main>
</template>
