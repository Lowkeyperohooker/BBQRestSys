<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';

// Vue Router gives us access to the current page information
const route = useRoute();

// This dynamically changes the title based on where the user clicks in the sidebar!
const pageTitle = computed(() => {
  switch (route.path) {
    case '/': return 'Performance Dashboard';
    case '/pos': return 'Point of Sale Terminal';
    case '/inventory': return 'Inventory & Prep Station';
    case '/staff': return 'Staff Directory & Management';
    default: return 'BBQ Management System';
  }
});
</script>

<template>
  <header class="bg-white shadow-sm border-b px-8 py-4 flex justify-between items-center z-10 sticky top-0">
    
    <h2 class="text-xl font-semibold text-gray-800">
      {{ pageTitle }}
    </h2>
    
    <div class="flex items-center gap-5">
      <button class="text-gray-400 hover:text-gray-600 relative transition-colors">
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path></svg>
        <span class="absolute -top-1 -right-1 bg-red-500 text-white text-[10px] font-bold rounded-full h-4 w-4 flex items-center justify-center border-2 border-white">4</span>
      </button>
      
      <div class="h-9 w-9 rounded-full bg-blue-600 flex items-center justify-center text-white font-bold shadow-sm ring-2 ring-white cursor-pointer hover:bg-blue-700 transition-colors">
        A
      </div>
    </div>
    
  </header>
</template>