<script setup lang="ts">
import { useResponsive } from '../../composables/useResponsive';
import type { PosItem } from '../../services/posService';

const { fontSm, fontLg, fontXl } = useResponsive();

defineProps<{
  isOpen: boolean;
  group: any | null;
}>();

defineEmits<{
  (e: 'close'): void;
  (e: 'select-variant', variant: PosItem): void;
}>();
</script>

<template>
  <div v-if="isOpen && group" class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
    <div class="bg-surface rounded-3xl w-full max-w-md overflow-hidden shadow-2xl flex flex-col border border-outline-variant/20 animate-in fade-in zoom-in-95 duration-200">
      
      <div class="px-6 py-5 border-b border-outline-variant/20 flex justify-between items-center bg-surface-container-low">
        <h3 :class="['font-black text-on-surface', fontXl]">Select {{ group.display_name }}</h3>
        <button @click="$emit('close')" class="text-on-surface-variant hover:text-error transition-colors active:scale-90">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="p-4 md:p-6 flex-1 overflow-y-auto bg-surface space-y-3">
        <button v-for="variant in group.variants" :key="variant.prep_item_id"
          @click="$emit('select-variant', variant)"
          class="w-full text-left bg-surface-container-low hover:bg-surface-container-high border-2 border-outline-variant/20 hover:border-primary rounded-2xl p-4 flex justify-between items-center transition-all group"
          :disabled="variant.current_stock_pieces <= 0"
          :class="{'opacity-50 cursor-not-allowed': variant.current_stock_pieces <= 0}">
          
          <div>
            <h4 :class="['font-bold text-on-surface group-hover:text-primary transition-colors', fontLg]">
              {{ variant.variant_name || variant.pos_display_name }}
            </h4>
            <p v-if="variant.current_stock_pieces > 0" :class="['text-on-surface-variant font-medium uppercase tracking-wider', fontSm]">Available</p>
            <p v-else :class="['text-error font-bold uppercase tracking-wider', fontSm]">Out of stock</p>
          </div>
          
          <span :class="['font-black text-primary-container', fontLg]">₱{{ variant.unit_price.toFixed(2) }}</span>
        </button>
      </div>

    </div>
  </div>
</template>