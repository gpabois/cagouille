import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import VueAiots from '@/views/VueAiots.vue'
import VueAiot from '@/views/VueAiot.vue'
import VueSuivisInspections from '@/views/VueSuivisInspections.vue'
import VueTaches from '@/views/VueTaches.vue'
import VueMesTaches from '@/views/VueMesTaches.vue'
import VueTache from '@/views/VueTache.vue'
import VueRvats from '@/views/VueRvats.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/taches',
      name: 'taches',
      component: VueTaches
    },
    {
      path: '/moi/taches',
      name: 'mes_taches',
      component: VueMesTaches
    },
    {
      path: '/taches/:id',
      name: 'tache',
      component: VueTache
    },
    {
      path: '/rvats',
      name: 'rvats',
      component: VueRvats
    },
    {
      path: '/aiots',
      name: 'aiots',
      component: VueAiots
    }, 
    {
      path: '/aiot/:id',
      name: 'detail_aiot',
      component: VueAiot
    },
    {
      path: '/suivis/inspections',
      name: "suivis_inspections",
      component: VueSuivisInspections
    }
  ]
})

export default router
