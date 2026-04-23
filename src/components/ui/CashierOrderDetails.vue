<script setup lang="ts">
import BaseButton from './BaseButton.vue';
import { useResponsive } from '../../composables/useResponsive';
import type { ActiveOrder } from '../../services/posService';
import type { PendingOrder } from '../../services/queueService';

const { fontSm, fontBase, fontLg, fontXl } = useResponsive();

defineProps<{
  selectedOrder: ActiveOrder | null;
  selectedPending: PendingOrder | null;
  searchQueueInput: string;
  isSearching: boolean;
  tableNumberInput: string;
}>();

const emit = defineEmits<{
  (e: 'update:searchQueueInput', val: string): void;
  (e: 'update:tableNumberInput', val: string): void;
  (e: 'search'): void;
  (e: 'clear-pending'): void;
  (e: 'reject-pending'): void;
  (e: 'accept-pending'): void;
  (e: 'update-status', status: string): void;
  (e: 'settle-payment'): void;
}>();

// Force the input to strictly be digits between 1 and 100
function validateTableInput(e: Event) {
  const target = e.target as HTMLInputElement;
  let val = target.value.replace(/\D/g, '');
  if (val !== '') {
    let num = parseInt(val, 10);
    if (num > 100) val = '100';
  }
  target.value = val; // Force UI update
  emit('update:tableNumberInput', val);
}
</script>

