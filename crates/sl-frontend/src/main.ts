import { createRouter, createWebHistory } from "vue-router";
import { createApp } from "vue";
import App from "@/App.vue";
import "@/App.css";
import "@/lib/logger"
import "@/lib/managers/launcher"

import InstancesPage from "@/pages/InstancesPage.vue";
import StorePage from "@/pages/StorePage.vue";
import InstancePage from "@/pages/InstancePage.vue";
import ProgressPage from "@/pages/ProgressPage.vue";
import HomePage from "./pages/HomePage.vue";
import CosmeticsPage from "./pages/CosmeticsPage.vue";
import ServersPage from "./pages/ServersPage.vue";

const routes = [
  {
    path: "/",
    component: () => import("@/components/layout/Layout.vue"),
    children: [
      { path: "", name: "home", component: HomePage },
      { path: "instances", name: "instances", component: InstancesPage },
      { path: "store", name: "store", component: StorePage },
      {
        path: "instance/:instance_name",
        name: "instance",
        component: InstancePage,
      },
      { path: "cosmetics", name: "cosmetics", component: CosmeticsPage },
      { path: "servers", name: "servers", component: ServersPage }
    ],
  },
  { path: "/progress", name: "progress", component: ProgressPage },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

const app = createApp(App);
app.use(router).mount("#app");
