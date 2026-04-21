import { ref, computed } from 'vue';
import { authService, type LoginResponse } from '../services/authService';

export type Role = 'Super Admin' | 'Admin' | 'Staff' | 'Customer' | null;

export interface User {
  id: number;
  name: string;
  role: Role;
}

// Default to null. The user is locked out until they log in.
const currentUser = ref<User | null>(null);

export const useAuth = () => {
  const isAuthenticated = computed(() => currentUser.value !== null);
  const userRole = computed(() => currentUser.value?.role);

  function hasAccess(allowedRoles: Role[]): boolean {
    if (!currentUser.value) return false;
    if (currentUser.value.role === 'Super Admin') return true;
    return allowedRoles.includes(currentUser.value.role);
  }

  // The new Database Login Action
  async function login(passcode: string): Promise<boolean> {
    try {
      const dbUser: LoginResponse = await authService.login(passcode);
      
      currentUser.value = {
        id: dbUser.staff_id,
        name: dbUser.full_name,
        role: dbUser.role
      };
      
      return true; // Login successful
    } catch (error) {
      console.error("Login failed:", error);
      throw error; // Pass error to UI for display
    }
  }

  function logout() {
    currentUser.value = null;
  }

  return { 
    currentUser, 
    isAuthenticated, 
    userRole, 
    hasAccess,
    login,
    logout 
  };
};