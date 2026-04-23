<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import BaseButton from './BaseButton.vue';
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

// Reset modal state every time it opens with a new item
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
  <div v-if="isOpen && item" class="fixed inset-0 z-[110] flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm" @click.self="$emit('close')">
    <div class="bg-white w-full max-w-sm rounded-3xl shadow-2xl flex flex-col overflow-hidden animate-in zoom-in-95 duration-200">
      
      <div class="p-5 border-b border-gray-100 bg-gray-50 flex justify-between items-start">
        <div>
          <span v-if="item.is_variable_price" class="bg-orange-100 text-orange-700 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider mb-2 inline-block">Variable Price</span>
          <h3 :class="['font-black text-gray-900 leading-tight', fontXl]">{{ item.pos_display_name }}</h3>
          <p :class="['text-gray-500 font-medium mt-1', fontSm]">Stock: {{ item.current_stock_pieces }}</p>
        </div>
        <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600 bg-gray-200/50 hover:bg-gray-200 p-2 rounded-full transition-colors ml-4">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="p-6 space-y-6">
        <div class="flex flex-col items-center">
          <label :class="['font-bold text-gray-500 mb-3 uppercase tracking-wider', fontSm]">Quantity</label>
          <div class="flex items-center gap-4 bg-gray-50 p-2 rounded-2xl border border-gray-100">
            <button @click="decrement" :disabled="qty <= 1" class="w-12 h-12 rounded-xl bg-white shadow-sm border border-gray-200 flex items-center justify-center text-gray-600 hover:bg-gray-100 disabled:opacity-50 transition-colors">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4"></path></svg>
            </button>
            <span :class="['w-12 text-center font-black text-gray-900', font2xl]">{{ qty }}</span>
            <button @click="increment" :disabled="qty >= item.current_stock_pieces" class="w-12 h-12 rounded-xl bg-white shadow-sm border border-gray-200 flex items-center justify-center text-gray-600 hover:bg-gray-100 disabled:opacity-50 transition-colors">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path></svg>
            </button>
          </div>
        </div>

        <div v-if="item.is_variable_price" class="flex flex-col">
          <label :class="['font-bold text-gray-500 mb-2 uppercase tracking-wider text-center', fontSm]">Set Item Price (PHP)</label>
          <input 
            v-model.number="customPrice" 
            type="number" 
            min="1"
            class="w-full text-center border-2 border-orange-200 bg-orange-50/50 rounded-xl px-4 py-3 font-black text-xl focus:outline-none focus:border-orange-500 transition-colors"
          />
        </div>
      </div>

      <div class="p-5 bg-gray-50 border-t border-gray-100">
        <div class="flex justify-between items-center mb-4">
          <span :class="['font-bold text-gray-500', fontBase]">Item Total</span>
          <span :class="['font-black text-blue-600', font2xl]">₱{{ totalValue.toFixed(2) }}</span>
        </div>
        <BaseButton variant="primary" @click="handleConfirm" class="w-full py-4 shadow-md hover:shadow-lg transition-all rounded-xl">
          <span :class="['font-black uppercase tracking-wider', fontLg]">Add to Cart</span>
        </BaseButton>
      </div>

    </div>
  </div>
</template>