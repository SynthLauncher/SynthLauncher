<script setup lang="ts">
import { ref } from 'vue'
import { Settings, X } from 'lucide-vue-next'
import NavigationSidebarItem from '@/components/layout/NavigationSidebarItem.vue'
import Dialog from '@/components/ui/Dialog.vue'

const isOpen = ref(false)
const showContent = ref(false)
</script>

<template>
  <Dialog v-model:isOpen="isOpen" v-model:showContent="showContent" class="bg-black/60">
    <template #trigger>
      <NavigationSidebarItem :onClick="() => isOpen = true" id="settings" label="Settings" :icon="Settings" />
    </template>

    <template #content>
      <div v-if="showContent" class="bg-[#1b1d21] p-6 rounded-lg shadow-lg text-white w-[400px] max-w-[90%] relative">
        <button @click="isOpen = false" class="absolute top-2 right-2 text-gray-400 hover:text-gray-300 cursor-pointer">
          <X />
        </button>

        <h2 class="text-lg font-bold mb-2">Settings</h2>
        <p class="text-sm text-gray-400 mb-4">Make changes here. Click save when done.</p>

        <div class="flex justify-end mt-4">
          <button @click="isOpen = false" class="bg-sky-500 text-white px-4 py-2 rounded hover:bg-sky-600">
            Save
          </button>
        </div>
      </div>
    </template>
  </Dialog>

</template>

<style>
.modal-enter-active {
  animation: fade-in-and-slide-up 0.2s;
}

.modal-leave-active {
  animation: fade-in-and-slide-up 0.2s reverse;
}

.content-enter-active {
  animation: pop-up 0.2s;
}

.content-leave-active {
  transform: translateX(50px);
}

@keyframes pop-up {
  0% {
    transform: scale(0.75);
  }

  100% {
    transform: scale(1);
  }
}

@keyframes fade-in-and-slide-up {
  0% {
    opacity: 0;
  }

  100% {
    opacity: 1;
  }
}
</style>
