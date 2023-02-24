import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import AiotsView from '../views/AiotsView.vue'
import VueAiot from '../views/VueAiot.vue'

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
    }
  ]
})

export default router
