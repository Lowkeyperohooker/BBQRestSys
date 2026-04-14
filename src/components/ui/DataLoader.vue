<script setup lang="ts">
// We define a 'message' prop so each page can pass its own custom text
defineProps({
  message: {
    type: String,
    default: 'Loading data...'
  }
});
</script>

<template>
  <div class="flex flex-col items-center justify-center py-16 w-full h-full">
    <svg class="w-10 h-10 animate-spin text-blue-500 mb-4" fill="none" viewBox="0 0 24 24">
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
    </svg>
    <p class="text-gray-500 font-medium animate-pulse">{{ message }}</p>
  </div>
</template>