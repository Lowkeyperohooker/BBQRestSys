<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { posService, type PosItem, type CartItem } from '../services/posService';

const availableItems = ref<PosItem[]>([]);
const cart = ref<CartItem[]>([]);
const currentStaffId = 1; 

// Loading State
const isLoadingData = ref(true);

async function loadItems() {
  isLoadingData.value = true;
  try {
    availableItems.value = await posService.getAvailablePosItems();
  } catch (error) {
    console.error("Failed to load POS items:", error);
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

// Master Checkout
async function handleCheckout() {
  if (cart.value.length === 0) return;

  try {
    const orderId = await posService.processCheckout(
      currentStaffId,
      cart.value,
      subtotal.value,
      tax.value,
      total.value
    );

    alert(`Payment Processed!\nOrder #${orderId} complete.\nInventory has been deducted.`);
    cart.value = [];
    await loadItems();

  } catch (error) {
    console.error("Checkout failed:", error);
    alert("Transaction failed. No inventory was deducted.");
  }
}

onMounted(() => {
  loadItems();
});
</script>

<template>
  <div class="h-full flex bg-gray-50 rounded-xl overflow-hidden shadow-sm border border-gray-100">
    
    <div class="flex-1 p-6 overflow-y-auto">
      <div class="mb-6 flex justify-between items-center">
        <div>
          <h2 class="text-2xl font-bold text-gray-800">Point of Sale</h2>
          <p class="text-sm text-gray-500">Select items to add to the current order.</p>
        </div>
        <button class="px-4 py-2 bg-gray-800 text-white rounded-lg font-semibold shadow hover:bg-gray-700 transition-colors">
          View Logs
        </button>
      </div>
      
      <div v-if="isLoadingData" class="flex flex-col items-center justify-center py-16">
        <svg class="w-10 h-10 animate-spin text-blue-500 mb-4" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <p class="text-gray-500 font-medium animate-pulse">Loading menu items...</p>
      </div>

      <div v-else class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
        
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
    </div>

    <div class="w-96 bg-white border-l border-gray-200 flex flex-col h-full z-10">
      
      <div class="p-4 border-b border-gray-100 bg-gray-50">
        <h3 class="text-lg font-bold text-gray-800">Current Order</h3>
        <p class="text-sm text-gray-500">Cashier ID: {{ currentStaffId }}</p>
      </div>
      
      <div class="flex-1 p-4 overflow-y-auto bg-white">
        <div v-if="cart.length === 0" class="h-full flex items-center justify-center text-gray-400">
          <p>Select items to begin</p>
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
        <button 
          @click="handleCheckout"
          :disabled="cart.length === 0"
          class="w-full bg-green-500 hover:bg-green-600 text-white py-4 rounded-xl font-bold text-lg transition-colors shadow-md disabled:bg-gray-300 disabled:cursor-not-allowed"
        >
          Process Payment
        </button>
      </div>
      
    </div>
  </div>
</template>