import { createRouter, createWebHistory } from "vue-router";
import { createApp } from "vue";
import App from "./App.vue";
import "./App.css";

import InstancesPage from "./pages/InstancesPage.vue";
import StorePage from "./pages/StorePage.vue";
import InstancePage from "./pages/InstancePage.vue";
import ProgressPage from "./pages/ProgressPage.vue";

const routes = [
  { path: "/", name: "instances", component: InstancesPage },
  { path: "/store", name: "store", component: StorePage },
  {
    path: "/instance/:instance_name",
    name: "instance",
    component: InstancePage,
  },
  { path: "/progress", name: "progress", component: ProgressPage },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

const app = createApp(App);
app.use(router).mount("#app");
