<script setup lang="ts">
import { useRoute } from 'vue-router'
import { bytesToImageUrl } from '@/lib/utils';
import { instancesManager } from '@/lib/managers/instances';
import { Joystick, MoreVertical, PlayIcon, Settings } from 'lucide-vue-next';
import { ContentList, contentManager } from '@/lib/managers/content';
import { onMounted, ref } from 'vue';

const route = useRoute()
const instanceName = route.params.instance_name as string
const instanceMetadata = instancesManager.get(instanceName)

const list = ref<ContentList>()
const showInstanceMenu = ref(false)

onMounted(async () => {
  list.value = await contentManager.get_content_list(instanceName)
  console.log(list.value)
})
</script>

<template>
  <div class="flex flex-col gap-5 h-full w-full p-6">
    <div class="bg-background border-2 border-border rounded-lg w-full flex items-center justify-between gap-4 p-4">
      <div class="flex gap-4 items-center">
        <div class="bg-foreground/70 p-2 rounded-lg border-2 border-border">
          <img
            :src="instanceMetadata?.icon ? bytesToImageUrl(instanceMetadata.icon as Uint8Array) : '/default_instance_icon.png'"
            class="size-24 rounded-md object-cover">
        </div>
        <div class="flex flex-col gap-2">
          <h1 class="text-white text-3xl font-bold">
            {{ instanceMetadata?.name }}
          </h1>
          <div class="flex items-center gap-2 text-gray-400 text-base">
            <Joystick :size="18" />
            <span>{{ instanceMetadata?.mc_version }} {{ instanceMetadata?.mod_loader }} {{ instanceMetadata?.mod_loader_version }} </span>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <button @click.stop="async () => await instancesManager.launch_instance(instanceName)" class="flex items-center gap-2 text-white text-base cursor-pointer bg-accent 
                px-5 py-2.5 rounded-lg font-medium 
                 hover:bg-hover-accent/80 transition-colors">
          <PlayIcon :size="20" :stroke-width="2" />
          Play
        </button>

        <button class="p-2.5 bg-background border-2 border-border rounded-lg text-gray-400 
                 hover:text-white hover:bg-primary/50 transition-colors">
          <Settings :size="20" />
        </button>

        <div class="relative">
          <button @click="showInstanceMenu = !showInstanceMenu" class="p-2.5 bg-background border-2 border-border rounded-lg text-gray-400 
                   hover:text-white hover:bg-primary/50 transition-colors">
            <MoreVertical :size="20" />
          </button>

          <transition enter-active-class="transition ease-out duration-100" enter-from-class="opacity-0 scale-95"
            enter-to-class="opacity-100 scale-100" leave-active-class="transition ease-in duration-75"
            leave-from-class="opacity-100 scale-100" leave-to-class="opacity-0 scale-95">
            <div v-if="showInstanceMenu" class="absolute right-0 mt-2 w-48 bg-background border-2 border-border 
                     rounded-lg shadow-xl overflow-hidden z-50">
              <button class="w-full px-4 py-2 text-left text-sm text-gray-300 hover:bg-primary/50 transition-colors">
                Duplicate Instance
              </button>
              <button class="w-full px-4 py-2 text-left text-sm text-gray-300 hover:bg-primary/50 transition-colors">
                Export Instance
              </button>
              <button class="w-full px-4 py-2 text-left text-sm text-gray-300 hover:bg-primary/50 transition-colors">
                Open Folder
              </button>
              <div class="border-t-2 border-border"></div>
              <button class="w-full px-4 py-2 text-left text-sm text-red-400 hover:bg-red-500/10 transition-colors">
                Delete Instance
              </button>
            </div>
          </transition>
        </div>
      </div>
    </div>

    <!-- The instance tab -->
    <div class="bg-background rounded-md h-full w-full flex gap-4 p-4">

    </div>
  </div>
</template>
