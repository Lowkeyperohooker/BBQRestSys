<script setup lang="ts">
import { useResponsive } from '../../composables/useResponsive';
import type { ActiveOrder } from '../../services/posService';

const { fontSm, fontBase } = useResponsive();

defineProps<{
  isOpen: boolean;
  activeOrders: ActiveOrder[];
  selectedOrderId?: number | null;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'select-order', order: ActiveOrder): void;
}>();
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-surface/80 backdrop-blur-sm">
    <div class="bg-surface-container-low border border-outline-variant/20 rounded-2xl shadow-xl w-full max-w-md flex flex-col max-h-[80vh] overflow-hidden">
      
      <div class="p-4 border-b border-outline-variant/20 bg-surface-container-highest/20 flex justify-between items-center shrink-0">
        <h3 class="font-black text-on-surface tracking-wide uppercase">Active Tabs</h3>
        <button @click="emit('close')" class="text-on-surface-variant hover:text-on-surface transition-colors">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="flex-1 p-3 overflow-y-auto bg-surface min-h-0">
        <div v-if="activeOrders.length === 0" class="h-full flex flex-col items-center justify-center text-on-surface-variant py-10 text-center">
          <svg class="w-8 h-8 mb-2 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path></svg>
          <p :class="['font-medium uppercase tracking-widest text-xs', fontSm]">No tabs open</p>
        </div>

        <div v-else class="space-y-2">
          <div v-for="order in activeOrders" :key="order.order_id" @click="emit('select-order', order); emit('close')"
            :class="['p-3 rounded-xl cursor-pointer transition-all border', selectedOrderId === order.order_id ? 'border-primary-container bg-primary-container/5 shadow-[0_0_12px_rgba(255,109,0,0.1)]' : 'border-outline-variant/15 bg-surface-container hover:border-outline-variant/30']">
            
            <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-2 gap-1">
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