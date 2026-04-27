<script setup lang="ts">
import BaseButton from '../ui/BaseButton.vue';
import { useResponsive } from '../../composables/useResponsive';
import type { CartItem } from '../../services/posService';

const { fontSm, fontBase, fontLg, fontXl, font2xl } = useResponsive();

defineProps<{
  isOpen: boolean;
  cart: CartItem[];
  subtotal: number;
  total: number;
}>();

defineEmits<{
  (e: 'close'): void;
  (e: 'remove-item', index: number): void;
  (e: 'finalize-order'): void;
}>();
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-100 flex items-end sm:items-center justify-end sm:justify-center p-4 md:p-8 bg-black/60 backdrop-blur-md" @click.self="$emit('close')">
    <div class="bg-surface-container-low border border-outline-variant/20 w-full max-w-md rounded-2xl shadow-2xl flex flex-col max-h-[90vh] md:max-h-[85vh] overflow-hidden animate-in slide-in-from-bottom-8 duration-300 origin-bottom-right sm:origin-center">
      
      <div class="p-4 sm:p-5 border-b border-outline-variant/10 bg-surface-container-highest/30 flex justify-between items-center shrink-0">
        <h3 :class="['font-extrabold text-on-surface tracking-tight', fontXl]">Review Order</h3>
        <button @click="$emit('close')" class="text-on-surface-variant hover:text-on-surface bg-surface-container hover:bg-surface-container-high p-2 rounded-full transition-colors active:scale-90">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto p-4 sm:p-5">
        <div class="space-y-3 sm:space-y-4">
          <div v-for="(cartItem, index) in cart" :key="index" class="flex justify-between items-center group bg-surface-container py-3 px-3 sm:px-4 rounded-xl border border-outline-variant/5">
            <div class="flex-1 pr-2 min-w-0">
              <p :class="['font-bold text-on-surface truncate', fontBase]">{{ cartItem.pos_display_name }}</p>
              <p :class="['font-medium text-on-surface-variant mt-0.5', fontSm]">₱{{ cartItem.unit_price.toFixed(2) }} x {{ cartItem.qty }}</p>
            </div>
            <div class="flex items-center gap-3 sm:gap-4 shrink-0">
              <span :class="['font-black text-on-surface', fontLg]">₱{{ (cartItem.unit_price * cartItem.qty).toFixed(2) }}</span>
              <button @click="$emit('remove-item', index)" class="text-on-surface-variant hover:text-error bg-surface-container-high hover:bg-error/10 rounded-lg p-2 transition-all active:scale-90">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="p-5 sm:p-6 bg-surface-container-highest/20 border-t border-outline-variant/10 shrink-0">
        <div :class="['flex justify-between mb-2 sm:mb-3 text-on-surface-variant font-medium', fontSm]">
          <span>Subtotal</span><span>₱{{ subtotal.toFixed(2) }}</span>
        </div>
        <div :class="['flex justify-between mb-5 sm:mb-6 font-black text-on-surface', font2xl]">
          <span>Total Due</span><span class="text-primary-container">₱{{ total.toFixed(2) }}</span>
        </div>
        
        <BaseButton variant="primary" @click="$emit('finalize-order')" class="w-full py-4 text-lg">
          <span class="font-black uppercase tracking-widest">Finalize Order</span>
        </BaseButton>
      </div>

    </div>
  </div>
</template>