<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { posService, type PosItem, type CartItem } from '../services/posService';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';

const availableItems = ref<PosItem[]>([]);
const cart = ref<CartItem[]>([]);
const isLoading = ref(true);

// Kiosk State
const customerName = ref('');
const orderType = ref<'Dine-in' | 'Takeout'>('Dine-in');
const generatedPin = ref<string | null>(null);

// For kiosk, we don't know the exact staff ID, so we use a generic "Kiosk/Waiter" ID
const KIOSK_STAFF_ID = 2; 

async function loadMenu() {
  isLoading.value = true;
  try {
    availableItems.value = await posService.getAvailablePosItems();
  } catch (error) {
    console.error("Failed to load menu:", error);
  } finally {
    setTimeout(() => isLoading.value = false, 500);
  }
}

const subtotal = computed(() => cart.value.reduce((sum, item) => sum + (item.unit_price * item.qty), 0));
const totalCartItems = computed(() => cart.value.reduce((sum, item) => sum + item.qty, 0));

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

async function submitOrder() {
  if (cart.value.length === 0 || !customerName.value.trim()) return;

  try {
    const pin = await posService.holdOrderWithPin(
      KIOSK_STAFF_ID, 
      customerName.value, 
      orderType.value, 
      cart.value, 
      subtotal.value, 
      0, // Tax
      subtotal.value // Total
    );
    
    generatedPin.value = pin;
    
  } catch (error) {
    console.error("Order submission failed:", error);
    alert("Failed to submit order.");
  }
}

function startNewOrder() {
  generatedPin.value = null;
  cart.value = [];
  customerName.value = '';
  loadMenu(); // Refresh to get latest stock
}

onMounted(() => loadMenu());
</script>

