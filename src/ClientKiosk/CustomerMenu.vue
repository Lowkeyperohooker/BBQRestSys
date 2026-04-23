<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { posService, type PosItem, type CartItem } from '../services/posService';
import { queueService } from '../services/queueService'; // NEW: Import the queue service
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import { useResponsive } from '../composables/useResponsive';

const { fontSm, fontBase, fontLg, fontXl, font2xl, isMobile } = useResponsive();

// --- State ---
const availableItems = ref<PosItem[]>([]);
const cart = ref<CartItem[]>([]);
const isLoadingData = ref(true);

// Flow Control
const hasSelectedOrderType = ref(false);
const orderType = ref<'Dine-in' | 'Takeout' | null>(null);
const generatedQueueNumber = ref<number | null>(null);

// Modal state
const isCartModalOpen = ref(false);

// --- Initialization ---
async function loadMenu() {
  isLoadingData.value = true;
  try {
    availableItems.value = await posService.getAvailablePosItems();
  } catch (error) {
    console.error("Failed to load menu items:", error);
  } finally {
    setTimeout(() => { isLoadingData.value = false; }, 600);
  }
}

onMounted(() => {
  loadMenu();
});

// --- Computed Properties ---
const subtotal = computed(() => cart.value.reduce((sum, item) => sum + (item.unit_price * item.qty), 0));
const tax = computed(() => subtotal.value * 0.0);
const total = computed(() => subtotal.value + tax.value);
const totalCartItems = computed(() => cart.value.reduce((sum, item) => sum + item.qty, 0));

// --- Actions ---
function selectOrderType(type: 'Dine-in' | 'Takeout') {
  orderType.value = type;
  hasSelectedOrderType.value = true;
}

