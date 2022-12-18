import { createWebHistory, createRouter } from "vue-router";

const routes = [
    {
        path: "/",
        alias: "/online",
        name: "online",
        component: () => import("./components/Online.vue")
    },
    {
        path: "/add",
        name: "add",
        component: () => import("./components/AddDevice.vue")
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;