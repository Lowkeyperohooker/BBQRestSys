<script setup lang="ts">
import BaseButton from './BaseButton.vue';
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
  <div v-if="isOpen" class="fixed inset-0 z-100 flex items-end justify-end p-4 md:p-8 bg-black/40 backdrop-blur-sm" @click.self="$emit('close')">
    <div class="bg-white w-full max-w-md rounded-3xl shadow-2xl flex flex-col max-h-[85vh] overflow-hidden animate-in slide-in-from-bottom-8 duration-300 origin-bottom-right">
      
      <div class="p-5 border-b border-gray-100 bg-gray-50 flex justify-between items-center shrink-0">
        <h3 :class="['font-extrabold text-gray-900', fontXl]">Review Order</h3>
        <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600 bg-gray-200/50 hover:bg-gray-200 p-2 rounded-full transition-colors">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto p-5">
        <div class="space-y-4">
          <div v-for="(cartItem, index) in cart" :key="index" class="flex justify-between items-center group">
            <div class="flex-1 pr-2">
              <p :class="['font-bold text-gray-900', fontBase]">{{ cartItem.pos_display_name }}</p>
              <p :class="['font-medium text-gray-500 mt-0.5', fontSm]">₱{{ cartItem.unit_price.toFixed(2) }} x {{ cartItem.qty }}</p>
            </div>
            <div class="flex items-center gap-3">
              <span :class="['font-black text-gray-900', fontLg]">₱{{ (cartItem.unit_price * cartItem.qty).toFixed(2) }}</span>
              <button @click="$emit('remove-item', index)" class="text-gray-400 hover:text-red-500 bg-gray-50 hover:bg-red-50 rounded-lg p-2 transition-all">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="p-5 bg-gray-50 border-t border-gray-100 shrink-0">
        <div :class="['flex justify-between mb-2 text-gray-500 font-medium', fontSm]">
          <span>Subtotal</span><span>₱{{ subtotal.toFixed(2) }}</span>
        </div>
        <div :class="['flex justify-between mb-5 font-black text-gray-900', font2xl]">
          <span>Total Due</span><span class="text-blue-600">₱{{ total.toFixed(2) }}</span>
        </div>
        
        <BaseButton variant="primary" @click="$emit('finalize-order')" class="w-full py-4 shadow-lg hover:shadow-xl transition-all rounded-xl">
          <span :class="['font-black uppercase tracking-wider', fontLg]">Finalize Order</span>
        </BaseButton>
      </div>

    </div>
  </div>
</template>