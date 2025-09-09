<template>
  <main class="h-full w-full p-6 flex flex-col gap-3 overflow-y-auto overflow-x-hidden">
    <!--  
      TODO: Make this work
    -->
    <OptionSelector :selected-value='"All Instances"' :values='["All Instances", "Downloaded", "Custom"]' />
    <InstancesSearhBar v-model:search-query="searchQuery" />

    <!--  
      FIXME: Make this responsive
    -->
    <div class="grid grid-cols-4 gap-3">
      <InstanceCard 
        v-for="instance in instances" 
        :name="instance.name" 
        :mc_version="instance.mc_version"
        :mod_loader="instance.mod_loader" 
        :mod_loader_version="instance.mod_loader_version"
        :icon="instance.icon ? bytesToImageUrl(instance.icon) : ''"

      />

      <CreateInstanceButton />
    </div>
</main>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getAllInstances } from '../lib/commands/instances';
import { InstanceMetadata } from '../types/instances';
import OptionSelector from '../components/OptionSelector.vue';
import InstancesSearhBar from '../components/InstancesSearhBar.vue';
import InstanceCard from '../components/InstanceCard.vue';
import { bytesToImageUrl } from '../lib/utils';
// import { useRouter } from 'vue-router';
import CreateInstanceButton from '../components/CreateInstanceButton.vue';

// const router = useRouter()

const instances = ref<InstanceMetadata[]>([]);
const searchQuery = ref("");

const loadInstances = async () => {
  const result = await getAllInstances();
  instances.value = result as InstanceMetadata[];
}

onMounted(async () => {
  await loadInstances();
});

// @click="() => router.push(`instance/${instance.name}`)" 
</script>
