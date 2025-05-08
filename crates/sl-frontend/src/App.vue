<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { appWindow } from "@tauri-apps/api/window"
const greetMsg = ref("");
const name = ref("");

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}

function minimize() {
  appWindow.minimize();
}
</script>

<template>
  <div class="flex flex-col h-screen overflow-hidden">
    <div
      class="h-10 w-full flex items-center justify-between bg-gray-800 text-shadow-white px-4 select-none"
      style="-webkit-app-region: drag;"
    >
      <div class="font-bold text-white text-sm">SynthLauncher</div>
      <div class="flex gap-2 text-white" style="-webkit-app-region: no-drag;">
        <button @click="minimize" class="w-8 h-8 hover:bg-gray-700 rounded">-</button>
      </div>
    </div>
  </div>
</template>
