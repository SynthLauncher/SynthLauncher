<template>
  <main>
    <div class="flex flex-col">
      <router-link :to="{ name: 'home' }">Home</router-link>
      <router-link :to="{ name: 'instance', params: { instance_name: 'my-instance' }}">my-instance</router-link>
    </div>
    <router-view />
  </main>
</template>
