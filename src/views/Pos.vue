<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { posService, type PosItem, type CartItem, type ActiveOrder } from '../services/posService';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import PosLogsModal from '../components/ui/PosLogsModal.vue';

const availableItems = ref<PosItem[]>([]);
const cart = ref<CartItem[]>([]);
const activeOrders = ref<ActiveOrder[]>([]);
const currentStaffId = 1; 

// UI States
const isLoadingData = ref(true);
const isLogsModalOpen = ref(false);
const viewMode = ref<'new' | 'active'>('new'); // Switch between Menu and Active Tabs

// Order Details
const customerName = ref('');
const orderType = ref<'Dine-in' | 'Takeout'>('Dine-in');

async function loadData() {
  isLoadingData.value = true;
  try {
    const [items, orders] = await Promise.all([
      posService.getAvailablePosItems(),
      posService.getActiveOrders()
    ]);
    availableItems.value = items;
    activeOrders.value = orders;
  } catch (error) {
    console.error("Failed to load POS data:", error);
  } finally {
    setTimeout(() => {
      isLoadingData.value = false;
    }, 600);
  }
}

// Financial Computations
const subtotal = computed(() => {
  return cart.value.reduce((sum, item) => sum + (item.unit_price * item.qty), 0);
});

const tax = computed(() => subtotal.value * 0.0); 
const total = computed(() => subtotal.value + tax.value);

// Cart Actions
function addToCart(item: PosItem) {
  let finalPrice = item.unit_price;

  if (item.is_variable_price === 1) {
    const userInput = prompt(`Enter custom price for ${item.pos_display_name} (PHP):`, item.unit_price.toString());
    
    if (userInput === null) return; 
    
    const parsedPrice = parseFloat(userInput);
    if (isNaN(parsedPrice) || parsedPrice <= 0) {
      alert("Invalid price entered.");
      return;
    }
    finalPrice = parsedPrice;
  }

  const totalQtyInCart = cart.value
    .filter(c => c.prep_item_id === item.prep_item_id)
    .reduce((sum, c) => sum + c.qty, 0);

  if (totalQtyInCart >= item.current_stock_pieces) {
    alert(`Cannot add more. Only ${item.current_stock_pieces} in stock.`);
    return;
  }

  const existing = cart.value.find(c => c.prep_item_id === item.prep_item_id && c.unit_price === finalPrice);

  if (existing) {
    existing.qty++; 
  } else {
    cart.value.push({ ...item, unit_price: finalPrice, qty: 1 }); 
  }
}

function removeFromCart(index: number) {
  cart.value.splice(index, 1);
}

// 1. Send Order to the Grill (Unpaid Tab)
async function handleSendToGrill() {
  if (cart.value.length === 0) return;
  if (!customerName.value.trim()) {
    alert("Please enter a customer name or table number.");
    return;
  }

  try {
    const orderId = await posService.sendToGrill(
      currentStaffId,
      customerName.value,
      orderType.value,
      cart.value,
      subtotal.value,
      tax.value,
      total.value
    );

    alert(`Ticket #${orderId} sent to grill for ${customerName.value}!`);
    
    // Reset Form
    cart.value = [];
    customerName.value = '';
    orderType.value = 'Dine-in';
    
    // Refresh lists
    await loadData();

  } catch (error) {
    console.error("Checkout failed:", error);
    alert("Transaction failed. No inventory was deducted.");
  }
}

