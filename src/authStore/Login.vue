<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useAuth } from '../stores/authStore';
import BaseButton from '../components/ui/BaseButton.vue';

const router = useRouter();

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
  <div class="h-full w-full flex items-center justify-center bg-surface">
    <div class="bg-surface-container-low border border-outline-variant/15 p-8 rounded-2xl shadow-2xl w-full max-w-sm text-center">
      
      <div class="w-16 h-16 bg-primary-container/20 border border-primary-container/30 rounded-xl mx-auto flex items-center justify-center mb-6 shadow-lg shadow-primary-container/10">
        <img src="../assets/vue.svg" class="w-16 h-16" alt="lock icon">
      </div>

      <h2 class="text-2xl font-black text-on-surface tracking-tight mb-2">BBQ na murag lami</h2>
      <p class="text-on-surface-variant font-medium text-sm mb-8 tracking-wide">Enter your access code to continue</p>

      <form @submit.prevent="handleLogin" class="space-y-6">
        <div>
          <input 
            v-model="passcode" 
            type="password" 
            required
            placeholder="•••••" 
            class="w-full text-center tracking-[0.5em] text-2xl bg-surface-container border-b-2 border-outline-variant/30 text-on-surface focus:border-primary-container px-4 py-3 outline-none transition-colors rounded-t-lg"
          />
          <p v-if="errorMsg" class="text-error text-sm font-bold mt-3 uppercase tracking-wider">{{ errorMsg }}</p>
        </div>

        <BaseButton type="submit" variant="primary" class="w-full py-4 text-lg" :disabled="isLoading">
          <span class="font-black uppercase tracking-widest">{{ isLoading ? 'Verifying...' : 'Secure Login' }}</span>
        </BaseButton>
      </form>

    </div>
  </div>
</template>