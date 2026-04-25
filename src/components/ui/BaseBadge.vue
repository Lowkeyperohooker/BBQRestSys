<script setup lang="ts">
defineProps<{
  text: string;
  variant?: 'success' | 'danger' | 'warning' | 'info' | 'default';
}>();
</script>

<template>
  <span 
    class="px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider whitespace-nowrap border"
    :class="{
      'bg-tertiary/10 text-tertiary border-tertiary/20': variant === 'success',
      'bg-error/10 text-error border-error/20': variant === 'danger',
      'bg-tertiary-container/10 text-tertiary-container border-tertiary-container/20': variant === 'warning',
      'bg-secondary/10 text-secondary border-secondary/20': variant === 'info',
      'bg-surface-container-highest/50 text-on-surface-variant border-outline-variant/30': variant === 'default' || !variant
    }"
  >
    {{ text }}
  </span>
</template>