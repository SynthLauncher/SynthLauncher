<script setup lang="ts">
import { Crown, Shield } from 'lucide-vue-next';
import { ref } from 'vue';

const isPremium = ref(true);
const name = ref("STierProgrammer")
</script>

<template>
    <div class="p-6 border-b-2 border-neutral-700">
        <div class="flex flex-col items-center">
            <div class="relative group mb-4">
                <div
                    class="w-20 h-20 bg-neutral-700 rounded-2xl flex items-center justify-center border-2 border-neutral-600 group-hover:border-blue-400/50 transition-all duration-300">
                    <img :src="`https://mc-heads.net/avatar/${name}/80`" :alt="name" class="w-16 h-16 rounded-xl">
                </div>

                <div
                    class="absolute -bottom-1 -right-1 w-6 h-6 bg-neutral-700 rounded-full flex items-center justify-center border-2 border-neutral-600">
                    <Crown class="w-3 h-3 text-yellow-400" v-if="isPremium" />
                    <Shield class="w-3 h-3 text-neutral-400" v-else />
                </div>
                <div
                    class="absolute inset-0 w-20 h-20 bg-blue-400/10 rounded-2xl blur-md opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                </div>
            </div>

            <div class="text-center space-y-2 w-full">
                <div class="flex items-center justify-center gap-2">
                    <h2 class="text-xl font-bold text-white text-center tracking-wide">{{ name ? name : 'Guest' }}</h2>
                </div>

                <div class="space-y-1">
                    <div class="text-xs text-neutral-400 text-center">ID: {{ }}</div>
                    <div class="flex items-center justify-center gap-2">
                        <div :class="[
                            'inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium',
                            isPremium
                                ? 'bg-yellow-500/20 text-yellow-400 border border-yellow-500/30'
                                : 'bg-neutral-700 text-neutral-400 border border-neutral-600'
                        ]">
                            <span class="inline-flex gap-1 items-center" v-if="isPremium">
                                <Crown class="w-3 h-3 text-yellow-400" />
                                Premium
                            </span>

                            <span class="inline-flex gap-1 items-center" v-else>
                              <Shield class="w-3 h-3 text-neutral-400" />
                              Offline
                            </span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>