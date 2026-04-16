import { createRouter, createWebHistory } from 'vue-router'

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', name: 'dashboard', component: () => import('@/views/DashboardView.vue') },
    { path: '/discovery', name: 'discovery', component: () => import('@/views/DiscoveryView.vue') },
    { path: '/device/:id', name: 'device', component: () => import('@/views/DeviceView.vue') },
    { path: '/gas', name: 'gas', component: () => import('@/views/GasView.vue') },
    { path: '/settings', name: 'settings', component: () => import('@/views/SettingsView.vue') },
    { path: '/about', name: 'about', component: () => import('@/views/AboutView.vue') },
    {
      path: '/tray-customize',
      name: 'tray-customize',
      component: () => import('@/views/TrayCustomizeView.vue'),
    },
    { path: '/:pathMatch(.*)*', redirect: '/' },
  ],
})
