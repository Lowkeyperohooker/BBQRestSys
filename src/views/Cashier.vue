<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { posService, type ActiveOrder } from '../services/posService';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import PosLogsModal from '../components/ui/PosLogsModal.vue';
import { useResponsive } from '../composables/useResponsive';
import { useAuth } from '../stores/authStore';

const { fontSm, fontBase, fontLg, fontXl, font2xl, isMobile, isTablet } = useResponsive();
const { currentUser } = useAuth();

const activeOrders = ref<ActiveOrder[]>([]);
const selectedOrder = ref<ActiveOrder | null>(null);

const isLoadingData = ref(true);
const isLogsModalOpen = ref(false);

const staffId = currentUser.value?.id || 1;

async function loadData() {
  isLoadingData.value = true;
  try {
    activeOrders.value = await posService.getActiveOrders();
    
    // Keep the currently selected order updated if it still exists in the active list
    if (selectedOrder.value) {
      const updatedOrder = activeOrders.value.find(o => o.order_id === selectedOrder.value?.order_id);
      selectedOrder.value = updatedOrder || null;
    }
  } catch (error) {
    console.error("Failed to load active tabs:", error);
  } finally {
    setTimeout(() => { isLoadingData.value = false; }, 600);
  }
}

function selectOrder(order: ActiveOrder) {
  selectedOrder.value = order;
}

async function handleUpdateStatus(newStatus: string) {
  if (!selectedOrder.value) return;
  try {
    await posService.updateOrderStatus(selectedOrder.value.order_id, newStatus, staffId);
    await loadData();
  } catch (error) {
    console.error("Failed to update status:", error);
    alert("An error occurred while updating the order status.");
  }
}

async function handleSettlePayment() {
  if (!selectedOrder.value) return;
  const confirmPayment = confirm(`Settle payment of PHP ${selectedOrder.value.total_amount.toFixed(2)} for ${selectedOrder.value.customer_identifier}?`);
  if (!confirmPayment) return;

  try {
    await posService.settlePayment(selectedOrder.value.order_id, staffId);
    alert(`Payment settled for ${selectedOrder.value.customer_identifier}!`);
    selectedOrder.value = null; // Clear selection after settling
    await loadData();
  } catch (error) {
    console.error("Failed to settle payment:", error);
    alert("An error occurred while settling payment.");
  }
}

onMounted(() => {
  loadData();
});
</script>

