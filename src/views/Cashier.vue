<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { posService, type ActiveOrder, type PosItem } from '../services/posService';
import { queueService, type PendingOrder } from '../services/queueService';
import DataLoader from '../components/ui/DataLoader.vue';
import CashierOrderDetails from '../components/ui/CashierOrderDetails.vue';
import { useResponsive } from '../composables/useResponsive';
import { useAuth } from '../stores/authStore';

const { fontSm, fontBase, isMobile, isTablet } = useResponsive();
const { currentUser } = useAuth();

const activeOrders = ref<ActiveOrder[]>([]);
const selectedOrder = ref<ActiveOrder | null>(null);
const availableItems = ref<PosItem[]>([]);

const selectedPending = ref<PendingOrder | null>(null);
const searchQueueInput = ref('');
const isSearching = ref(false);

const tableNumberInput = ref('');
const isLoadingData = ref(true);

const staffId = currentUser.value?.id || 1;

async function loadData() {
  isLoadingData.value = true;
  try {
    activeOrders.value = await posService.getActiveOrders();
    availableItems.value = await posService.getAvailablePosItems(); 
    
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
      // Map the backend data to match the strict CartItem frontend interface
      found.cart_items = found.cart_items.map((item: any) => ({
        prep_item_id: item.prep_item_id,
        pos_display_name: item.pos_display_name,
        qty: item.qty,
        unit_price: item.unit_price,
        is_variable_price: item.is_variable_price || false,
        photo_url: item.photo_url || null,
        category: "Kiosk Order",
        current_stock_pieces: 0,
        variant_group: null,
        variant_name: null
      }));

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

async function handleSaveActiveEdit(updatedOrder: ActiveOrder) {
  try {
    await posService.editActiveOrder(
      updatedOrder.order_id, 
      staffId, 
      updatedOrder.cart_items as any, 
      updatedOrder.total_amount, 
      0, 
      updatedOrder.total_amount
    );
    await loadData();
    alert("Order successfully updated.");
  } catch (error) {
    console.error("Failed to edit order:", error);
    alert("An error occurred saving the edit.");
  }
}

function handleSavePendingEdit(updatedPending: PendingOrder) {
  selectedPending.value = updatedPending;
}

onMounted(() => {
  loadData();
});
</script>

<template>
  <div class="w-full max-w-400 mx-auto h-full flex flex-row gap-2 md:gap-4 lg:gap-6 items-stretch">

    <div class="w-[calc(70%-4px)] md:w-[calc(60%-8px)] lg:w-[calc(60%-12px)] h-full min-h-0 transition-all duration-300">
      <CashierOrderDetails 
        :selected-order="selectedOrder"
        :selected-pending="selectedPending"
        :available-items="availableItems"
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
        @save-active-edit="handleSaveActiveEdit"
        @save-pending-edit="handleSavePendingEdit"
      />
    </div>

    <div class="w-[calc(30%-4px)] md:w-[calc(40%-8px)] lg:w-[calc(40%-12px)] h-full flex flex-col bg-surface-container-low border border-outline-variant/15 rounded-2xl shadow-sm shrink-0 overflow-hidden transition-all duration-300">
      
      <div class="p-2 sm:p-4 border-b border-outline-variant/20 bg-surface-container-highest/20 flex flex-wrap justify-between items-center gap-1 shrink-0">
        <h3 :class="['font-black text-on-surface tracking-wide uppercase truncate', fontBase]">Active Tabs</h3>
        <span v-if="activeOrders.length > 0" class="bg-error text-on-error text-[9px] md:text-[10px] font-black px-2 py-0.5 rounded-full shadow-sm shrink-0">{{ activeOrders.length }}</span>
      </div>

      <div class="flex-1 p-2 sm:p-3 overflow-y-auto bg-surface min-h-0">
        <DataLoader v-if="isLoadingData" message="Loading..." />
        
        <div v-else-if="activeOrders.length === 0" class="h-full flex flex-col items-center justify-center text-on-surface-variant pt-10 text-center">
          <svg class="w-6 h-6 md:w-8 md:h-8 mb-2 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path></svg>
          <p :class="['font-medium uppercase tracking-widest text-[9px] sm:text-xs', fontSm]">No tabs open</p>
        </div>

        <div v-else class="space-y-2">
          <div v-for="order in activeOrders" :key="order.order_id" @click="selectOrder(order)"
            :class="['p-2 sm:p-3 rounded-xl cursor-pointer transition-all border', selectedOrder?.order_id === order.order_id ? 'border-primary-container bg-primary-container/5 shadow-[0_0_12px_rgba(255,109,0,0.1)]' : 'border-outline-variant/15 bg-surface-container hover:border-outline-variant/30']">
            
            <div class="flex flex-col xl:flex-row justify-between items-start xl:items-center mb-2 gap-1">
              <h4 :class="['font-black text-on-surface line-clamp-1 w-full', fontSm]">{{ order.customer_identifier }}</h4>
              <span :class="[order.status === 'Cooking' ? 'text-tertiary-container bg-tertiary-container/10 border-tertiary-container/20' : 'text-tertiary bg-tertiary/10 border-tertiary/20', 'font-bold text-[8px] md:text-[10px] uppercase tracking-wider px-1.5 md:px-2 py-0.5 rounded-full border shrink-0']">
                {{ order.status === 'Cooking' ? 'Grilling' : 'Ready' }}
              </span>
            </div>
            
            <div class="flex flex-col xl:flex-row justify-between items-start xl:items-end gap-1 mt-2 xl:mt-0">
              <span class="text-on-surface-variant font-bold text-[8px] md:text-[10px] uppercase tracking-widest truncate w-full">#{{ order.order_id }} • {{ order.order_type }}</span>
              <span :class="['font-black text-primary shrink-0', fontBase]">₱{{ order.total_amount.toFixed(2) }}</span>
            </div>
            
          </div>
        </div>

      </div>
    </div>
  </div>
</template>