<script setup lang="ts">
import { Plus } from 'lucide-vue-next'
import { ref } from 'vue'
import Dialog from '@/components/ui/Dialog.vue'
import { ModLoader } from '@/types/instances'
import { instancesManager } from '@/lib/managers/instances'

const isOpen = ref(false)
const showContent = ref(false)

const instanceName = ref('')
const modLoader = ref<ModLoader>('Vanilla')

const isCreating = ref(false)

async function onClickCreateInstance() {
    if (isCreating.value) return

    isCreating.value = true

    try {
        await instancesManager.create_instance(
            instanceName.value,
            '1.12.2',
            modLoader.value,
            undefined
        )

        isOpen.value = false
    } catch (err) {
        console.error('Failed to create instance:', err)
    } finally {
        isCreating.value = false
    }
}
</script>

<template>
    <Dialog v-model:isOpen="isOpen" v-model:showContent="showContent" class="bg-black/60">
        <template #trigger>
            <button
                class="bg-gray-800/50 hover:bg-sky-300/20
                       hover:cursor-pointer rounded-lg h-full p-4 border-3
                       border-dashed border-gray-700 hover:border-sky-600/50
                       transition-colors flex items-center justify-center group"
                @click="isOpen = true"
            >
                <div
                    class="shrink-0 w-12 h-12 rounded-full bg-gray-700
                           flex items-center justify-center transition-colors
                           group-hover:bg-gray-600"
                >
                    <Plus class="text-gray-400 group-hover:text-gray-300 size-8" />
                </div>
            </button>
        </template>

        <template #content>
            <div
                v-if="showContent"
                class="bg-gradient-to-br from-[#1a1c20] to-[#16181c]
                       p-8 rounded-2xl shadow-2xl text-white
                       w-[480px] max-w-[90%] relative
                       border border-gray-800/50"
            >
                <h2 class="text-2xl font-bold mb-2">Create New Instance</h2>
                <p class="text-sm text-gray-400 mb-6">
                    Configure your instance settings below
                </p>

                <div class="flex flex-col gap-5">
                    <div class="flex flex-col gap-2">
                        <label class="text-sm font-medium text-gray-300">
                            Instance Name
                        </label>
                        <input
                            v-model="instanceName"
                            type="text"
                            placeholder="My Awesome Modpack"
                            maxlength="50"
                            :disabled="isCreating"
                            class="text-sm w-full bg-neutral-800/60 text-white
                                   border-2 border-neutral-700
                                   placeholder-neutral-500
                                   focus:border-sky-500 focus:bg-neutral-800
                                   focus:outline-none transition-all duration-200
                                   rounded-xl px-4 py-3
                                   disabled:opacity-50"
                        />
                    </div>

                    
                </div>

                <div class="flex justify-end gap-3 mt-8">
                    <button
                        @click="!isCreating && (isOpen = false)"
                        :disabled="isCreating"
                        class="px-5 py-2.5 rounded-lg bg-neutral-800
                               hover:bg-neutral-800 text-gray-300
                               transition-colors duration-200
                               text-sm font-medium
                               disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer"
                    >
                        Cancel
                    </button>

                    <button
                        @click="onClickCreateInstance"
                        :disabled="isCreating || !instanceName"
                        class="px-5 py-2.5 rounded-lg bg-sky-500
                               hover:bg-sky-600 text-white
                               transition-all duration-200
                               text-sm font-medium
                               flex items-center gap-2 justify-center
                               disabled:bg-sky-500/50
                               disabled:cursor-not-allowed cursor-pointer"
                    >
                        <template v-if="isCreating">
                            <div
                                class="w-4 h-4 border-2 border-white/30
                                       border-t-white rounded-full animate-spin"
                            ></div>
                            Creating…
                        </template>
                        <template v-else>
                            Create Instance
                        </template>
                    </button>
                </div>
            </div>
        </template>
    </Dialog>
</template>
