<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useAuth } from '../stores/authStore';
import BaseButton from '../components/ui/BaseButton.vue';

const router = useRouter();

// Extract both login and currentUser from the store
const { login, currentUser } = useAuth();

const passcode = ref('');
const errorMsg = ref('');
const isLoading = ref(false);

async function handleLogin() {
  if (!passcode.value) return;
  
  isLoading.value = true;
  errorMsg.value = '';

  try {
    await login(passcode.value);
    
    if (currentUser.value?.role === 'Staff') {
      router.push('/cashier');
    } else {
      router.push('/dashboard'); 
    }
    
  } catch (error: any) {
    errorMsg.value = error.message || "Authentication failed.";
    passcode.value = '';
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <div class="h-full w-full flex items-center justify-center bg-gray-900">
    <div class="bg-white p-8 rounded-2xl shadow-2xl w-full max-w-sm text-center">
      
      <div class="w-16 h-16 bg-blue-600 rounded-xl mx-auto flex items-center justify-center mb-6 shadow-lg">
        <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path></svg>
      </div>

      <h2 class="text-2xl font-black text-gray-900 mb-2">BBQNaMuragLami</h2>
      <p class="text-gray-500 mb-8">Enter your access code to continue</p>

      <form @submit.prevent="handleLogin" class="space-y-6">
        <div>
          <input 
            v-model="passcode" 
            type="password" 
            required
            placeholder="•••••" 
            class="w-full text-center tracking-widest text-2xl border-b-2 border-gray-300 focus:border-blue-600 px-4 py-3 outline-none transition-colors"
          />
          <p v-if="errorMsg" class="text-red-500 text-sm font-medium mt-3">{{ errorMsg }}</p>
        </div>

        <BaseButton type="submit" variant="primary" class="w-full py-4 text-lg font-bold" :disabled="isLoading">
          {{ isLoading ? 'Verifying...' : 'Secure Login' }}
        </BaseButton>
      </form>

    </div>
  </div>
</template>