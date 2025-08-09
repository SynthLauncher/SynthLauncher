<script setup lang="ts">
import Navbar from './components/Navbar.vue';
import NavigationSidebar from './components/NavigationSidebar.vue';

</script>

<template>
  <main class="bg-neutral-900 h-screen w-screen flex flex-col overflow-y-hidden">
    <Navbar />
    <div class="flex w-full h-full">
      <NavigationSidebar />
      <router-view />
    </div>
  </main>
</template>