function addToCart(item: PosItem) {
  let finalPrice = item.unit_price;

  if (item.is_variable_price) {
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
    alert(`Sorry, only ${item.current_stock_pieces} left in stock.`);
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
  if (cart.value.length === 0) {
    isCartModalOpen.value = false;
  }
}

function resetKiosk() {
  cart.value = [];
  hasSelectedOrderType.value = false;
  orderType.value = null;
  generatedQueueNumber.value = null;
  isCartModalOpen.value = false;
}

async function handleFinalizeOrder() {
  if (cart.value.length === 0 || !orderType.value) return;

  try {
    // 1. Get the next reliable queue number (1000 -> 9090) from the service
    const queueNum = await queueService.getNextQueueNumber();

    // 2. Prepare the new order payload
    const pendingOrder = {
      queue_number: queueNum,
      order_type: orderType.value,
      cart_items: cart.value,
      subtotal: subtotal.value,
      tax: tax.value,
      total: total.value,
      timestamp: new Date().toISOString()
    };

    // 3. Send to the Rust Backend to save in the JSON file
    await queueService.addToQueue(pendingOrder);

    // 4. Display the generated number to the customer
    generatedQueueNumber.value = queueNum;

  } catch (error) {
    console.error("Failed to save pending order:", error);
    alert("Failed to process your order. Please call a staff member.");
  }
}
</script>

<template>
  <div class="h-full flex flex-col relative max-w-400 mx-auto">

    <div v-if="!hasSelectedOrderType" class="flex-1 flex flex-col items-center justify-center p-6 bg-white rounded-2xl shadow-sm border border-gray-100">
      <div class="w-24 h-24 bg-blue-600 rounded-3xl shadow-xl flex items-center justify-center text-white mb-8">
        <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path></svg>
      </div>
      <h1 :class="['font-black text-gray-900 mb-2', font2xl]">Welcome to BBQRestSys</h1>
      <p :class="['text-gray-500 font-medium mb-12', fontLg]">How would you like your order today?</p>
      
      <div class="flex flex-col sm:flex-row gap-6 w-full max-w-2xl">
        <button @click="selectOrderType('Dine-in')" class="flex-1 bg-gray-50 hover:bg-blue-50 border-2 border-gray-200 hover:border-blue-500 rounded-3xl p-8 flex flex-col items-center gap-4 transition-all hover:shadow-lg group">
          <svg class="w-16 h-16 text-gray-400 group-hover:text-blue-600 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 15.546c-.523 0-1.046.151-1.5.454a2.704 2.704 0 01-3 0 2.704 2.704 0 00-3 0 2.704 2.704 0 01-3 0 2.704 2.704 0 00-3 0 2.704 2.704 0 01-3 0 2.701 2.701 0 00-1.5-.454M9 6v2m3-2v2m3-2v2M9 3h.01M12 3h.01M15 3h.01M6.75 21A3.75 3.75 0 013 17.25V14.16M21 14.16v3.09a3.75 3.75 0 01-3.75 3.75M6.75 21H17.25"></path></svg>
          <span :class="['font-bold text-gray-900 group-hover:text-blue-700', fontXl]">Dine-In</span>
        </button>
        <button @click="selectOrderType('Takeout')" class="flex-1 bg-gray-50 hover:bg-blue-50 border-2 border-gray-200 hover:border-blue-500 rounded-3xl p-8 flex flex-col items-center gap-4 transition-all hover:shadow-lg group">
          <svg class="w-16 h-16 text-gray-400 group-hover:text-blue-600 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"></path></svg>
          <span :class="['font-bold text-gray-900 group-hover:text-blue-700', fontXl]">Takeout</span>
        </button>
      </div>
    </div>

    <div v-else-if="!generatedQueueNumber" class="flex-1 bg-white rounded-2xl overflow-hidden shadow-sm border border-gray-100 flex flex-col min-h-0 relative">
      <div class="h-full overflow-y-auto p-4 md:p-6 lg:p-8 bg-gray-50/30 pb-32">

        <div class="sticky top-0 z-40 bg-white/95 backdrop-blur -mt-4 md:-mt-6 lg:-mt-8 -mx-4 md:-mx-6 lg:-mx-8 px-4 md:px-6 lg:px-8 pt-4 md:pt-6 lg:pt-8 pb-4 mb-8 border-b border-gray-200 flex justify-between items-center">
          <div>
            <div class="flex items-center gap-3 mb-1">
              <span class="bg-blue-100 text-blue-800 text-xs font-bold px-2.5 py-0.5 rounded-full uppercase tracking-wider">{{ orderType }}</span>
              <button @click="resetKiosk" class="text-gray-400 hover:text-red-500 text-xs font-bold underline">Change</button>
            </div>
            <h2 :class="['font-extrabold text-gray-900 tracking-tight', font2xl]">Self-Service Menu</h2>
            <p :class="['font-medium text-gray-500 mt-1', fontSm]">Tap items to add them to your order</p>
          </div>
        </div>

        <div v-if="isLoadingData" class="flex-1 flex items-center justify-center h-64">
          <DataLoader message="Loading fresh menu..." />
        </div>

        <div v-else :class="['grid gap-4 md:gap-5', isMobile ? 'grid-cols-2' : 'grid-cols-3 lg:grid-cols-4 xl:grid-cols-5']">
          <div v-for="item in availableItems" :key="item.prep_item_id" @click="addToCart(item)"
            class="bg-white p-4 md:p-5 rounded-2xl shadow-sm border border-gray-100 cursor-pointer hover:border-blue-500 hover:shadow-lg hover:-translate-y-1 transition-all group relative flex flex-col h-full min-h-40">
            
            <span v-if="item.is_variable_price" class="absolute top-3 right-3 bg-orange-50 text-orange-600 text-[10px] font-bold px-2.5 py-1 rounded-full z-10">VAR</span>

            <div class="h-16 md:h-24 bg-blue-50/50 rounded-xl mb-3 md:mb-4 flex items-center justify-center text-blue-500 group-hover:bg-blue-100 transition-colors duration-300">
              <svg class="w-8 h-8 md:w-10 md:h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
            </div>
            
            <h4 :class="['font-bold text-gray-900 mb-1', fontLg]" :title="item.pos_display_name">{{ item.pos_display_name }}</h4>
            
            <div class="flex justify-between items-end mt-auto pt-2">
              <div class="flex flex-col">
                <span v-if="item.is_variable_price" class="text-[9px] text-gray-400 font-medium uppercase tracking-wider mb-0.5">Starts at</span>
                <span :class="['text-blue-600 font-black leading-none', fontXl]">₱{{ item.unit_price.toFixed(2) }}</span>
              </div>
              <p v-if="item.current_stock_pieces < 10" class="text-xs text-red-500 font-bold">Only {{ item.current_stock_pieces }} left!</p>
              <p v-else class="text-[10px] md:text-xs text-gray-400 font-medium">Available</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="flex-1 flex flex-col items-center justify-center p-6 bg-white rounded-2xl shadow-sm border border-gray-100 text-center">
      <div class="w-24 h-24 bg-green-100 rounded-full flex items-center justify-center text-green-600 mb-8">
        <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path></svg>
      </div>
      <h2 :class="['font-black text-gray-900 mb-2', font2xl]">Order Placed!</h2>
      <p :class="['text-gray-500 font-medium mb-8', fontLg]">Please proceed to the cashier and present this number:</p>
      
      <div class="bg-gray-100 border-2 border-dashed border-gray-300 rounded-3xl px-16 py-8 mb-12">
        <span class="text-7xl font-black text-blue-600 tracking-widest">{{ generatedQueueNumber }}</span>
      </div>

      <BaseButton @click="resetKiosk" variant="primary" class="py-4 px-12 shadow-lg hover:shadow-xl rounded-full">
        <span :class="['font-bold', fontLg]">Done</span>
      </BaseButton>
    </div>

    <div v-if="totalCartItems > 0 && hasSelectedOrderType && !generatedQueueNumber" class="fixed bottom-6 right-6 md:bottom-8 md:right-8 z-50">
      <BaseButton @click="isCartModalOpen = true" variant="primary" class="py-4 shadow-2xl flex items-center gap-4 px-6 rounded-full hover:scale-105 transition-transform">
        <div class="flex items-center justify-center bg-white/20 w-8 h-8 rounded-full text-sm font-bold">{{ totalCartItems }}</div>
        <span :class="['font-black uppercase tracking-wider', fontLg]">Check Order</span>
        <span :class="['font-black ml-2', fontLg]">₱{{ total.toFixed(2) }}</span>
      </BaseButton>
    </div>

    <div v-if="isCartModalOpen" class="fixed inset-0 z-100 flex items-end justify-end p-4 md:p-8 bg-black/40 backdrop-blur-sm" @click.self="isCartModalOpen = false">
      <div class="bg-white w-full max-w-md rounded-3xl shadow-2xl flex flex-col max-h-[85vh] overflow-hidden animate-in slide-in-from-bottom-8 duration-300 origin-bottom-right">
        
        <div class="p-5 border-b border-gray-100 bg-gray-50 flex justify-between items-center shrink-0">
          <h3 :class="['font-extrabold text-gray-900', fontXl]">Review Order</h3>
          <button @click="isCartModalOpen = false" class="text-gray-400 hover:text-gray-600 bg-gray-200/50 hover:bg-gray-200 p-2 rounded-full transition-colors">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
          </button>
        </div>

        <div class="flex-1 overflow-y-auto p-5">
          <div class="space-y-4">
            <div v-for="(cartItem, index) in cart" :key="index" class="flex justify-between items-center group">
              <div class="flex-1 pr-2">
                <p :class="['font-bold text-gray-900', fontBase]">{{ cartItem.pos_display_name }}</p>
                <p :class="['font-medium text-gray-500 mt-0.5', fontSm]">₱{{ cartItem.unit_price.toFixed(2) }} x {{ cartItem.qty }}</p>
              </div>
              <div class="flex items-center gap-3">
                <span :class="['font-black text-gray-900', fontLg]">₱{{ (cartItem.unit_price * cartItem.qty).toFixed(2) }}</span>
                <button @click="removeFromCart(index)" class="text-gray-400 hover:text-red-500 bg-gray-50 hover:bg-red-50 rounded-lg p-2 transition-all">
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
                </button>
              </div>
            </div>
          </div>
        </div>

        <div class="p-5 bg-gray-50 border-t border-gray-100 shrink-0">
          <div :class="['flex justify-between mb-2 text-gray-500 font-medium', fontSm]">
            <span>Subtotal</span><span>₱{{ subtotal.toFixed(2) }}</span>
          </div>
          <div :class="['flex justify-between mb-5 font-black text-gray-900', font2xl]">
            <span>Total Due</span><span class="text-blue-600">₱{{ total.toFixed(2) }}</span>
          </div>
          
          <BaseButton variant="primary" @click="handleFinalizeOrder" class="w-full py-4 shadow-lg hover:shadow-xl transition-all rounded-xl">
            <span :class="['font-black uppercase tracking-wider', fontLg]">Finalize Order</span>
          </BaseButton>
        </div>

      </div>
    </div>
  </div>
</template>