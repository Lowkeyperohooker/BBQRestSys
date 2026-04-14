<script setup lang="ts">
import { ref, onMounted } from 'vue';
import Database from "@tauri-apps/plugin-sql";

// Import your layout components
import Sidebar from './components/layout/Sidebar.vue';
import Header from './components/layout/Header.vue';
import LoadingScreen from './components/ui/LoadingScreen.vue';

// App readiness state
const isAppReady = ref(false);

async function initializeApp() {
  try {
    // 1. Force the database connection to wake up and run any migrations
    await Database.load("sqlite:bbq_system.db");
    
    // 2. Add an artificial 1.5-second delay for a smooth, professional splash screen
    setTimeout(() => {
      isAppReady.value = true;
    }, 1500);

  } catch (error) {
    console.error("Critical Error: Failed to initialize app", error);
    alert("Database failed to load. Please restart the application.");
  }
}

onMounted(() => {
  initializeApp();
});
</script>

<template>
  <LoadingScreen :is-loading="!isAppReady" />

  <div v-show="isAppReady" class="flex h-screen bg-gray-100 font-sans overflow-hidden">
    
    <Sidebar />

    <main class="flex-1 flex flex-col h-full relative overflow-y-auto">
      
      <Header />

      <div class="p-8 h-full">
        <router-view></router-view>
      </div>

    </main>

  </div>
</template>