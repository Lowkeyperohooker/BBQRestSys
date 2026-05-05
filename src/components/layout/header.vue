<script setup lang="ts">
import { useResponsive } from '../../composables/useResponsive';

const { fontSm, fontXl } = useResponsive();

withDefaults(defineProps<{
  title: string;
  subtitle?: string;
  isSticky?: boolean;
  isGlass?: boolean;
  customClass?: string;
}>(), {
  isSticky: false,
  isGlass: false,
  customClass: ''
});
</script>

<template>
  <header 
    :class="[
      'flex flex-col md:flex-row justify-between items-start md:items-center gap-3 sm:gap-4 shrink-0',
      isSticky ? 'sticky top-0 z-40' : '',
      isGlass ? 'bg-surface-container/80 md:bg-surface-container/90 backdrop-blur-md' : '',
      customClass
    ]"
  >
    <div class="min-w-0">
      <h3 :class="['font-black text-on-surface tracking-tight truncate', fontXl]">{{ title }}</h3>
      <p v-if="subtitle" :class="['text-on-surface-variant mt-1 font-bold tracking-widest uppercase text-[10px] truncate', fontSm]">
        {{ subtitle }}
      </p>
    </div>
    
    <!-- Render the actions container only if the slot is populated -->
    <div v-if="$slots.actions" class="flex flex-col sm:flex-row items-stretch sm:items-center gap-2 sm:gap-3 w-full md:w-auto shrink-0">
      <slot name="actions"></slot>
    </div>
  </header>
</template>