// 2. Settle the Customer's Tab when they are done eating
async function handleSettlePayment(order: ActiveOrder) {
  const confirmPayment = confirm(`Settle payment of PHP ${order.total_amount.toFixed(2)} for ${order.customer_identifier}?`);
  if (!confirmPayment) return;

  try {
    await posService.settlePayment(order.order_id, currentStaffId);
    alert(`Payment settled for ${order.customer_identifier}!`);
    await loadData(); // Refresh the active tabs list
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
  <div class="h-full flex bg-gray-50 rounded-xl overflow-hidden shadow-sm border border-gray-100">
    
    <div class="flex-1 p-6 overflow-y-auto flex flex-col">
      <div class="mb-6 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <div>
          <h2 class="text-2xl font-bold text-gray-800">Point of Sale</h2>
          <p class="text-sm text-gray-500">Manage orders and customer tabs.</p>
        </div>
        
        <div class="flex items-center gap-4">
          <div class="flex bg-gray-200 rounded-lg p-1">
            <button 
              @click="viewMode = 'new'"
              :class="viewMode === 'new' ? 'bg-white shadow text-gray-800' : 'text-gray-500 hover:text-gray-700'"
              class="px-4 py-1.5 text-sm font-medium rounded-md transition-all"
            >
              Take Order
            </button>
            <button 
              @click="viewMode = 'active'"
              :class="viewMode === 'active' ? 'bg-white shadow text-gray-800' : 'text-gray-500 hover:text-gray-700'"
              class="px-4 py-1.5 text-sm font-medium rounded-md transition-all flex items-center gap-2"
            >
              Active Tabs
              <span v-if="activeOrders.length > 0" class="bg-orange-500 text-white text-[10px] font-bold px-1.5 py-0.5 rounded-full">
                {{ activeOrders.length }}
              </span>
            </button>
          </div>

          <BaseButton variant="secondary" @click="isLogsModalOpen = true">
            View Logs
          </BaseButton>
        </div>
      </div>
      
      <div v-if="isLoadingData" class="flex-1 flex items-center justify-center">
        <DataLoader message="Loading register data..." />
      </div>

      <div v-else-if="viewMode === 'new'" class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
        <div 
          v-for="item in availableItems" 
          :key="item.prep_item_id"
          @click="addToCart(item)"
          class="bg-white p-4 rounded-xl shadow-sm border border-gray-100 cursor-pointer hover:border-blue-500 transition-all hover:shadow-md group relative"
        >
          <span v-if="item.is_variable_price === 1" class="absolute top-2 right-2 bg-orange-100 text-orange-700 text-[10px] font-bold px-2 py-0.5 rounded-full z-10">
            Variable
          </span>

          <div class="h-16 bg-blue-50 rounded-lg mb-3 flex items-center justify-center text-blue-500 group-hover:scale-105 transition-transform">
            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"></path></svg>
          </div>
          <h4 class="font-semibold text-gray-800 truncate" :title="item.pos_display_name">{{ item.pos_display_name }}</h4>
          <div class="flex justify-between items-center mt-1">
            <p class="text-blue-600 font-bold">
              <span v-if="item.is_variable_price === 1" class="text-xs text-gray-400 font-normal mr-1">Starts at</span>
              PHP {{ item.unit_price.toFixed(2) }}
            </p>
            <p class="text-xs text-gray-400">{{ item.current_stock_pieces }} left</p>
          </div>
        </div>

        <div v-if="availableItems.length === 0" class="col-span-full py-12 text-center text-gray-500">
          No items currently available. Please prepare skewers in the Prep Station first.
        </div>
      </div>

      <div v-else-if="viewMode === 'active'" class="flex-1">
        <div v-if="activeOrders.length === 0" class="flex flex-col items-center justify-center h-full text-gray-400 space-y-4">
          <svg class="w-16 h-16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path></svg>
          <p class="text-lg font-medium">No active tabs currently open.</p>
        </div>

        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div v-for="order in activeOrders" :key="order.order_id" class="bg-white border border-orange-200 rounded-xl p-5 shadow-sm flex flex-col relative overflow-hidden">
            <div class="absolute top-0 left-0 w-full h-1 bg-orange-400"></div>
            
            <div class="flex justify-between items-start mb-3 mt-1">
              <span class="text-sm text-gray-500 font-medium">Order #{{ order.order_id }}</span>
              <span class="bg-orange-100 text-orange-800 px-2 py-0.5 rounded text-xs font-bold animate-pulse">
                {{ order.status }}
              </span>
            </div>
            
            <h3 class="text-xl font-bold text-gray-900 truncate" :title="order.customer_identifier">
              {{ order.customer_identifier }}
            </h3>
            <p class="text-sm text-gray-500 mb-6">{{ order.order_type }}</p>
            
            <div class="mt-auto flex justify-between items-center border-t border-gray-100 pt-4">
              <span class="text-xl font-black text-gray-900">PHP {{ order.total_amount.toFixed(2) }}</span>
              <BaseButton variant="success" @click="handleSettlePayment(order)">
                Settle Payment
              </BaseButton>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-show="viewMode === 'new'" class="w-96 bg-white border-l border-gray-200 flex flex-col h-full z-10 transition-all">
      
      <div class="p-4 border-b border-gray-100 bg-gray-50">
        <h3 class="text-lg font-bold text-gray-800 mb-3">Order Details</h3>
        <input 
          v-model="customerName" 
          type="text" 
          placeholder="Table #" 
          class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 mb-3 bg-white"
        />
        <div class="flex bg-gray-200 rounded-lg p-1 w-full">
          <button @click="orderType = 'Dine-in'" :class="orderType === 'Dine-in' ? 'bg-white shadow text-gray-800' : 'text-gray-500'" class="flex-1 py-1 text-sm font-medium rounded-md transition-all">Dine-in</button>
          <button @click="orderType = 'Takeout'" :class="orderType === 'Takeout' ? 'bg-white shadow text-gray-800' : 'text-gray-500'" class="flex-1 py-1 text-sm font-medium rounded-md transition-all">Takeout</button>
        </div>
      </div>
      
      <div class="flex-1 p-4 overflow-y-auto bg-white">
        <div v-if="cart.length === 0" class="h-full flex flex-col items-center justify-center text-gray-400">
          <svg class="w-12 h-12 mb-2 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
          <p>Cart is empty</p>
        </div>
        
        <div v-else class="space-y-4">
          <div v-for="(cartItem, index) in cart" :key="index" class="flex justify-between items-center group">
            <div class="flex-1">
              <p class="font-semibold text-gray-800">{{ cartItem.pos_display_name }}</p>
              <p class="text-sm text-gray-500">PHP {{ cartItem.unit_price.toFixed(2) }} x {{ cartItem.qty }}</p>
            </div>
            <div class="flex items-center gap-3">
              <span class="font-bold text-gray-800">PHP {{ (cartItem.unit_price * cartItem.qty).toFixed(2) }}</span>
              <button @click="removeFromCart(index)" class="text-red-400 hover:text-red-600 opacity-0 group-hover:opacity-100 transition-opacity">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="p-6 bg-gray-50 border-t border-gray-200">
        <div class="flex justify-between mb-2 text-gray-600">
          <span>Subtotal</span>
          <span>PHP {{ subtotal.toFixed(2) }}</span>
        </div>
        <div class="flex justify-between mb-6 text-xl font-bold text-gray-900 border-t border-gray-200 pt-4">
          <span>Total</span>
          <span>PHP {{ total.toFixed(2) }}</span>
        </div>
        <BaseButton 
          variant="primary"
          @click="handleSendToGrill"
          :disabled="cart.length === 0 || !customerName.trim()"
          class="w-full py-4 text-lg"
        >
          Send to Grill
        </BaseButton>
        <p v-if="cart.length > 0 && !customerName.trim()" class="text-xs text-red-500 text-center mt-2 font-medium">
          * Please enter customer name/table
        </p>
      </div>
      
    </div>

    <PosLogsModal 
      :is-open="isLogsModalOpen" 
      @close="isLogsModalOpen = false" 
    />
  </div>
</template>