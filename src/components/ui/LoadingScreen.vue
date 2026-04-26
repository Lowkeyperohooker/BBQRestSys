<script setup lang="ts">
defineProps<{
  isLoading: boolean;
}>();
</script>

<template>
  <transition name="fade">
    <div 
      v-if="isLoading" 
      class="fixed inset-0 z-100 flex flex-col items-center justify-center bg-surface text-on-surface"
    >
      <svg class="w-16 h-16 animate-spin text-primary-container mb-6" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
      
      <h1 class="text-3xl font-black tracking-tighter text-primary mb-2">EMBER & ASH</h1>
      <p class="text-sm text-on-surface-variant font-bold uppercase tracking-widest animate-pulse">Initializing Station...</p>
    </div>
  </transition>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>