<template>
  <div class="flex-1 bg-white rounded-2xl overflow-hidden shadow-sm border border-gray-100 flex flex-col min-h-0 relative">
    
    <div class="sticky top-0 z-40 bg-white/95 backdrop-blur px-5 pt-5 pb-3 border-b border-gray-200 flex flex-col lg:flex-row justify-between items-start lg:items-center gap-3 shrink-0">
      <div>
        <h2 :class="['font-extrabold text-gray-900 tracking-tight', fontLg]">Order Details</h2>
        <p :class="['font-medium text-gray-500', fontSm]">Manage and fulfill selected tab</p>
      </div>
      
      <div class="flex items-center gap-2 w-full lg:w-auto">
        <div class="relative flex-1 lg:w-40">
          <input 
            :value="searchQueueInput" 
            @input="emit('update:searchQueueInput', ($event.target as HTMLInputElement).value)"
            @keyup.enter="emit('search')"
            type="number" 
            placeholder="Queue #" 
            :class="['w-full border-2 border-gray-200 rounded-xl pl-9 pr-3 py-1.5 font-bold focus:outline-none focus:border-blue-500 transition-colors', fontSm]"
          />
          <svg class="w-4 h-4 text-gray-400 absolute left-3 top-1/2 -translate-y-1/2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
        </div>
        <BaseButton variant="primary" @click="emit('search')" :disabled="!searchQueueInput || isSearching" class="py-1.5 px-3">
          <span :class="fontSm">{{ isSearching ? '...' : 'Search' }}</span>
        </BaseButton>
      </div>
    </div>

    <div class="flex-1 p-4 bg-gray-50/30 flex flex-col min-h-0">
      
      <div v-if="!selectedOrder && !selectedPending" class="h-full flex flex-col items-center justify-center text-gray-400 space-y-3 m-auto">
        <svg class="w-12 h-12 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path></svg>
        <p :class="['font-medium text-center', fontBase]">Select an active tab from the right,<br/>or search a Customer Queue # above.</p>
      </div>

      <div v-else-if="selectedPending" class="h-full max-w-2xl w-full mx-auto flex flex-col gap-3 animate-in fade-in duration-300">
        
        <div class="bg-white border border-gray-200 rounded-xl p-4 shadow-sm relative flex flex-col flex-1 min-h-0">
          <div class="bg-blue-400 absolute top-0 left-0 w-full h-1 animate-pulse"></div>
          
          <div class="flex justify-between items-start mb-3 shrink-0">
            <div>
              <span class="text-blue-500 font-bold tracking-wide uppercase text-[10px]">Kiosk Order Ready</span>
              <h3 :class="['font-black text-gray-900 mt-0.5', fontLg]">Queue #{{ selectedPending.queue_number }}</h3>
              <p :class="['font-medium text-gray-500', fontSm]">{{ selectedPending.order_type }}</p>
            </div>
            <button @click="emit('clear-pending')" class="text-gray-400 hover:bg-gray-100 p-1 rounded-full transition-colors">
               <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
            </button>
          </div>

          <div class="flex-1 overflow-y-auto border-t border-b border-gray-100 py-2 my-2 pr-2">
            <h4 class="font-bold text-gray-800 mb-2 text-xs uppercase tracking-wide sticky top-0 bg-white z-10 py-1">Customer Cart</h4>
            <div class="space-y-1.5">
              <div v-for="(item, idx) in selectedPending.cart_items" :key="idx" class="flex justify-between items-center bg-gray-50 py-1.5 px-2 rounded border border-gray-100">
                <div>
                  <p :class="['font-bold text-gray-800', fontSm]">{{ item.qty }}x {{ item.pos_display_name }}</p>
                </div>
                <span :class="['font-bold text-gray-600', fontSm]">₱{{ (item.unit_price * item.qty).toFixed(2) }}</span>
              </div>
            </div>
          </div>

          <div class="pt-2 flex justify-between items-center shrink-0">
            <span :class="['font-bold text-gray-800', fontSm]">Total Value</span>
            <span :class="['font-black text-gray-900', fontLg]">₱{{ selectedPending.total.toFixed(2) }}</span>
          </div>
        </div>

        <div class="bg-gray-50 p-3 rounded-xl border border-gray-200 shrink-0">
          <label :class="['block font-bold text-gray-700 mb-1.5', fontSm]">Assign Table Number (1-100)</label>
          <input 
            :value="tableNumberInput" 
            @input="validateTableInput"
            type="text" 
            pattern="[0-9]*"
            placeholder="e.g., 5" 
            :class="['w-full border-2 border-gray-300 rounded-lg px-3 py-2 font-bold focus:outline-none focus:border-blue-500 transition-colors', fontBase]"
          />
        </div>

        <div class="flex flex-col sm:flex-row gap-2 shrink-0">
          <BaseButton variant="danger" @click="emit('reject-pending')" class="flex-none py-2 px-4 shadow-sm bg-white border border-red-200 text-red-600! hover:bg-red-50 hover:border-red-300">
            <span :class="fontSm">Reject</span>
          </BaseButton>
          <BaseButton variant="primary" @click="emit('accept-pending')" :disabled="!tableNumberInput" class="flex-1 py-2 shadow-sm hover:shadow-md transition-all">
            <span :class="fontBase">Accept & Send</span>
          </BaseButton>
        </div>
      </div>

      <div v-else-if="selectedOrder" class="h-full max-w-2xl w-full mx-auto flex flex-col gap-3 animate-in fade-in duration-300">
        
        <div class="bg-white border border-gray-200 rounded-xl p-4 shadow-sm relative flex flex-col flex-1 min-h-0">
          <div :class="selectedOrder.status === 'Cooking' ? 'bg-orange-400' : 'bg-green-400'" class="absolute top-0 left-0 w-full h-1"></div>
          
          <div class="flex justify-between items-start mb-3 shrink-0">
            <div>
              <span class="text-gray-500 font-bold tracking-wide uppercase text-[10px]">Order #{{ selectedOrder.order_id }}</span>
              <h3 :class="['font-black text-gray-900 mt-0.5', fontLg]">{{ selectedOrder.customer_identifier }}</h3>
              <p :class="['font-medium text-gray-500', fontSm]">{{ selectedOrder.order_type }}</p>
            </div>
            <span :class="[selectedOrder.status === 'Cooking' ? 'bg-orange-50 text-orange-600 animate-pulse' : 'bg-green-50 text-green-600', 'px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-wider']">
              {{ selectedOrder.status }}
            </span>
          </div>

          <div class="flex-1 overflow-y-auto border-t border-b border-gray-100 py-2 my-2 pr-2">
            <div class="h-full flex flex-col justify-center items-center">
               <p class="text-gray-400 text-sm font-medium text-center">Cart items will be dynamically listed here in future updates.</p>
            </div>
          </div>

          <div class="pt-2 flex justify-between items-center shrink-0">
            <span :class="['font-bold text-gray-800', fontSm]">Total Due</span>
            <span :class="['font-black text-blue-600', fontLg]">₱{{ selectedOrder.total_amount.toFixed(2) }}</span>
          </div>
        </div>

        <div class="flex flex-col sm:flex-row gap-2 shrink-0">
          <BaseButton v-if="selectedOrder.status === 'Cooking'" variant="secondary" @click="emit('update-status', 'Cooked')" class="flex-1 py-2 shadow-sm">
            <span :class="fontBase">Mark Cooked</span>
          </BaseButton>
          <BaseButton variant="success" @click="emit('settle-payment')" class="flex-1 py-2 shadow-sm hover:shadow-md transition-all">
            <span :class="fontBase">Settle Payment</span>
          </BaseButton>
        </div>
      </div>
    </div>
  </div>
</template>