<script setup lang="ts">
import { emit, listen } from "@tauri-apps/api/event";
import { onMounted, onUnmounted, Ref, ref } from "vue";
import { Progress } from "@/types/instances";

type CurrentProgress = {
    current: number;
    total: number;
};

function progPrec(progress: CurrentProgress): string {
    if (progress.total === 0) return "??%";
    const percent = ((progress.current / progress.total) * 100).toFixed(2);
    return `${percent}%`;
}

function displayTotal(progress: CurrentProgress): string {
    const percent = progPrec(progress);
    const totalMiBs = (progress.total / (1024 * 1024)).toFixed(2);
    const currentMiBs = (progress.current / (1024 * 1024)).toFixed(2);
    return `${percent} (${currentMiBs} MiBs / ${totalMiBs} MiBs)`;
}

var progressMessage = ref("Loading...");
var currentProgresses: Ref<Map<string, CurrentProgress>> = ref(new Map());
var unlisteners: (() => void)[] = [];

const onInit = async () => {
    console.log("ProgressPage.vue mounted");

    const unlistenerBegin = await listen("progress-begin", (event) => {
        progressMessage.value = event.payload as string;
        currentProgresses.value = new Map();
        console.log("Progress started for ", progressMessage.value);
    });

    const unlistenerUpdate = await listen("progress-update", (event) => {
        const payload = event.payload as Progress;
        console.log("Progress updated for ", payload.url);
        currentProgresses.value.set(payload.url, {
            current: payload.current_bytes,
            total: payload.total_bytes,
        });
        currentProgresses.value = new Map(currentProgresses.value);
    });

    const unlistenerDone = await listen("progress-done", () => {
        currentProgresses.value = new Map();
        progressMessage.value = "Done";
    });

    const unlistenerStop = await listen("progress-stop", (event) => {
        const url = event.payload as string;
        currentProgresses.value.delete(url);
        currentProgresses.value = new Map(currentProgresses.value);
    });

    await emit("progress-ready");
    unlisteners = [
        unlistenerBegin,
        unlistenerUpdate,
        unlistenerDone,
        unlistenerStop,
    ];
};

onUnmounted(async () => {
    await emit("progress-cancel");
    unlisteners.forEach((unlistener) => unlistener());
});

onMounted(onInit);
</script>

<template>
    <div>
        <h1>
            {{ progressMessage }}
        </h1>
        <ul>
            <li v-for="[url, progress] in currentProgresses" :key="url">
                {{ url }}: {{ displayTotal(progress) }}
            </li>
        </ul>
    </div>
</template>

<style scoped>
h1,
li {
    color: white;
}
</style>
