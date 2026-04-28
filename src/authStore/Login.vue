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
    <div class="bg-surface-container-low border border-outline-variant/10 p-8 md:p-10 rounded-4xl shadow-[0_20px_60px_rgba(0,0,0,0.3)] w-full max-w-sm text-center animate-in fade-in zoom-in-95 duration-500 ease-out">
      
      <div class="w-28 h-28 mx-auto flex items-center justify-center mb-6 animate-in slide-in-from-bottom-4 duration-700">
        <img src="../assets/bbq-icon.png" class="w-full h-full object-contain drop-shadow-[0_0_20px_rgba(255,109,0,0.3)]" alt="bbq icon">
      </div>

      <p class="text-on-surface-variant font-medium text-sm mb-8 tracking-wide">Enter your access code to continue</p>

      <form @submit.prevent="handleLogin" class="space-y-6">
        <div>
          <input 
            v-model="passcode" 
            type="password" 
            required
            placeholder="•••••" 
            class="w-full text-center tracking-[0.3em] text-xl bg-surface-container border-2 border-outline-variant/20 text-on-surface focus:border-primary-container focus:ring-4 focus:ring-primary-container/20 px-4 py-3 outline-none transition-all duration-300 rounded-xl shadow-inner placeholder-on-surface-variant/30"
          />
          <p v-if="errorMsg" class="text-error text-xs font-bold mt-3 uppercase tracking-widest animate-in slide-in-from-top-1">{{ errorMsg }}</p>
        </div>

        <BaseButton type="submit" variant="primary" class="w-full py-3 text-sm shadow-[0_8px_20px_rgba(255,109,0,0.25)] hover:shadow-[0_12px_28px_rgba(255,109,0,0.4)] transition-all duration-300 active:scale-95" :disabled="isLoading">
          <span class="font-black uppercase tracking-widest">{{ isLoading ? 'Verifying...' : 'Secure Login' }}</span>
        </BaseButton>
      </form>

    </div>
  </div>
</template>