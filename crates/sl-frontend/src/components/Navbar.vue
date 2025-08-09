<template>
  <nav
    class="w-full flex h-[3.4rem] justify-between items-center p-2 bg-[#1b1d21]"
    data-tauri-drag-region
  >
    <div class="flex items-center gap-2 ml-[0.4rem]">
      <img
        width="37"
        height="37"
        src="/images/synthlauncher-logo.png"
        alt="SynthLauncher Logo"
      />
      <h1 class="text-xl font-bold text-white">SynthLauncher</h1>
    </div>

    <div class="flex gap-[0.1rem]">
      <WindowControl
        @click="handleWindowMinimize"
        :icon="MinusIcon"
        label="Minimize Window"
        class="hover:bg-[#8fe21a]"
      />
      <WindowControl
        @click="handleWindowMaximize"
        :icon="MaximizeIcon"
        label="Maximize Window"
        class="hover:bg-[#fcbd35]"
      />
      <WindowControl
        @click="handleWindowClose"
        :icon="XIcon"
        label="Close Window"
        class="hover:bg-red-400"
      />
    </div>
  </nav>
</template>

<script setup lang="ts">
import { Minus as MinusIcon, Maximize as MaximizeIcon, X as XIcon } from 'lucide-vue-next'
import WindowControl from './WindowControl.vue'
import {
  handleWindowMinimize,
  handleWindowMaximize,
  handleWindowClose,
} from '../lib/commands/window'
</script>
