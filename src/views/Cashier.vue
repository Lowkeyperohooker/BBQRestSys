<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { posService, type ActiveOrder } from '../services/posService';
import { queueService, type PendingOrder } from '../services/queueService';
import DataLoader from '../components/ui/DataLoader.vue';
import CashierOrderDetails from '../components/ui/CashierOrderDetails.vue';
import { useResponsive } from '../composables/useResponsive';
import { useAuth } from '../stores/authStore';

const { fontSm, fontBase, isMobile, isTablet } = useResponsive();
const { currentUser } = useAuth();

// Database Orders
const activeOrders = ref<ActiveOrder[]>([]);
const selectedOrder = ref<ActiveOrder | null>(null);

// JSON Kiosk Queue
const selectedPending = ref<PendingOrder | null>(null);
const searchQueueInput = ref('');
const isSearching = ref(false);

// Table Number Input
const tableNumberInput = ref('');

// UI State
const isLoadingData = ref(true);

const staffId = currentUser.value?.id || 1;

async function loadData() {
  isLoadingData.value = true;
  try {
    activeOrders.value = await posService.getActiveOrders();
    
    if (selectedOrder.value) {
      selectedOrder.value = activeOrders.value.find(o => o.order_id === selectedOrder.value?.order_id) || null;
    }
  } catch (error) {
    console.error("Failed to load tabs:", error);
  } finally {
    setTimeout(() => { isLoadingData.value = false; }, 600);
  }
}

function selectOrder(order: ActiveOrder) {
  selectedOrder.value = order;
  selectedPending.value = null; 
}

async function handleSearchQueue() {
  if (!searchQueueInput.value) return;
  
  isSearching.value = true;
  try {
    const queueNum = parseInt(searchQueueInput.value);
    const currentQueue = await queueService.getQueue();
    const found = currentQueue.find(q => q.queue_number === queueNum);
    
    if (found) {
      selectedPending.value = found;
      selectedOrder.value = null; 
      searchQueueInput.value = ''; 
      tableNumberInput.value = ''; 
    } else {
      alert(`Queue #${queueNum} not found. It may have already been processed.`);
    }
  } catch (error) {
    console.error("Search failed:", error);
    alert("Error communicating with the queue system.");
  } finally {
    isSearching.value = false;
  }
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
    selectedOrder.value = null;
    await loadData();
  } catch (error) {
    console.error("Failed to settle payment:", error);
    alert("An error occurred while settling payment.");
  }
}

async function handleAcceptPending() {
  if (!selectedPending.value) return;
  
  if (!tableNumberInput.value) {
    alert("Please enter the Table Number (1-100).");
    return;
  }

  try {
    const finalIdentifier = `Queue #${selectedPending.value.queue_number} - Table ${tableNumberInput.value}`;

    await posService.sendToGrill(
      staffId,
      finalIdentifier, 
      selectedPending.value.order_type,
      selectedPending.value.cart_items,
      selectedPending.value.subtotal,
      selectedPending.value.tax,
      selectedPending.value.total
    );

    await queueService.removeFromQueue(selectedPending.value.queue_number);
    
    alert(`${finalIdentifier} accepted and sent to grill!`);
    selectedPending.value = null;
    tableNumberInput.value = ''; 
    await loadData(); 
  } catch (error) {
    console.error("Failed to accept pending order:", error);
    alert("Failed to process queue order.");
  }
}

async function handleRejectPending() {
  if (!selectedPending.value) return;
  const confirmReject = confirm(`Are you sure you want to delete Queue #${selectedPending.value.queue_number}? This cannot be undone.`);
  if (!confirmReject) return;

  try {
    await queueService.removeFromQueue(selectedPending.value.queue_number);
    selectedPending.value = null;
    tableNumberInput.value = '';
  } catch(error) {
    console.error("Failed to reject pending order", error);
  }
}

onMounted(() => {
  loadData();
});
</script>

<template>
  <div :class="['h-full flex max-w-400 mx-auto', isMobile || isTablet ? 'flex-col gap-4' : 'flex-row gap-6']">

    <CashierOrderDetails 
      :selected-order="selectedOrder"
      :selected-pending="selectedPending"
      :search-queue-input="searchQueueInput"
      :is-searching="isSearching"
      :table-number-input="tableNumberInput"
      @update:search-queue-input="searchQueueInput = $event"
      @update:table-number-input="tableNumberInput = $event"
      @search="handleSearchQueue"
      @clear-pending="selectedPending = null"
      @reject-pending="handleRejectPending"
      @accept-pending="handleAcceptPending"
      @update-status="handleUpdateStatus"
      @settle-payment="handleSettlePayment"
    />

    <div :class="['bg-white border border-gray-100 rounded-2xl shadow-sm flex flex-col shrink-0', isMobile || isTablet ? 'h-96' : 'w-80 lg:w-96']">
      
      <div class="p-4 md:p-5 border-b border-gray-100 bg-gray-50/80 rounded-t-2xl flex justify-between items-center">
        <h3 :class="['font-extrabold text-gray-900', fontBase]">Active Tabs</h3>
        <span v-if="activeOrders.length > 0" class="bg-blue-600 text-white text-[10px] font-bold px-2 py-0.5 rounded-full shadow-sm">{{ activeOrders.length }}</span>
      </div>

      <div class="flex-1 p-3 md:p-4 overflow-y-auto">
        <DataLoader v-if="isLoadingData" message="Loading..." />
        
        <div v-else-if="activeOrders.length === 0" class="h-full flex flex-col items-center justify-center text-gray-400 pt-10">
          <svg class="w-8 h-8 mb-2 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path></svg>
          <p :class="['font-medium', fontSm]">No tabs open</p>
        </div>

        <div v-else class="space-y-2.5">
          <div v-for="order in activeOrders" :key="order.order_id" @click="selectOrder(order)"
            :class="['p-3.5 rounded-xl cursor-pointer transition-all border-2', selectedOrder?.order_id === order.order_id ? 'border-gray-800 bg-gray-50 shadow-md' : 'border-gray-100 bg-white hover:border-gray-300']">
            <div class="flex justify-between items-start mb-1.5">
              <h4 :class="['font-bold text-gray-900 line-clamp-1', fontSm]">{{ order.customer_identifier }}</h4>
              <span :class="[order.status === 'Cooking' ? 'text-orange-500' : 'text-green-500', 'font-bold text-xs']">
                {{ order.status === 'Cooking' ? 'Grilling' : 'Ready' }}
              </span>
            </div>
            <div class="flex justify-between items-end">
              <span class="text-gray-500 font-medium text-xs">#{{ order.order_id }} • {{ order.order_type }}</span>
              <span :class="['font-extrabold text-gray-900', fontBase]">₱{{ order.total_amount.toFixed(2) }}</span>
            </div>
          </div>
        </div>

      </div>
    </div>

  </div>
</template>