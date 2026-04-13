import { createRouter, createWebHistory } from 'vue-router'
// Import the Staff page we just made
import Staff from '../views/Staff.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/staff',
      name: 'staff',
      component: Staff
    }
  ]
})

export default router