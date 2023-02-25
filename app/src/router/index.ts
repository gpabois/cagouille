import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import AiotsView from '../views/AiotsView.vue'
import VueAiot from '../views/VueAiot.vue'
import VueSuivisInspections from '../views/VueSuivisInspections.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/aiots',
      name: 'aiots',
      component: AiotsView
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
