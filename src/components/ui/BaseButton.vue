<script setup lang="ts">
defineProps<{
  text?: string;
  variant?: 'primary' | 'danger' | 'success' | 'secondary';
  icon?: boolean; 
}>();
</script>

<template>
  <button 
    class="px-6 py-3 rounded-md font-bold transition-all duration-200 flex items-center justify-center gap-2 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed disabled:active:scale-100 disabled:shadow-none focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-surface"
    :class="{
      'bg-primary-container text-on-primary-container shadow-[0_0_24px_rgba(255,109,0,0.2)] hover:bg-primary-container/90 focus:ring-primary-container': variant === 'primary' || !variant,
      'bg-error text-on-error hover:bg-error/90 focus:ring-error shadow-[0_0_16px_rgba(255,180,171,0.2)]': variant === 'danger',
      'bg-tertiary text-on-tertiary hover:bg-tertiary/90 focus:ring-tertiary shadow-[0_0_16px_rgba(240,190,116,0.2)]': variant === 'success',
      'bg-secondary-container text-on-secondary-container hover:bg-secondary-container/80 focus:ring-outline': variant === 'secondary',
    }"
  >
    <slot>
      {{ text }}
    </slot>
  </button>
</template>