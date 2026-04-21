<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { useRoute } from 'vue-router';
import { posService } from '../../services/posService';
import { useResponsive } from '../../composables/useResponsive';

const { isMobile, fontBase, fontXl } = useResponsive();

const activeTabCount = ref(0);
let pollInterval: any = null;
const isMobileMenuOpen = ref(false);
const route = useRoute();

watch(route, () => {
  isMobileMenuOpen.value = false;
});

async function checkActiveTabs() {
  try {
    const orders = await posService.getActiveOrders();
    activeTabCount.value = orders.length;
  } catch (error) {
    console.error("Failed to check active tabs:", error);
  }
}

onMounted(() => {
  checkActiveTabs();
  pollInterval = setInterval(checkActiveTabs, 3000);
});

onUnmounted(() => {
  if (pollInterval) clearInterval(pollInterval);
});
</script>

<template>
  <div 
    v-if="isMobileMenuOpen" 
    @click="isMobileMenuOpen = false" 
    class="fixed inset-0 bg-black/60 z-40 md:hidden backdrop-blur-sm transition-opacity"
  ></div>

  <button 
    v-if="isMobile"
    @click="isMobileMenuOpen = !isMobileMenuOpen" 
    class="fixed bottom-6 right-6 z-50 bg-blue-600 text-white p-4 rounded-full shadow-2xl hover:bg-blue-700 transition-transform active:scale-95"
  >
    <svg v-if="!isMobileMenuOpen" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
    <svg v-else class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
  </button>

  <aside 
    :class="[
      'w-64 bg-gray-900 text-white flex flex-col z-50 shadow-xl h-full shrink-0',
      'fixed md:relative inset-y-0 left-0 transition-transform duration-300 ease-in-out',
      isMobileMenuOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0'
    ]"
  >
    
    <div class="flex items-center gap-3 px-6 py-6 border-b border-gray-800">
      <div class="w-10 h-10 min-w-10 bg-gradient-to-br from-blue-600 to-blue-800 rounded-xl shadow-md flex items-center justify-center text-white">
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 14v6m-3-3h6M6 10h2a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v2a2 2 0 002 2zm10 0h2a2 2 0 002-2V6a2 2 0 00-2-2h-2a2 2 0 00-2 2v2a2 2 0 002 2zM6 20h2a2 2 0 002-2v-2a2 2 0 00-2-2H6a2 2 0 00-2 2v2a2 2 0 002 2z"></path></svg>
      </div>
      <div class="flex flex-col">
        <span :class="['font-black tracking-tight leading-none text-white', fontXl]">BBQ<span class="text-blue-500">SYS</span></span>
        <span class="text-gray-400 text-[10px] font-bold uppercase tracking-widest mt-1">Admin Portal</span>
      </div>
    </div>
    
    <nav :class="['flex-1 px-4 py-6 space-y-1 overflow-y-auto pb-24 md:pb-6', fontBase]">
      
      <router-link to="/" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z"></path></svg>
        Dashboard
      </router-link>
      
      <router-link to="/pos" class="flex items-center justify-between px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <div class="flex items-center gap-3">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"></path></svg>
          Point of Sale
        </div>
        <span v-if="activeTabCount > 0" class="bg-orange-500 text-white text-[10px] font-bold px-2 py-0.5 rounded-full shadow-sm">
          {{ activeTabCount }}
        </span>
      </router-link>

      <router-link to="/inventory" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0M6 7l3 9M6 7l6-2m6 2l3-1m-3 1l-3 9a5.002 5.002 0 006.001 0M18 7l3 9m-3-9l-6-2m0-2v2m0 16V5m0 16H9m3 0h3"></path></svg>
        Inventory
      </router-link>

      <router-link to="/prep" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.121 14.121L19 19m-7-7l7-7m-7 7l-2.879 2.879M12 12L9.121 9.121m0 5.758a3 3 0 10-4.243-4.243 3 3 0 004.243 4.243zm8.486 8.486a3 3 0 10-4.243-4.243 3 3 0 004.243 4.243z"></path></svg>
        Prep Station
      </router-link>

      <router-link to="/schedule" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
        Staff Scheduling
      </router-link>
      
      <router-link to="/staff" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>
        Staff Directory
      </router-link>

      <router-link to="/logs" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
        System Logs
      </router-link>

    </nav>
    
  </aside>
</template>