<script setup lang="ts">
import { useRouter } from 'vue-router';
import { bytesToImageUrl } from '@/lib/utils';
import { instancesManager } from '@/lib/managers/instances';
import InstanceCard from '@/components/instances/InstanceCard.vue';
import CreateInstanceButton from '@/components/instances/CreateInstanceButton.vue';

const router = useRouter()

</script>

<template>
  <main class="h-full w-full p-6 flex flex-col gap-3 overflow-y-auto overflow-x-hidden">
    <div class="grid grid-cols-[repeat(auto-fit,minmax(300px,1fr))] gap-3">
      <InstanceCard v-for="instance in instancesManager.instances" :name="instance.name"
        :mc_version="instance.mc_version" :mod_loader="instance.mod_loader"
        :mod_loader_version="instance.mod_loader_version"
        :icon="instance.icon ? bytesToImageUrl(instance.icon) : '/default_instance_icon.png'"
        @click="router.push(`/instance/${instance.name}`)" />

      <CreateInstanceButton />
    </div>
  </main>
</template>
