<script setup lang="ts">
import { ref } from 'vue';

// A simple way to track which page we are on for the sidebar styling
const currentPath = ref(window.location.pathname);
</script>

<template>
  <div class="flex h-screen bg-gray-100 font-sans overflow-hidden">
    
    <aside class="w-64 bg-gray-900 text-white flex flex-col z-20 shadow-xl">
      <div class="p-6 border-b border-gray-800">
        <h1 class="text-2xl font-bold uppercase tracking-wider text-amber-500">BBQ System</h1>
        <p class="text-sm text-gray-400 mt-1">Admin Portal</p>
      </div>
      
      <nav class="flex-1 px-4 py-6 space-y-2 overflow-y-auto">
        <router-link to="/" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
          Dashboard
        </router-link>
        
        <router-link to="/pos" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
          Point of Sale
        </router-link>

        <router-link to="/inventory" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
          Inventory & Prep
        </router-link>
        
        <router-link to="/staff" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
          Staff Directory
        </router-link>
      </nav>
      
      <div class="p-4 border-t border-gray-800 text-sm text-gray-500 text-center">
        <p>System Status: <span class="text-green-400 font-semibold">Online</span></p>
      </div>
    </aside>

    <main class="flex-1 flex flex-col h-full relative overflow-y-auto">
      
      <header class="bg-white shadow-sm border-b px-8 py-4 flex justify-end items-center z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <div class="h-8 w-8 rounded-full bg-amber-500 flex items-center justify-center text-white font-bold shadow">
            A
          </div>
        </div>
      </header>

      <div class="p-8 h-full">
        <router-view></router-view>
      </div>

    </main>

  </div>
</template>