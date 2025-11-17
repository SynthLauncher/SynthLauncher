<template>
    <main
        class="bg-[#1b1d21] h-screen w-screen flex flex-col overflow-y-hidden"
    >
        <!-- FIXME: I had to do this because of the unscalable page design, each page should be wrapped in a layout component which has the navbar instead of this shit, App shouldn't be more than styles -->
        <div v-if="$route.path == '/progress'">
            <ProgressPage />
            <!-- <router-view
                class="bg-neutral-900 w-full h-full p-2 rounded-tl-2xl border-neutral-700 border-t-2 border-l-2"
            /> -->
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
                        <!-- <ProgressBar v-model:progressValue="progressValue" :downloadLabel="currentDownload" /> -->
                    </div>
                    <AccountsSidebar />
                </div>
            </div>
        </div>
    </main>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, Suspense } from "vue";
import AccountsSidebar from "./components/AccountsSidebar.vue";
import Navbar from "./components/Navbar.vue";
import NavigationSidebar from "./components/NavigationSidebar.vue";
import ProgressPage from "./pages/ProgressPage.vue";
// import ProgressBar from './components/ProgressBar.vue';

const progressValue = ref(0);
// const currentDownload = ref("Sigma Pack")

let frameId: number;
let lastTime = 0;

function step(timestamp: number) {
    if (!lastTime) lastTime = timestamp;
    const delta = (timestamp - lastTime) / 1000;
    lastTime = timestamp;

    progressValue.value += delta * 20;

    if (progressValue.value >= 100) {
        progressValue.value = 100;
        return;
    }

    frameId = requestAnimationFrame(step);
}

onMounted(() => {
    frameId = requestAnimationFrame(step);
});

onBeforeUnmount(() => {
    cancelAnimationFrame(frameId);
});
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