<template>
  <div :class="['h-full flex max-w-[1600px] mx-auto', isMobile || isTablet ? 'flex-col gap-4' : 'flex-row gap-6']">

    <div class="flex-1 bg-white rounded-2xl overflow-hidden shadow-sm border border-gray-100 flex flex-col min-h-0 relative">
      
      <div class="sticky top-0 z-40 bg-white/95 backdrop-blur px-6 pt-6 pb-4 border-b border-gray-200 flex justify-between items-center">
        <div>
          <h2 :class="['font-extrabold text-gray-900 tracking-tight', fontXl]">Order Details</h2>
          <p :class="['font-medium text-gray-500 mt-1', fontSm]">Manage and fulfill selected tab</p>
        </div>
        <BaseButton variant="secondary" @click="isLogsModalOpen = true">View Logs</BaseButton>
      </div>

      <div class="flex-1 overflow-y-auto p-6 md:p-8 bg-gray-50/30 flex flex-col">
        
        <div v-if="!selectedOrder" class="h-full flex flex-col items-center justify-center text-gray-400 space-y-4 m-auto">
          <svg class="w-20 h-20 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path></svg>
          <p :class="['font-medium', fontLg]">Select a tab from the right to view details.</p>
        </div>

        <div v-else class="max-w-2xl w-full mx-auto space-y-8">
          
          <div class="bg-white border border-gray-200 rounded-2xl p-6 md:p-8 shadow-sm relative overflow-hidden">
            <div :class="selectedOrder.status === 'Cooking' ? 'bg-orange-400' : 'bg-green-400'" class="absolute top-0 left-0 w-full h-1.5"></div>
            
            <div class="flex justify-between items-start mb-6">
              <div>
                <span :class="['text-gray-500 font-bold tracking-wide uppercase', fontSm]">Order #{{ selectedOrder.order_id }}</span>
                <h3 :class="['font-black text-gray-900 mt-1', font2xl]">{{ selectedOrder.customer_identifier }}</h3>
                <p :class="['font-medium text-gray-500 mt-1', fontBase]">{{ selectedOrder.order_type }}</p>
              </div>
              <span :class="[selectedOrder.status === 'Cooking' ? 'bg-orange-50 text-orange-600 animate-pulse' : 'bg-green-50 text-green-600', 'px-3 py-1.5 rounded-lg font-bold uppercase tracking-wider', fontSm]">
                {{ selectedOrder.status }}
              </span>
            </div>

            <div class="border-t border-b border-gray-100 py-6 my-6">
              <h4 :class="['font-bold text-gray-800 mb-4', fontBase]">Ordered Items</h4>
              <div class="bg-gray-50 rounded-xl p-6 text-center border border-dashed border-gray-200">
                <p :class="['text-gray-500', fontSm]">Line items will be displayed here once connected to the updated backend schema.</p>
              </div>
            </div>

            <div class="space-y-3">
              <div :class="['flex justify-between text-gray-500 font-medium', fontBase]">
                <span>Subtotal</span>
                <span>₱{{ selectedOrder.subtotal.toFixed(2) }}</span>
              </div>
              <div :class="['flex justify-between text-gray-500 font-medium', fontBase]">
                <span>Tax (0%)</span>
                <span>₱{{ selectedOrder.tax_amount.toFixed(2) }}</span>
              </div>
              <div class="pt-4 border-t border-gray-100 flex justify-between items-center">
                <span :class="['font-bold text-gray-800', fontLg]">Total Due</span>
                <span :class="['font-black text-blue-600', font2xl]">₱{{ selectedOrder.total_amount.toFixed(2) }}</span>
              </div>
            </div>
          </div>

          <div class="flex flex-col sm:flex-row gap-4">
            <BaseButton 
              v-if="selectedOrder.status === 'Cooking'" 
              variant="secondary" 
              @click="handleUpdateStatus('Cooked')" 
              class="flex-1 py-4 shadow-sm"
            >
              <span :class="fontLg">Mark as Cooked</span>
            </BaseButton>
            <BaseButton 
              variant="success" 
              @click="handleSettlePayment" 
              class="flex-1 py-4 shadow-lg hover:shadow-xl transition-all"
            >
              <span :class="fontLg">Settle Payment</span>
            </BaseButton>
          </div>

        </div>
      </div>
    </div>

    <div :class="['bg-white border border-gray-100 rounded-2xl shadow-sm flex flex-col shrink-0', isMobile || isTablet ? 'h-96' : 'w-80 lg:w-96']">
      <div class="p-5 md:p-6 border-b border-gray-100 bg-gray-50/80 rounded-t-2xl flex justify-between items-center">
        <h3 :class="['font-extrabold text-gray-900', fontLg]">Active Tabs</h3>
        <span class="bg-blue-600 text-white text-[11px] font-bold px-2.5 py-0.5 rounded-full shadow-sm">{{ activeOrders.length }}</span>
      </div>

      <div class="flex-1 p-4 overflow-y-auto">
        <DataLoader v-if="isLoadingData" message="Loading tabs..." />
        
        <div v-else-if="activeOrders.length === 0" class="h-full flex flex-col items-center justify-center text-gray-400">
          <svg class="w-12 h-12 mb-3 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path></svg>
          <p :class="['font-medium', fontBase]">No tabs open</p>
        </div>

        <div v-else class="space-y-3">
          <div 
            v-for="order in activeOrders" 
            :key="order.order_id" 
            @click="selectOrder(order)"
            :class="[
              'p-4 rounded-xl cursor-pointer transition-all border-2',
              selectedOrder?.order_id === order.order_id 
                ? 'border-blue-500 bg-blue-50/50 shadow-md' 
                : 'border-gray-100 bg-white hover:border-blue-200 hover:shadow-sm'
            ]"
          >
            <div class="flex justify-between items-start mb-2">
              <h4 :class="['font-bold text-gray-900 line-clamp-1', fontBase]">{{ order.customer_identifier }}</h4>
              <span :class="[order.status === 'Cooking' ? 'text-orange-500' : 'text-green-500', 'font-bold', fontSm]">
                {{ order.status === 'Cooking' ? 'Grilling' : 'Ready' }}
              </span>
            </div>
            <div class="flex justify-between items-end">
              <span :class="['text-gray-500 font-medium', fontSm]">#{{ order.order_id }} • {{ order.order_type }}</span>
              <span :class="['font-extrabold text-gray-900', fontBase]">₱{{ order.total_amount.toFixed(2) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <PosLogsModal :is-open="isLogsModalOpen" @close="isLogsModalOpen = false" />
  </div>
</template>