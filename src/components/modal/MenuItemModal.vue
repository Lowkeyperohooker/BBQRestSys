<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import BaseButton from '../ui/BaseButton.vue';
import { useResponsive } from '../../composables/useResponsive';
import type { PosItem } from '../../services/posService';

const { fontSm, font2xl, fontXl } = useResponsive();

const props = defineProps<{
  isOpen: boolean;
  item: PosItem | null;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'confirm', payload: { qty: number; customPrice: number }): void;
}>();

const qty = ref(1);

watch(() => props.isOpen, (newVal) => {
  if (newVal && props.item) qty.value = 1;
});

const totalValue = computed(() => {
  if (!props.item) return 0;
  return qty.value * props.item.unit_price;
});

function increment() {
  if (props.item && qty.value < props.item.current_stock_pieces) qty.value++;
}

function decrement() {
  if (qty.value > 1) qty.value--;
}

function handleConfirm() {
  if (!props.item) return;
  emit('confirm', { qty: qty.value, customPrice: props.item.unit_price });
}
</script>

<template>
  <div v-if="isOpen && item" class="fixed inset-0 z-110 flex items-center justify-center p-4 bg-black/70 backdrop-blur-md transition-opacity duration-300" @click.self="$emit('close')">
    <div class="bg-surface rounded-[2rem] w-full max-w-sm shadow-[0_20px_60px_rgba(0,0,0,0.4)] flex flex-col border border-outline-variant/10 overflow-hidden animate-in fade-in zoom-in-95 duration-300 ease-out">
      
      <div class="p-6 flex justify-between items-start bg-surface-container-lowest">
        <div>
          <h3 :class="['font-black text-on-surface leading-tight tracking-tight', fontXl]">{{ item.pos_display_name }}</h3>
          <div class="mt-2 flex items-center gap-2">
            <span v-if="item.current_stock_pieces > 0" class="inline-block w-2 h-2 rounded-full bg-success shadow-[0_0_8px_rgba(var(--color-success),0.6)]"></span>
            <span v-else class="inline-block w-2 h-2 rounded-full bg-error shadow-[0_0_8px_rgba(var(--color-error),0.6)]"></span>
            <p :class="['text-on-surface-variant font-bold uppercase tracking-widest', fontSm]">
              {{ (item.current_stock_pieces > 0) ? "Available" : "Out of stock" }}
            </p>
          </div>
        </div>
        <button @click="$emit('close')" class="text-on-surface-variant hover:text-on-surface bg-surface-container hover:bg-surface-container-high p-2.5 rounded-full transition-all ml-4 active:scale-90">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="p-8 flex flex-col items-center justify-center bg-surface border-y border-outline-variant/5">
        <label :class="['font-black text-on-surface-variant mb-4 uppercase tracking-widest', fontSm]">Quantity</label>
        <div class="flex items-center gap-6 bg-surface-container-low p-2 rounded-3xl border border-outline-variant/10 shadow-inner">
          <button @click="decrement" :disabled="qty <= 1" class="w-14 h-14 rounded-2xl bg-surface-container flex items-center justify-center text-on-surface hover:bg-surface-container-high hover:shadow-md disabled:opacity-30 disabled:shadow-none transition-all duration-200 active:scale-90 disabled:active:scale-100">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M20 12H4"></path></svg>
          </button>
          <span :class="['w-16 text-center font-black text-on-surface', font2xl]">{{ qty }}</span>
          <button @click="increment" :disabled="qty >= item.current_stock_pieces" class="w-14 h-14 rounded-2xl bg-surface-container flex items-center justify-center text-on-surface hover:bg-surface-container-high hover:shadow-md disabled:opacity-30 disabled:shadow-none transition-all duration-200 active:scale-90 disabled:active:scale-100">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M12 4v16m8-8H4"></path></svg>
          </button>
        </div>
      </div>

      <div class="p-6 bg-surface-container-lowest">
        <div class="flex justify-between items-center mb-6 px-2">
          <span :class="['font-bold text-on-surface-variant uppercase tracking-widest', fontSm]">Item Total</span>
          <span :class="['font-black text-primary-container', font2xl]">₱{{ totalValue.toFixed(2) }}</span>
        </div>
        <BaseButton variant="primary" @click="handleConfirm" class="w-full py-4 text-lg shadow-[0_8px_20px_rgba(255,109,0,0.25)] hover:shadow-[0_12px_28px_rgba(255,109,0,0.4)] transition-all duration-300">
          <span class="font-black uppercase tracking-widest">Add to Cart</span>
        </BaseButton>
      </div>

    </div>
  </div>
</template>