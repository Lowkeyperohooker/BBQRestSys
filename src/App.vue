<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRoute } from 'vue-router';
import Sidebar from './components/layout/Sidebar.vue';
import LoadingScreen from './components/ui/LoadingScreen.vue';

const isAppReady = ref(false);
const route = useRoute();

// Dynamically check if the route is public. 
// If it is public (Login, Menu), we hide the sidebar.
const isPublicRoute = computed(() => route.meta.public === true);

onMounted(() => {
  try {
    // Session initialization logic can go here later
  } catch (error) {
    console.error("Error loading user session:", error);
  } finally {
    isAppReady.value = true;
  }
});
</script>

<template>
  <LoadingScreen :is-loading="!isAppReady" />

  <div v-show="isAppReady" class="flex h-screen bg-gray-50 font-sans overflow-hidden">
    
    <Sidebar v-if="!isPublicRoute" />

    <main class="flex-1 flex flex-col h-screen relative overflow-hidden">
      
      <div :class="['flex-1 overflow-y-auto', route.path === '/login' ? 'p-0' : 'p-3 md:p-4']">
        <router-view></router-view>
      </div>

    </main>

  </div>
</template>