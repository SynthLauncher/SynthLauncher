<script setup lang="ts">
import { ref } from 'vue';
import { bytesToImageUrl } from '../lib/utils';
import { instancesManager } from '../lib/managers/instances';
import Dialog from '../components/ui/Dialog.vue';
import InstanceCard from '../components/instances/InstanceCard.vue';
import CreateInstanceButton from '../components/instances/CreateInstanceButton.vue';
import { useRouter } from 'vue-router';

const router = useRouter()

const isOpen = ref(false)
const showContent = ref(false)
</script>

<template>
  <main class="h-full w-full p-6 flex flex-col gap-3 overflow-y-auto overflow-x-hidden">
    <div class="grid grid-cols-4 gap-3">
      <InstanceCard
        v-for="instance in instancesManager.instances" 
        :name="instance.name" 
        :mc_version="instance.mc_version"
        :mod_loader="instance.mod_loader" 
        :mod_loader_version="instance.mod_loader_version"
        :icon="instance.icon ? bytesToImageUrl(instance.icon) : ''"
        @click="router.push(`/instance/${instance.name}`)"
      />

      <Dialog v-model:isOpen="isOpen" v-model:showContent="showContent">
        <template #trigger>
          <CreateInstanceButton @click="" />
        </template>

        <template #content>

        </template>
      </Dialog>
    </div>
</main>
</template>
