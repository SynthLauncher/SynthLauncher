<script setup lang="ts">
import { Plus, Upload, ChevronDown } from 'lucide-vue-next'
import { ref, computed } from 'vue'
import Dialog from '@/components/ui/Dialog.vue'
import HorizontalSeparator from '@/components/HorizontalSeparator.vue'
import { ModLoader } from '@/types/instances'
import { instancesManager } from '@/lib/managers/instances'
import { launcherManager } from '@/lib/managers/launcher'

const isOpen = ref(false)
const showContent = ref(false)

const instanceName = ref('')
const mcVersion = ref('1.21.4')
const modLoader = ref<ModLoader>('Vanilla')
const modLoaderVersion = ref('')
const iconFile = ref<File | null>(null)
const iconPreview = ref<string>('/default_instance_icon.png')

const isCreating = ref(false)
const versionDropdownOpen = ref(false)
const versionSearch = ref('')

// Expanded version list
const allMcVersions = launcherManager.minecraftVersions;

const modLoaders: ModLoader[] = ['Vanilla', 'Fabric', 'Forge', 'Quilt', 'NeoForge']

// Lazy loading for version list
const displayLimit = ref(50)
const filteredVersions = computed(() => {
    const search = versionSearch.value.toLowerCase()
    const filtered = search 
        ? allMcVersions.filter(v => v.toLowerCase().includes(search))
        : allMcVersions    
    return filtered.slice(0, displayLimit.value)
})

const hasMoreVersions = computed(() => {
    const search = versionSearch.value.toLowerCase()
    const totalFiltered = search 
        ? allMcVersions.filter(v => v.toLowerCase().includes(search)).length
        : allMcVersions.length
    return displayLimit.value < totalFiltered
})

function loadMoreVersions() {
    displayLimit.value += 50
}

function resetVersionList() {
    displayLimit.value = 50
    versionSearch.value = ''
}

function selectVersion(version: string) {
    mcVersion.value = version
    versionDropdownOpen.value = false
    resetVersionList()
}

function handleIconUpload(event: Event) {
    const target = event.target as HTMLInputElement
    const file = target.files?.[0]
    
    if (file && file.type.startsWith('image/')) {
        iconFile.value = file
        const reader = new FileReader()
        reader.onload = (e) => {
            iconPreview.value = e.target?.result as string
        }
        reader.readAsDataURL(file)
    }
}

function onScroll(event: Event) {
    const target = event.target as HTMLElement
    const scrollPercentage = (target.scrollTop + target.clientHeight) / target.scrollHeight
    
    if (scrollPercentage > 0.8 && hasMoreVersions.value) {
        loadMoreVersions()
    }
}

async function onClickCreateInstance() {
    if (isCreating.value) return

    isCreating.value = true

    try {
        await instancesManager.create_instance(
            instanceName.value,
            mcVersion.value,
            modLoader.value,
            modLoaderVersion.value || undefined
        )

        isOpen.value = false
        instanceName.value = ''
        mcVersion.value = '1.21.4'
        modLoader.value = 'Vanilla'
        modLoaderVersion.value = ''
        iconFile.value = null
        iconPreview.value = '/default_instance_icon.png'
    } catch (err) {
        console.error('Failed to create instance:', err)
    } finally {
        isCreating.value = false
    }
}
</script>

