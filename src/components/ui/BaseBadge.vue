<script setup lang="ts">
defineProps<{
  text: string;
  variant?: 'success' | 'danger' | 'warning' | 'info' | 'default';
}>();
</script>

<template>
  <span 
    class="px-3 py-1 rounded-full text-xs font-bold whitespace-nowrap"
    :class="{
      'bg-green-100 text-green-800': variant === 'success',
      'bg-red-100 text-red-800': variant === 'danger',
      'bg-orange-100 text-orange-800': variant === 'warning',
      'bg-blue-100 text-blue-800': variant === 'info',
      'bg-gray-100 text-gray-600': variant === 'default' || !variant
    }"
  >
    {{ text }}
  </span>
</template>