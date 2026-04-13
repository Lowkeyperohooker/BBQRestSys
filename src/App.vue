<script setup lang="ts">
// Import the clean layout components we just created
import Sidebar from './components/layout/Sidebar.vue';
import Header from './components/layout/Header.vue';
</script>

<template>
  <div class="flex h-screen bg-gray-100 font-sans overflow-hidden">
    
    <Sidebar />

    <main class="flex-1 flex flex-col h-full relative overflow-y-auto">
      
      <Header />

      <div class="p-8 h-full">
        <router-view></router-view>
      </div>

    </main>

  </div>
</template>