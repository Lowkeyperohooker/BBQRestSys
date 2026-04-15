<script setup lang="ts">
defineProps<{
  title?: string;
  subtitle?: string;
  noPadding?: boolean;
}>();
</script>

<template>
  <div class="bg-white rounded-xl shadow-sm border border-gray-100 flex flex-col" :class="{ 'p-6': !noPadding }">
    
    <div v-if="title || $slots.action" class="flex justify-between items-start mb-6" :class="{ 'p-6 pb-0': noPadding }">
      <div>
        <h3 v-if="title" class="text-lg font-bold text-gray-800">{{ title }}</h3>
        <p v-if="subtitle" class="text-sm text-gray-500 mt-1">{{ subtitle }}</p>
      </div>
      <div v-if="$slots.action">
        <slot name="action"></slot>
      </div>
    </div>

    <slot></slot>
    
  </div>
</template>