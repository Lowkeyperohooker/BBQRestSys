<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { posService, type ActiveOrder } from '../services/posService';
import { queueService, type PendingOrder } from '../services/queueService';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import PosLogsModal from '../components/ui/PosLogsModal.vue';
import { useResponsive } from '../composables/useResponsive';
import { useAuth } from '../stores/authStore';

const { fontSm, fontBase, fontLg, fontXl, isMobile, isTablet } = useResponsive();
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
const isLogsModalOpen = ref(false);

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

// Restricts input to only digits and max value of 100
function validateTableInput() {
  let val = tableNumberInput.value.replace(/\D/g, '');
  if (val !== '') {
    let num = parseInt(val, 10);
    if (num > 100) val = '100';
  }
  tableNumberInput.value = val;
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

    <div class="flex-1 bg-white rounded-2xl overflow-hidden shadow-sm border border-gray-100 flex flex-col min-h-0 relative">
      
      <div class="sticky top-0 z-40 bg-white/95 backdrop-blur px-5 pt-5 pb-3 border-b border-gray-200 flex flex-col lg:flex-row justify-between items-start lg:items-center gap-3">
        <div>
          <h2 :class="['font-extrabold text-gray-900 tracking-tight', fontLg]">Order Details</h2>
          <p :class="['font-medium text-gray-500', fontSm]">Manage and fulfill selected tab</p>
        </div>
        
        <div class="flex items-center gap-2 w-full lg:w-auto">
          <div class="relative flex-1 lg:w-40">
            <input 
              v-model="searchQueueInput" 
              @keyup.enter="handleSearchQueue"
              type="number" 
              placeholder="Queue #" 
              :class="['w-full border-2 border-gray-200 rounded-xl pl-9 pr-3 py-1.5 font-bold focus:outline-none focus:border-blue-500 transition-colors', fontSm]"
            />
            <svg class="w-4 h-4 text-gray-400 absolute left-3 top-1/2 -translate-y-1/2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
          </div>
          <BaseButton variant="primary" @click="handleSearchQueue" :disabled="!searchQueueInput || isSearching" class="py-1.5 px-3">
            <span :class="fontSm">{{ isSearching ? '...' : 'Search' }}</span>
          </BaseButton>
          <BaseButton variant="secondary" @click="isLogsModalOpen = true" class="hidden sm:block py-1.5 px-3">
             <span :class="fontSm">Logs</span>
          </BaseButton>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto p-4 bg-gray-50/30 flex flex-col">
        
        <div v-if="!selectedOrder && !selectedPending" class="h-full flex flex-col items-center justify-center text-gray-400 space-y-3 m-auto">
          <svg class="w-12 h-12 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path></svg>
          <p :class="['font-medium text-center', fontBase]">Select an active tab from the right,<br/>or search a Customer Queue # above.</p>
        </div>

        <div v-else-if="selectedPending" class="max-w-2xl w-full mx-auto space-y-3 animate-in fade-in duration-300">
          <div class="bg-white border border-gray-200 rounded-xl p-4 shadow-sm relative overflow-hidden">
            <div class="bg-blue-400 absolute top-0 left-0 w-full h-1 animate-pulse"></div>
            
            <div class="flex justify-between items-start mb-3">
              <div>
                <span class="text-blue-500 font-bold tracking-wide uppercase text-[10px]">Kiosk Order Ready</span>
                <h3 :class="['font-black text-gray-900 mt-0.5', fontLg]">Queue #{{ selectedPending.queue_number }}</h3>
                <p :class="['font-medium text-gray-500', fontSm]">{{ selectedPending.order_type }}</p>
              </div>
              <button @click="selectedPending = null" class="text-gray-400 hover:bg-gray-100 p-1 rounded-full transition-colors">
                 <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
              </button>
            </div>

            <div class="border-t border-b border-gray-100 py-2 my-2 space-y-1">
              <h4 class="font-bold text-gray-800 mb-1 text-xs uppercase tracking-wide">Customer Cart</h4>
              <div v-for="(item, idx) in selectedPending.cart_items" :key="idx" class="flex justify-between items-center bg-gray-50 py-1.5 px-2 rounded border border-gray-100">
                <div>
                  <p :class="['font-bold text-gray-800', fontSm]">{{ item.qty }}x {{ item.pos_display_name }}</p>
                </div>
                <span :class="['font-bold text-gray-600', fontSm]">₱{{ (item.unit_price * item.qty).toFixed(2) }}</span>
              </div>
            </div>

            <div class="pt-1 flex justify-between items-center">
              <span :class="['font-bold text-gray-800', fontSm]">Total Value</span>
              <span :class="['font-black text-gray-900', fontLg]">₱{{ selectedPending.total.toFixed(2) }}</span>
            </div>
          </div>

          <div class="bg-gray-50 p-3 rounded-xl border border-gray-200">
            <label :class="['block font-bold text-gray-700 mb-1.5', fontSm]">Assign Table Number (1-100)</label>
            <input 
              v-model="tableNumberInput" 
              @input="validateTableInput"
              type="text" 
              pattern="[0-9]*"
              placeholder="e.g., 5" 
              :class="['w-full border-2 border-gray-300 rounded-lg px-3 py-2 font-bold focus:outline-none focus:border-blue-500 transition-colors', fontBase]"
            />
          </div>

          <div class="flex flex-col sm:flex-row gap-2">
            <BaseButton variant="danger" @click="handleRejectPending" class="flex-none py-2 px-4 shadow-sm bg-white border border-red-200 text-red-600! hover:bg-red-50 hover:border-red-300">
              <span :class="fontSm">Reject</span>
            </BaseButton>
            <BaseButton variant="primary" @click="handleAcceptPending" :disabled="!tableNumberInput" class="flex-1 py-2 shadow-sm hover:shadow-md transition-all">
              <span :class="fontBase">Accept & Send</span>
            </BaseButton>
          </div>
        </div>

        <div v-else-if="selectedOrder" class="max-w-2xl w-full mx-auto space-y-3 animate-in fade-in duration-300">
          <div class="bg-white border border-gray-200 rounded-xl p-4 shadow-sm relative overflow-hidden">
            <div :class="selectedOrder.status === 'Cooking' ? 'bg-orange-400' : 'bg-green-400'" class="absolute top-0 left-0 w-full h-1"></div>
            
            <div class="flex justify-between items-start mb-3">
              <div>
                <span class="text-gray-500 font-bold tracking-wide uppercase text-[10px]">Order #{{ selectedOrder.order_id }}</span>
                <h3 :class="['font-black text-gray-900 mt-0.5', fontLg]">{{ selectedOrder.customer_identifier }}</h3>
                <p :class="['font-medium text-gray-500', fontSm]">{{ selectedOrder.order_type }}</p>
              </div>
              <span :class="[selectedOrder.status === 'Cooking' ? 'bg-orange-50 text-orange-600 animate-pulse' : 'bg-green-50 text-green-600', 'px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-wider']">
                {{ selectedOrder.status }}
              </span>
            </div>

            <div class="border-t border-b border-gray-100 py-3 my-3">
              <h4 class="font-bold text-gray-800 mb-2 text-xs uppercase tracking-wide">Order Value</h4>
              <div class="pt-1 flex justify-between items-center">
                <span :class="['font-bold text-gray-800', fontSm]">Total Due</span>
                <span :class="['font-black text-blue-600', fontLg]">₱{{ selectedOrder.total_amount.toFixed(2) }}</span>
              </div>
            </div>
          </div>

          <div class="flex flex-col sm:flex-row gap-2">
            <BaseButton v-if="selectedOrder.status === 'Cooking'" variant="secondary" @click="handleUpdateStatus('Cooked')" class="flex-1 py-2 shadow-sm">
              <span :class="fontBase">Mark Cooked</span>
            </BaseButton>
            <BaseButton variant="success" @click="handleSettlePayment" class="flex-1 py-2 shadow-sm hover:shadow-md transition-all">
              <span :class="fontBase">Settle Payment</span>
            </BaseButton>
          </div>
        </div>
      </div>
    </div>

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

    <PosLogsModal :is-open="isLogsModalOpen" @close="isLogsModalOpen = false" />
  </div>
</template>