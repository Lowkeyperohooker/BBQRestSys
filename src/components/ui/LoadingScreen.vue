<script setup lang="ts">
defineProps<{
  isLoading: boolean;
}>();
</script>

<template>
  <transition name="fade">
    <div 
      v-if="isLoading" 
      class="fixed inset-0 z-100 flex flex-col items-center justify-center bg-gray-900 text-white"
    >
      <svg class="w-16 h-16 animate-spin text-blue-500 mb-6" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
      
      <h1 class="text-3xl font-bold tracking-wider mb-2">BBQ SYSTEM</h1>
      <p class="text-sm text-gray-400 font-medium animate-pulse">Initializing Database Environment...</p>
    </div>
  </transition>
</template>

<style scoped>
/* Smooth fade transition when the loading finishes */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>