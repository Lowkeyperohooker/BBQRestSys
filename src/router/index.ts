import { createRouter, createWebHistory } from 'vue-router';
import { useAuth } from '../stores/authStore';

import Dashboard from '../views/Dashboard.vue';
import Pos from '../views/Pos.vue';
import Inventory from '../views/Inventory.vue';
import Staff from '../views/Staff.vue';
import Logs from '../views/Logs.vue';
import Schedule from '../views/Schedule.vue';
import PrepStation from '../views/PrepStation.vue';

// Note: Super Admin automatically has access to everything, 
// so we don't need to explicitly list 'Super Admin' in every array.
const routes = [
  { 
    path: '/', 
    component: Dashboard, 
    meta: { roles: ['Admin'] } // Admin only
  },
  { 
    path: '/pos', 
    component: Pos, 
    meta: { roles: ['Staff'] } // Staff (Cashier) only
  },
  { 
    path: '/inventory', 
    component: Inventory, 
    meta: { roles: ['Admin'] } // Admin only
  },
  { 
    path: '/prep', 
    component: PrepStation, 
    meta: { roles: ['Staff'] } // Staff only
  },
  { 
    path: '/schedule', 
    component: Schedule, 
    meta: { roles: ['Admin', 'Staff'] } // Both need to clock in
  },
  { 
    path: '/staff', 
    component: Staff, 
    meta: { roles: ['Admin'] } // Admin only
  },
  { 
    path: '/logs', 
    component: Logs, 
    meta: { roles: ['Admin'] } // Admin only
  },
  // Future Public Customer Route
  // {
  //   path: '/menu',
  //   component: CustomerMenu,
  //   meta: { public: true } // No auth required
  // }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// The Gatekeeper: Runs before every page change
router.beforeEach((to, _from, next) => {
  const { isAuthenticated, hasAccess } = useAuth();

  // 1. If the route is public, let them in immediately
  if (to.meta.public) {
    return next();
  }

  // 2. If they are not logged in, boot them to a login page (placeholder for now)
  if (!isAuthenticated.value) {
    alert("You must be logged in.");
    return next(false); // For now, just cancel navigation
  }

  // 3. Check if their role is in the allowed list for this specific route
  const requiredRoles = to.meta.roles as Array<'Admin' | 'Staff'>;
  
  if (requiredRoles && !hasAccess(requiredRoles)) {
    alert("Access Denied: You do not have permission to view this page.");
    return next(false); // Block navigation
  }

  // 4. If all checks pass, let them through
  next();
});

export default router;