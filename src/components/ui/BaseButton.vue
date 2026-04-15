<script setup lang="ts">
// We define what customizable options this button accepts
defineProps<{
  text?: string;
  variant?: 'primary' | 'danger' | 'success' | 'secondary';
  icon?: boolean; // Just a flag to say if we are putting an icon inside
}>();
</script>

<template>
  <button 
    class="px-4 py-2 rounded-lg font-semibold shadow-md transition-all flex items-center justify-center gap-2 disabled:bg-gray-300 disabled:cursor-not-allowed disabled:text-gray-500 disabled:shadow-none"
    :class="{
      'bg-blue-600 hover:bg-blue-700 text-white': variant === 'primary' || !variant,
      'bg-red-500 hover:bg-red-600 text-white': variant === 'danger',
      'bg-green-500 hover:bg-green-600 text-white': variant === 'success',
      'bg-white hover:bg-gray-50 text-gray-700 border border-gray-200': variant === 'secondary',
    }"
  >
    <slot>
      {{ text }}
    </slot>
  </button>
</template>