<template>
  <div class="h-screen w-full bg-gray-50 flex flex-col font-sans relative overflow-hidden">
    
    <header class="bg-gray-900 text-white p-4 shadow-md flex justify-between items-center z-10 shrink-0">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 bg-blue-600 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"></path></svg>
        </div>
        <div>
          <h1 class="font-black text-xl tracking-tight leading-none">BBQ<span class="text-blue-400">SYS</span></h1>
          <p class="text-xs text-gray-400 font-bold tracking-widest uppercase">Tableside Kiosk</p>
        </div>
      </div>
    </header>

    <div class="flex-1 flex flex-col md:flex-row overflow-hidden">
      
      <div class="flex-1 p-4 md:p-6 overflow-y-auto bg-gray-50 pb-32 md:pb-6">
        <h2 class="text-2xl font-extrabold text-gray-900 mb-6">Select Items</h2>
        
        <DataLoader v-if="isLoading" message="Loading Menu..." />
        
        <div v-else class="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          <button 
            v-for="item in availableItems" 
            :key="item.prep_item_id"
            @click="addToCart(item)"
            class="bg-white p-4 rounded-2xl shadow-sm border border-gray-200 text-left hover:border-blue-500 hover:shadow-md transition-all active:scale-95 flex flex-col relative"
          >
            <span v-if="item.is_variable_price === 1" class="absolute top-2 right-2 bg-orange-100 text-orange-700 text-[10px] font-bold px-2 py-0.5 rounded-full">Variable</span>
            <div class="w-full h-16 bg-blue-50 rounded-xl mb-3 flex items-center justify-center text-blue-500">
               <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"></path></svg>
            </div>
            <h4 class="font-bold text-gray-900 leading-tight mb-1">{{ item.pos_display_name }}</h4>
            <span class="text-blue-600 font-black mt-auto">₱{{ item.unit_price.toFixed(2) }}</span>
          </button>
        </div>
      </div>

      <div class="w-full md:w-96 bg-white border-t md:border-t-0 md:border-l border-gray-200 flex flex-col shrink-0 shadow-[0_-4px_6px_-1px_rgba(0,0,0,0.1)] md:shadow-none absolute bottom-0 left-0 md:relative max-h-[60vh] md:max-h-full">
        
        <div class="p-4 md:p-6 border-b border-gray-100 bg-white z-10 shrink-0">
          <input 
            v-model="customerName" 
            type="text" 
            placeholder="Enter Table # or Customer Name" 
            class="w-full bg-gray-100 border-none rounded-xl px-4 py-3 text-base font-medium focus:ring-2 focus:ring-blue-500 outline-none mb-3"
          />
          <div class="flex bg-gray-100 rounded-xl p-1">
            <button @click="orderType = 'Dine-in'" :class="orderType === 'Dine-in' ? 'bg-white shadow text-gray-900' : 'text-gray-500'" class="flex-1 py-2 text-sm font-bold rounded-lg transition-all">Dine-in</button>
            <button @click="orderType = 'Takeout'" :class="orderType === 'Takeout' ? 'bg-white shadow text-gray-900' : 'text-gray-500'" class="flex-1 py-2 text-sm font-bold rounded-lg transition-all">Takeout</button>
          </div>
        </div>

        <div class="flex-1 overflow-y-auto p-4 md:p-6 bg-gray-50/50">
          <div v-if="cart.length === 0" class="h-full flex flex-col items-center justify-center text-gray-400 py-8">
            <p class="font-medium text-center">Tap items to add to order</p>
          </div>
          
          <div v-else class="space-y-3">
            <div v-for="(cartItem, index) in cart" :key="index" class="bg-white p-3 rounded-xl border border-gray-100 shadow-sm flex justify-between items-center">
              <div>
                <p class="font-bold text-gray-900 leading-tight">{{ cartItem.pos_display_name }}</p>
                <p class="text-xs text-gray-500 font-medium">₱{{ cartItem.unit_price.toFixed(2) }} x {{ cartItem.qty }}</p>
              </div>
              <div class="flex items-center gap-3">
                <span class="font-black text-gray-900">₱{{ (cartItem.unit_price * cartItem.qty).toFixed(2) }}</span>
                <button @click="removeFromCart(index)" class="w-8 h-8 bg-red-50 text-red-500 rounded-lg flex items-center justify-center hover:bg-red-100 active:scale-95 transition-all">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                </button>
              </div>
            </div>
          </div>
        </div>

        <div class="p-4 md:p-6 bg-white border-t border-gray-100 shrink-0 pb-6 md:pb-6">
          <div class="flex justify-between items-end mb-4">
            <span class="text-gray-500 font-bold">Total ({{ totalCartItems }} items)</span>
            <span class="text-2xl font-black text-blue-600">₱{{ subtotal.toFixed(2) }}</span>
          </div>
          <BaseButton 
            variant="primary" 
            @click="submitOrder" 
            :disabled="cart.length === 0 || !customerName.trim()"
            class="w-full py-4 text-lg font-black tracking-wide rounded-xl shadow-blue-500/30 shadow-lg"
          >
            Submit Order
          </BaseButton>
        </div>
      </div>
    </div>

    <div v-if="generatedPin" class="absolute inset-0 bg-gray-900/95 z-50 flex flex-col items-center justify-center text-center p-6 backdrop-blur-md animate-in fade-in duration-200">
      <div class="w-20 h-20 bg-green-500 rounded-full flex items-center justify-center text-white mb-6 animate-bounce">
        <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path></svg>
      </div>
      <h2 class="text-3xl font-bold text-white mb-2">Order Saved!</h2>
      <p class="text-gray-400 text-lg mb-8">Please provide this PIN to the Cashier.</p>
      
      <div class="bg-white px-12 py-6 rounded-3xl shadow-2xl mb-12">
        <p class="text-[5rem] font-black text-gray-900 tracking-widest leading-none font-mono">{{ generatedPin }}</p>
      </div>
      
      <BaseButton variant="secondary" @click="startNewOrder" class="px-12 py-4 text-lg rounded-xl">
        Start New Order
      </BaseButton>
    </div>

  </div>
</template>