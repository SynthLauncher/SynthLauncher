<script setup lang="ts">
import DiscoverModpacksCard from '@/components/home/DiscoverModpacksCard.vue';
import { LayoutGrid, Library } from 'lucide-vue-next';
</script>

<template>
    <div class="flex flex-col gap-6 p-8 overflow-y-auto">
        <!-- Welcome message -->
        <div class="flex justify-between">
            <div>
                <h1 class="text-white text-2xl font-bold">
                    The real gaming hours, STierProgrammer
                </h1>
                <p class="text-text-muted">
                    Ready to explore the blocky universe?
                </p>
            </div>

            <div class="flex items-center gap-3 py-3 px-6 bg-[#1f1f22] border-2 border-[#39393f] rounded-xl">
                <Library class="text-[#9c9ca5]" />
                <div>
                    <h1 class="text-[#9c9ca5] font-bold">
                        Browse Library
                    </h1>
                    <p class="text-[#7b7b83] text-sm">
                        Explore your instance
                    </p>
                </div>
            </div>
        </div>

        <!-- Quick play -->
        <div class="bg-[#1f1f22] border-2 border-[#39393f] py-6 px-5 rounded-xl flex gap-3">
            <div class="flex items-center gap-2">
                <LayoutGrid class="text-[#9c9ca5]" :size="25" />
                <h1 class="text-[#9c9ca5] text-sm font-medium leading-none">Quick play</h1>
            </div>

            <div class="w-px h-full bg-separator/20 rounded-full self-stretch"></div>

            <div class="bg-[#444449] p-1  rounded-xl">
                <img src="/co_modpack_icon.png" :width="35" :height="35" />
            </div>
            <div class="bg-[#444449] p-1  rounded-xl">
                <img src="/fo_modpack_icon.webp" :width="35" :height="35" />
            </div>
        </div>

        <!-- Discover modpacks -->
        <div class="flex flex-col gap-3">
            <h1 class="text-white text-2xl font-bold">
                Discover modpacks
            </h1>

            <div class="flex items-center w-full gap-6">
                <DiscoverModpacksCard modpack_author="robotkoer"
                    modpack_description="Beautiful graphics, speedy performance and familiar features in a simple package. 1.21.11..."
                    modpack_banner_url="/fo_background.png" modpack_icon_url="/fo_modpack_icon.webp"
                    source_icon_url="/modrinth_icon.svg" modpack_downloads="8.3M"
                    modpack_title="Fabulously Optimized" />
                <DiscoverModpacksCard modpack_author="robotkoer"
                    modpack_description="Beautiful graphics, speedy performance and familiar features in a simple package. 1.21.11..."
                    modpack_banner_url="/fo_background.png" modpack_icon_url="/fo_modpack_icon.webp"
                    source_icon_url="/modrinth_icon.svg" modpack_downloads="8.3M"
                    modpack_title="Fabulously Optimized" />
                <DiscoverModpacksCard modpack_author="robotkoer"
                    modpack_description="Beautiful graphics, speedy performance and familiar features in a simple package. 1.21.11..."
                    modpack_banner_url="/fo_background.png" modpack_icon_url="/fo_modpack_icon.webp"
                    source_icon_url="/modrinth_icon.svg" modpack_downloads="8.3M"
                    modpack_title="Fabulously Optimized" />
            </div>

        </div>

        <!-- Servers -->
        <div>

        </div>
    </div>
</template>
