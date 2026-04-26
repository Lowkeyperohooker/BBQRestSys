<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import BaseButton from '../ui/BaseButton.vue';
import { useResponsive } from '../../composables/useResponsive';
import type { PosItem } from '../../services/posService';

const { fontSm, fontBase, fontLg, fontXl, font2xl } = useResponsive();

const props = defineProps<{
  isOpen: boolean;
  item: PosItem | null;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'confirm', payload: { qty: number; customPrice: number }): void;
}>();

const qty = ref(1);
const customPrice = ref(0);

watch(() => props.isOpen, (newVal) => {
  if (newVal && props.item) {
    qty.value = 1;
    customPrice.value = props.item.unit_price;
  }
});

const totalValue = computed(() => {
  return qty.value * customPrice.value;
});

function increment() {
  if (props.item && qty.value < props.item.current_stock_pieces) {
    qty.value++;
  }
}

function decrement() {
  if (qty.value > 1) {
    qty.value--;
  }
}

function handleConfirm() {
  if (!props.item) return;
  if (props.item.is_variable_price && (!customPrice.value || customPrice.value <= 0)) {
    alert("Please enter a valid price greater than 0.");
    return;
  }
  emit('confirm', { qty: qty.value, customPrice: Number(customPrice.value) });
}
</script>

<template>
  <div v-if="isOpen && item" class="fixed inset-0 z-110 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md" @click.self="$emit('close')">
    <div class="bg-surface-container-low border border-outline-variant/20 w-full max-w-sm rounded-3xl shadow-2xl flex flex-col overflow-hidden animate-in zoom-in-95 duration-200">
      
      <div class="p-5 border-b border-outline-variant/10 bg-surface-container-highest/30 flex justify-between items-start">
        <div>
          <span v-if="item.is_variable_price" class="bg-tertiary-container/10 text-tertiary-container border border-tertiary-container/20 text-[10px] font-bold px-3 py-1 rounded-full uppercase tracking-wider mb-3 inline-block">Variable Price</span>
          <h3 :class="['font-black text-on-surface leading-tight', fontXl]">{{ item.pos_display_name }}</h3>
          <p :class="['text-on-surface-variant font-medium mt-1', fontSm]">Stock: {{ (item.current_stock_pieces > 0)?"Available":"Out of stock"  }}</p>
        </div>
        <button @click="$emit('close')" class="text-on-surface-variant hover:text-on-surface bg-surface-container hover:bg-surface-container-high p-2 rounded-full transition-colors ml-4 active:scale-90">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="p-6 space-y-6 bg-surface">
        <div class="flex flex-col items-center">
          <label :class="['font-bold text-on-surface-variant mb-3 uppercase tracking-widest', fontSm]">Quantity</label>
          <div class="flex items-center gap-4 bg-surface-container-low p-2 rounded-2xl border border-outline-variant/15">
            <button @click="decrement" :disabled="qty <= 1" class="w-12 h-12 rounded-xl bg-surface-container border border-outline-variant/20 flex items-center justify-center text-on-surface hover:bg-surface-container-high disabled:opacity-50 transition-colors active:scale-90 disabled:active:scale-100">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4"></path></svg>
            </button>
            <span :class="['w-12 text-center font-black text-on-surface', font2xl]">{{ qty }}</span>
            <button @click="increment" :disabled="qty >= item.current_stock_pieces" class="w-12 h-12 rounded-xl bg-surface-container border border-outline-variant/20 flex items-center justify-center text-on-surface hover:bg-surface-container-high disabled:opacity-50 transition-colors active:scale-90 disabled:active:scale-100">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path></svg>
            </button>
          </div>
        </div>

        <div v-if="item.is_variable_price" class="flex flex-col">
          <label :class="['font-bold text-on-surface-variant mb-3 uppercase tracking-widest text-center', fontSm]">Set Item Price (PHP)</label>
          <input 
            v-model.number="customPrice" 
            type="number" 
            min="1"
            class="w-full text-center border border-tertiary/30 bg-tertiary/10 text-tertiary rounded-xl px-4 py-3 font-black text-xl focus:outline-none focus:border-tertiary focus:ring-1 focus:ring-tertiary transition-colors"
          />
        </div>
      </div>

      <div class="p-6 bg-surface-container-highest/20 border-t border-outline-variant/10">
        <div class="flex justify-between items-center mb-5">
          <span :class="['font-bold text-on-surface-variant uppercase tracking-widest', fontSm]">Item Total</span>
          <span :class="['font-black text-primary-container', font2xl]">₱{{ totalValue.toFixed(2) }}</span>
        </div>
        <BaseButton variant="primary" @click="handleConfirm" class="w-full py-4 text-lg">
          <span class="font-black uppercase tracking-widest">Add to Cart</span>
        </BaseButton>
      </div>

    </div>
  </div>
</template>