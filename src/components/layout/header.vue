<script setup lang="ts">
import { useResponsive } from '../../composables/useResponsive';

const { fontSm, fontXl } = useResponsive();

defineProps<{
  title: string;
  subtitle?: string;
}>();
</script>

<template>
  <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-4 shrink-0">
    <div>
      <h3 :class="['font-black text-on-surface tracking-tight', fontXl]">{{ title }}</h3>
      <p v-if="subtitle" :class="['text-on-surface-variant mt-1 font-bold tracking-widest uppercase text-[10px]', fontSm]">
        {{ subtitle }}
      </p>
    </div>
    <div class="flex items-center gap-3 w-full md:w-auto">
      <slot name="actions"></slot>
    </div>
  </div>
</template>