import { createRouter, createWebHistory } from 'vue-router'
import Staff from '../views/Staff.vue'
import Inventory from '../views/Inventory.vue'
import PrepStation from '../views/PrepStation.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/staff',
      name: 'staff',
      component: Staff
    },
    {
      path: '/inventory',
      name: 'inventory',
      component: Inventory
    },
    {
      path: '/prep',
      name: 'prep',
      component: PrepStation
    }
  ]
})

export default router