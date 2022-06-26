import Vue from "vue";
import VueRouter, { RouteConfig } from "vue-router";
import HomeView from "../views/HomeView.vue";
import StatusView from "../views/StatusView.vue";
import StatisticsView from "../views/StatisticsView.vue";
import ExploitsView from "../views/ExploitsView.vue";
import TeamsView from "../views/TeamsView.vue";
import ConfigView from "../views/ConfigView.vue";

Vue.use(VueRouter);

const routes: Array<RouteConfig> = [
  {
    path: "/",
    name: "home",
    component: HomeView,
  },
  {
    path: "/status",
    name: "status",
    component: StatusView,
  },
  {
    path: "/stats",
    name: "statistics",
    component: StatisticsView,
  },
  {
    path: "/exploits",
    name: "exploits",
    component: ExploitsView,
  },
  {
    path: "/teams",
    name: "teams",
    component: TeamsView,
  },
  {
    path: "/config",
    name: "config",
    component: ConfigView,
  },
];

const router = new VueRouter({
  routes,
});

export default router;
