import { createMemoryHistory, createRouter } from 'vue-router'
import { createApp } from "vue";

import App from "./App.vue";
import './App.css';

import HomePage from './pages/HomePage.vue'
import InstancesPage from './pages/InstancesPage.vue';
import StorePage from './pages/StorePage.vue';
import InstancePage from './pages/InstancePage.vue';

const routes = [
  { path: '/', name: "home", component: HomePage },
  { path: '/instances', name: "instances", component: InstancesPage },
  { path: '/store', name: "store", component: StorePage },
  { path: '/instance/:instance_name', name: "instance", component: InstancePage },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

createApp(App).use(router).mount("#app");
