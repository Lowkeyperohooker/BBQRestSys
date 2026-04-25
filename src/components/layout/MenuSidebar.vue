<script setup lang="ts">
defineProps<{
  categories: string[];
  selectedCategory: string;
}>();

defineEmits<{
  (e: 'select', category: string): void;
}>();
</script>

<template>
  <div class="bg-surface-container-low border-b md:border-b-0 md:border-r border-outline-variant/15 w-full md:w-56 lg:w-64 shrink-0 overflow-x-auto md:overflow-y-auto shadow-sm md:shadow-none z-10">
    <div class="px-5 py-5 hidden md:block border-b border-outline-variant/10">
      <h3 class="font-black text-on-surface-variant text-sm uppercase tracking-widest">Categories</h3>
    </div>
    
    <div class="flex md:flex-col gap-2 p-2 md:p-4">
      <button 
        v-for="cat in categories" 
        :key="cat" 
        @click="$emit('select', cat)" 
        :class="[
          selectedCategory === cat 
            ? 'bg-primary-container/10 border-primary-container text-primary shadow-sm' 
            : 'bg-transparent border-transparent text-on-surface-variant hover:bg-surface-container hover:text-on-surface', 
          'px-4 py-3 md:py-4 border-b-2 md:border-b-0 md:border-l-4 font-bold text-left transition-all whitespace-nowrap rounded-lg md:rounded-r-lg md:rounded-l-none'
        ]"
      >
        <span class="text-sm tracking-wide uppercase font-bold truncate">{{ cat }}</span>
      </button>
    </div>
  </div>
</template>