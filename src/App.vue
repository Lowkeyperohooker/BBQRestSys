<script setup lang="ts">
import { ref, onMounted } from 'vue';
import Sidebar from './components/layout/Sidebar.vue';
import Header from './components/layout/Header.vue';
import LoadingScreen from './components/ui/LoadingScreen.vue';
// import { useAuthStore } from './stores/authStore';

const isAppReady = ref(false);
// const authStore = useAuthStore();

onMounted(() => {
  try {
    // Restore the logged-in user session when the app starts
    // authStore.loadUserFromStorage();
  } catch (error) {
    console.error("Error loading user session:", error);
  } finally {
    // The Rust backend connects instantly, so we can drop the SQLite check
    // and artificial timeout, immediately showing the app layout.
    isAppReady.value = true;
  }
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