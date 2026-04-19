<script setup lang="ts">
import { ref, onMounted } from 'vue';
import Database from "@tauri-apps/plugin-sql";
import Sidebar from './components/layout/Sidebar.vue';
import Header from './components/layout/Header.vue';
import LoadingScreen from './components/ui/LoadingScreen.vue';

const isAppReady = ref(false);

async function initializeApp() {
  try {
    await Database.load("sqlite:bbq_system.db");
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

  <div v-show="isAppReady" class="flex h-screen bg-gray-50 font-sans overflow-hidden">
    
    <Sidebar />

    <main class="flex-1 flex flex-col h-screen relative overflow-hidden">
      
      <Header />

      <div class="p-3 md:p-4 flex-1 overflow-hidden">
        <router-view></router-view>
      </div>

    </main>

  </div>
</template>