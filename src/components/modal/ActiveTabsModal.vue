<script setup lang="ts">
import { useResponsive } from '../../composables/useResponsive';
import DataLoader from '../ui/DataLoader.vue';
import type { ActiveOrder } from '../../services/posService';

const { fontSm, fontBase } = useResponsive();

defineProps<{
  show: boolean;
  activeOrders: ActiveOrder[];
  isLoadingData: boolean;
  selectedOrderId?: number;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'select-order', order: ActiveOrder): void;
}>();
</script>

<template>
  <div v-if="show" class="fixed inset-0 z-50 overflow-hidden">
    
    <div 
      class="absolute inset-0 bg-scrim/50 backdrop-blur-sm animate-in fade-in duration-300" 
      @click="emit('close')"
    ></div>

    <div class="absolute top-0 right-0 h-full w-full max-w-sm sm:max-w-md bg-surface shadow-2xl border-l border-outline-variant/20 flex flex-col animate-in slide-in-from-right duration-300">
      
      <div class="p-4 sm:p-5 border-b border-outline-variant/20 bg-surface-container-low flex justify-between items-center shrink-0">
        <div class="flex items-center gap-2">
          <h3 :class="['font-black text-on-surface tracking-wide uppercase', fontBase]">Active Tabs</h3>
          <span v-if="activeOrders.length > 0" class="bg-error text-on-error text-[10px] font-black px-2 py-0.5 rounded-full shadow-sm">
            {{ activeOrders.length }}
          </span>
        </div>
        <button @click="emit('close')" class="text-on-surface-variant hover:text-on-surface hover:bg-surface-container-high p-2 rounded-full transition-colors active:scale-90 shrink-0">
           <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="flex-1 p-4 overflow-y-auto bg-surface min-h-0">
        <DataLoader v-if="isLoadingData" message="Loading tabs..." />

        <div v-else-if="activeOrders.length === 0" class="h-full flex flex-col items-center justify-center text-on-surface-variant pt-10 text-center">
          <svg class="w-8 h-8 mb-2 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path></svg>
          <p :class="['font-medium uppercase tracking-widest text-xs', fontSm]">No tabs open</p>
        </div>

        <div v-else class="space-y-3">
          <div v-for="order in activeOrders" :key="order.order_id" @click="emit('select-order', order); emit('close')"
            :class="['p-3 rounded-xl cursor-pointer transition-all border', selectedOrderId === order.order_id ? 'border-primary-container bg-primary-container/5 shadow-[0_0_12px_rgba(255,109,0,0.1)]' : 'border-outline-variant/15 bg-surface-container hover:border-outline-variant/30']">
            
            <div class="flex justify-between items-center mb-2 gap-1">
              <h4 :class="['font-black text-on-surface line-clamp-1 w-full', fontSm]">{{ order.customer_identifier }}</h4>
              <span :class="[order.status === 'Cooking' ? 'text-tertiary-container bg-tertiary-container/10 border-tertiary-container/20' : 'text-tertiary bg-tertiary/10 border-tertiary/20', 'font-bold text-[10px] uppercase tracking-wider px-2 py-0.5 rounded-full border shrink-0']">
                {{ order.status === 'Cooking' ? 'Grilling' : 'Ready' }}
              </span>
            </div>
            
            <div class="flex justify-between items-end gap-1">
              <span class="text-on-surface-variant font-bold text-[10px] uppercase tracking-widest truncate w-full">#{{ order.order_id }} • {{ order.order_type }}</span>
              <span :class="['font-black text-primary shrink-0', fontBase]">₱{{ order.total_amount.toFixed(2) }}</span>
            </div>
            
          </div>
        </div>
      </div>
    </div>
  </div>
</template>