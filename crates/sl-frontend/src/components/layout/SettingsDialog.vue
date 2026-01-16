<script setup lang="ts">
import { ref } from 'vue'
import { Brush, Coffee, JoystickIcon, Settings, X } from 'lucide-vue-next'
import NavigationSidebarItem from '@/components/layout/NavigationSidebarItem.vue'
import Dialog from '@/components/ui/Dialog.vue'
import ToggleSwitch from '@/components/ui/ToggleSwitch.vue'
import HorizontalSeparator from '../HorizontalSeparator.vue'
import VerticalSeparator from '../VerticalSeparator.vue'

const isOpen = ref(false)
const showContent = ref(false)

const tabs = [
  {
    icon: Brush,
    name: 'Appearance'
  },
  {
    icon: Coffee,
    name: 'Java installations'
  },
  {
    icon: JoystickIcon,
    name: 'Default instance options'
  }
]
const curr_tab = ref(0);
</script>

<template>
  <Dialog v-model:isOpen="isOpen" v-model:showContent="showContent">
    <template #trigger>
      <NavigationSidebarItem :onClick="() => isOpen = true" id="settings" label="Settings" :icon="Settings" />
    </template>
    
    <template #content>
      <div v-if="showContent" class="bg-background rounded-2xl p-6 w-[1000px] max-w-[90%] h-[600px] flex flex-col gap-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <Settings class="text-white text-xl" />
            <h2 class="text-white text-xl font-bold">Settings</h2>
          </div>
          <button 
            @click="isOpen = false" 
            class="cursor-pointer p-2 bg-primary/60 text-gray-400 hover:text-white transition-colors rounded-full hover:bg-primary/50"
          >
            <X class="w-5 h-5" />
          </button>
        </div>
        <HorizontalSeparator />

        <div class="flex gap-3 h-full">
          <div>
            <ul class="flex flex-col gap-1">
              <li v-for="(tab, idx) in tabs" :class="['px-3 py-2 rounded-2xl transition-colors',  (idx == curr_tab) ? 'bg-secondary-accent/50 text-secondary-accent-foreground' : 'text-gray-300 bg-transparent']">
                <button @click="() => curr_tab = idx" class="flex items-center gap-2 text-sm font-bold cursor-pointer active:scale-95 transition-all">
                  <component :is="tab.icon" :size="18" /> {{ tab.name }}
                </button>
              </li>
            </ul>
          </div>

          <VerticalSeparator />

          <div>

          </div>
        </div>

        <!-- <div class="space-y-3 max-h-[60vh] overflow-y-auto">
          <div class="flex items-center justify-between p-3 rounded-lg border-2 border-border bg-background hover:bg-primary/50 transition-colors duration-300">
            <div>
              <div class="text-white font-medium">Auto-Update</div>
              <div class="text-gray-400 text-sm">Automatically updates the game launcher</div>
            </div>
            <ToggleSwitch v-model="settings.autoUpdate" />
          </div>
        </div>

        <div class="flex justify-end gap-3 mt-6 pt-4 border-t-2 border-border">
          <button 
            @click="isOpen = false" 
            class="px-5 py-2 rounded-lg border-2 border-border text-gray-400 hover:text-white hover:bg-primary/50 transition-colors duration-300 font-medium"
          >
            Cancel
          </button>
          <button 
            @click="isOpen = false" 
            class="px-5 py-2 rounded-lg bg-secondary-accent border-2 border-secondary-accent hover:bg-secondary-accent/80 text-white transition-colors duration-300 font-medium"
          >
            Save Changes
          </button>
        </div> -->
      </div>
    </template>
  </Dialog>
</template>

<style scoped>
.modal-enter-active {
  animation: fade-in 0.2s ease-out;
}

.modal-leave-active {
  animation: fade-in 0.2s ease-in reverse;
}

.content-enter-active {
  animation: pop-up 0.25s ease-out;
}

.content-leave-active {
  animation: pop-down 0.2s ease-in;
}

@keyframes pop-up {
  0% {
    opacity: 0;
    transform: scale(0.9) translateY(-20px);
  }
  100% {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

@keyframes pop-down {
  0% {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
  100% {
    opacity: 0;
    transform: scale(0.9) translateY(20px);
  }
}

@keyframes fade-in {
  0% {
    opacity: 0;
  }
  100% {
    opacity: 1;
  }
}
</style>