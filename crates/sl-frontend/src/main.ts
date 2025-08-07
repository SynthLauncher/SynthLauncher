import { createMemoryHistory, createRouter } from 'vue-router'
import HomePage from './pages/HomePage.vue'

import { createApp } from "vue";
import App from "./App.vue";
import './App.css';

const routes = [
  { path: '/', component: HomePage },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

createApp(App).use(router).mount("#app");
