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
  <div class="bg-white border-b md:border-b-0 md:border-r border-gray-200 w-full md:w-56 lg:w-64 shrink-0 overflow-x-auto md:overflow-y-auto shadow-sm md:shadow-none z-10">
    <div class="px-5 py-5 hidden md:block border-b border-gray-100">
      <h3 class="font-black text-gray-800 text-lg uppercase tracking-wider">Categories</h3>
    </div>
    
    <div class="flex md:flex-col gap-2 p-2 md:p-4">
      <button 
        v-for="cat in categories" 
        :key="cat" 
        @click="$emit('select', cat)" 
        :class="[
          selectedCategory === cat 
            ? 'bg-blue-50 border-blue-600 text-blue-700 shadow-sm' 
            : 'bg-transparent border-transparent text-gray-600 hover:bg-gray-50', 
          'px-4 py-3 md:py-4 border-b-2 md:border-b-0 md:border-l-4 font-bold text-left transition-all whitespace-nowrap rounded-lg md:rounded-r-lg md:rounded-l-none'
        ]"
      >
        <span class="text-base truncate">{{ cat }}</span>
      </button>
    </div>
  </div>
</template>