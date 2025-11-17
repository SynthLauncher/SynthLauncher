<script setup lang="ts">
import AccountsSidebar from '@/components/layout/AccountsSidebar.vue';
import Navbar from '@/components/layout/Navbar.vue';
import NavigationSidebar from '@/components/layout/NavigationSidebar.vue';
</script>

<template>
  <main class="bg-[#1b1d21] h-screen w-screen flex flex-col overflow-y-hidden">
    <Navbar />
    <div class="h-full flex w-full overflow-hidden">
      <NavigationSidebar />
      <div class="flex w-full h-full">
        <div class="relative flex w-full h-full">
          <router-view 
            class=" bg-neutral-900 w-full h-full p-2 rounded-tl-2xl border-neutral-700 border-t-2 border-l-2" />
        </div>
        <AccountsSidebar />
      </div>
    </div>
  </main>
</template>