<template>
    <Dialog v-model:isOpen="isOpen" v-model:showContent="showContent">
        <template #trigger>
            <button
                class="bg-background/50 hover:bg-primary/20
                       hover:cursor-pointer rounded-lg h-full p-4 border-3
                       border-dashed border-border-primary hover:border-border-accent
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
                class="bg-background rounded-2xl p-6 w-[700px] max-w-[90%] border-2 border-border"
            >
                <div class="flex items-center gap-2 mb-2">
                    <Plus class="text-white" :size="24" />
                    <h2 class="text-white text-2xl font-bold">Create New Instance</h2>
                </div>
                <p class="text-gray-400 text-sm mb-4">
                    Create new Minecraft instance
                </p>

                <HorizontalSeparator class="mb-6" />

                <div class="flex gap-6">
                    <div class="flex flex-col gap-2">
                        <label class="text-white font-medium text-sm">Instance Icon</label>
                        <div class="relative group">
                            <div class="w-32 h-32 rounded-lg overflow-hidden border-2 border-border bg-foreground/70">
                                <img :src="iconPreview" class="w-full h-full object-cover" />
                            </div>
                            <label 
                                class="absolute inset-0 flex items-center justify-center bg-black/60 
                                       opacity-0 group-hover:opacity-100 transition-opacity cursor-pointer rounded-lg"
                            >
                                <div class="flex flex-col items-center gap-1">
                                    <Upload class="text-white" :size="24" />
                                    <span class="text-white text-xs font-medium">Upload</span>
                                </div>
                                <input 
                                    type="file" 
                                    accept="image/*" 
                                    class="hidden" 
                                    @change="handleIconUpload"
                                    :disabled="isCreating"
                                />
                            </label>
                        </div>
                    </div>

                    <div class="flex-1 flex flex-col gap-4">
                        <div class="flex flex-col gap-2">
                            <label class="text-white font-medium text-sm">Instance Name</label>
                            <input
                                v-model="instanceName"
                                type="text"
                                placeholder="My Awesome Modpack"
                                maxlength="50"
                                :disabled="isCreating"
                                class="w-full bg-primary border-2 border-border rounded-lg px-3 py-2 
                                       text-white placeholder-gray-400 focus:border-secondary-accent 
                                       focus:outline-none transition-colors"
                            />
                        </div>

                        <div class="flex flex-col gap-2 relative">
                            <label class="text-white font-medium text-sm">Minecraft Version</label>
                            <button
                                @click="versionDropdownOpen = !versionDropdownOpen"
                                :disabled="isCreating"
                                class="w-full bg-primary border-2 border-border rounded-lg px-3 py-2 
                                       text-white focus:border-secondary-accent focus:outline-none 
                                       transition-colors flex items-center justify-between"
                            >
                                <span>{{ mcVersion }}</span>
                                <ChevronDown 
                                    :size="18" 
                                    class="transition-transform"
                                    :class="{ 'rotate-180': versionDropdownOpen }"
                                />
                            </button>

                            <transition
                                enter-active-class="transition ease-out duration-100"
                                enter-from-class="opacity-0 scale-95"
                                enter-to-class="opacity-100 scale-100"
                                leave-active-class="transition ease-in duration-75"
                                leave-from-class="opacity-100 scale-100"
                                leave-to-class="opacity-0 scale-95"
                            >
                                <div 
                                    v-if="versionDropdownOpen"
                                    class="absolute z-50 top-full mt-2 w-full bg-background border-2 
                                           border-border rounded-lg shadow-xl overflow-hidden"
                                >
                                    <div class="p-2 border-b-2 border-border">
                                        <input
                                            v-model="versionSearch"
                                            type="text"
                                            placeholder="Search versions..."
                                            class="w-full bg-primary border-2 border-border rounded-lg px-3 py-1.5 
                                                   text-white text-sm placeholder-gray-400 focus:border-secondary-accent 
                                                   focus:outline-none transition-colors"
                                        />
                                    </div>

                                    <div class="max-h-64 overflow-y-auto" @scroll="onScroll">
                                        <button
                                            v-for="version in filteredVersions"
                                            :key="version"
                                            @click="selectVersion(version)"
                                            :class="[
                                                'w-full px-3 py-2 text-left text-sm transition-colors',
                                                mcVersion === version 
                                                    ? 'bg-secondary-accent text-white font-medium' 
                                                    : 'text-gray-300 hover:bg-primary/50'
                                            ]"
                                        >
                                            {{ version }}
                                        </button>
                                        
                                        <div 
                                            v-if="hasMoreVersions"
                                            class="px-3 py-2 text-center text-gray-400 text-xs"
                                        >
                                            Scroll for more...
                                        </div>
                                        
                                        <div 
                                            v-if="filteredVersions.length === 0"
                                            class="px-3 py-4 text-center text-gray-400 text-sm"
                                        >
                                            No versions found
                                        </div>
                                    </div>
                                </div>
                            </transition>
                        </div>

                        <div class="flex flex-col gap-2">
                            <label class="text-white font-medium text-sm">Mod Loader</label>
                            <div class="flex gap-2 flex-wrap">
                                <button
                                    v-for="loader in modLoaders"
                                    :key="loader"
                                    @click="modLoader = loader"
                                    :disabled="isCreating"
                                    :class="[
                                        'px-3 py-1.5 rounded-lg border-2 font-medium transition-all duration-300 text-sm',
                                        modLoader === loader
                                            ? 'bg-secondary-accent border-secondary-accent text-white'
                                            : 'bg-background border-border text-gray-400 hover:bg-primary/50 hover:text-white'
                                    ]"
                                >
                                    {{ loader }}
                                </button>
                            </div>
                        </div>

                        <div class="flex flex-col gap-2" v-if="modLoader !== 'Vanilla'">
                            <label class="text-white font-medium text-sm">
                                {{ modLoader }} Version (Optional)
                            </label>
                            <input
                                v-model="modLoaderVersion"
                                type="text"
                                placeholder="Latest"
                                :disabled="isCreating"
                                class="w-full bg-primary border-2 border-border rounded-lg px-3 py-2 
                                       text-white placeholder-gray-400 focus:border-secondary-accent 
                                       focus:outline-none transition-colors"
                            />
                        </div>
                    </div>
                </div>

                <HorizontalSeparator class="my-6" />
                <div class="flex justify-end gap-3">
                    <button
                        @click="!isCreating && (isOpen = false)"
                        :disabled="isCreating"
                        class="px-5 py-2 rounded-lg border-2 border-border text-gray-400 
                               hover:text-white hover:bg-primary/50 transition-colors duration-300 
                               font-medium disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        Cancel
                    </button>

                    <button
                        @click="onClickCreateInstance"
                        :disabled="isCreating || !instanceName"
                        class="px-5 py-2 rounded-lg bg-secondary-accent border-2 border-secondary-accent 
                               hover:bg-secondary-accent/80 text-white transition-colors duration-300 
                               font-medium flex items-center gap-2 disabled:opacity-50 
                               disabled:cursor-not-allowed"
                    >
                        <template v-if="isCreating">
                            <div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                            Creating…
                        </template>
                        <template v-else>
                            <Plus :size="18" :stroke-width="2" />
                            Create Instance
                        </template>
                    </button>
                </div>
            </div>
        </template>
    </Dialog>
</template>