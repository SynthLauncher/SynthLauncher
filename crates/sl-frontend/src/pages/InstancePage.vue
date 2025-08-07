<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()
const instanceName = route.params.instance_name as string
</script>

<template>
  <div>
    <h1 class="text-green-500">Instance: {{ instanceName }}</h1>
  </div>
</template>
