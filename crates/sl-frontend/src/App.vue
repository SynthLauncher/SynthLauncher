<template>
    <main
        class="bg-[#1b1d21] h-screen w-screen flex flex-col overflow-y-hidden"
    >
        <!-- FIXME: I had to do this because of the unscalable page design, each page should be wrapped in a layout component which has the navbar instead of this shit, App shouldn't be more than styles -->
        <div v-if="$route.path == '/progress'">
            <ProgressPage />
        </div>
        <div v-else>
            <Navbar />
            <div class="h-full flex w-full overflow-hidden">
                <NavigationSidebar />
                <div class="flex w-full h-full">
                    <div class="relative flex w-full h-full">
                        <router-view
                            class="bg-neutral-900 w-full h-full p-2 rounded-tl-2xl border-neutral-700 border-t-2 border-l-2"
                        />
                    </div>
                    <AccountsSidebar />
                </div>
            </div>
        </div>
    </main>
</template>

<script setup lang="ts">
import AccountsSidebar from "@/components/layout/AccountsSidebar.vue";
import Navbar from "@/components/layout/Navbar.vue";
import NavigationSidebar from "@/components/layout/NavigationSidebar.vue";
import ProgressPage from "@/pages/ProgressPage.vue";
</script>

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
