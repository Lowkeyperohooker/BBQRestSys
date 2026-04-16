<script setup lang="ts">
defineProps<{
  text?: string;
  variant?: 'primary' | 'danger' | 'success' | 'secondary';
  icon?: boolean; 
}>();
</script>

<template>
  <button 
    class="px-5 py-2.5 rounded-xl font-bold transition-all flex items-center justify-center gap-2 disabled:bg-gray-200 disabled:cursor-not-allowed disabled:text-gray-400 disabled:shadow-none focus:outline-none focus:ring-2 focus:ring-offset-2"
    :class="{
      'bg-blue-600 hover:bg-blue-700 text-white focus:ring-blue-500': variant === 'primary' || !variant,
      'bg-red-500 hover:bg-red-600 text-white focus:ring-red-500': variant === 'danger',
      'bg-green-500 hover:bg-green-600 text-white focus:ring-green-500': variant === 'success',
      'bg-white hover:bg-gray-50 text-gray-700 border border-gray-200 shadow-sm focus:ring-gray-200': variant === 'secondary',
    }"
  >
    <slot>
      {{ text }}
    </slot>
  </button>
</template>