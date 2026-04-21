<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { posService, type PosItem, type CartItem } from '../services/posService';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import { useResponsive } from '../composables/useResponsive';

const { fontSm, fontBase, fontLg, fontXl, font2xl, isMobile, isTablet } = useResponsive();

const availableItems = ref<PosItem[]>([]);
const cart = ref<CartItem[]>([]);
const isLoadingData = ref(true);

// Modal state
const isCartModalOpen = ref(false);

const customerName = ref('');
const orderType = ref<'Dine-in' | 'Takeout'>('Dine-in');

// Defaulting to System Admin (1) for self-service orders. 
// In the future, you can create a dedicated "Kiosk" staff ID in your database.
const kioskStaffId = 1; 

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

const subtotal = computed(() => cart.value.reduce((sum, item) => sum + (item.unit_price * item.qty), 0));
const tax = computed(() => subtotal.value * 0.0);
const total = computed(() => subtotal.value + tax.value);
const totalCartItems = computed(() => cart.value.reduce((sum, item) => sum + item.qty, 0));

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

async function handlePlaceOrder() {
  if (cart.value.length === 0) return;
  if (!customerName.value.trim()) {
    alert("Please enter your name or table number so we can serve you.");
    return;
  }

  try {
    const orderId = await posService.sendToGrill(
      kioskStaffId,
      customerName.value,
      orderType.value,
      cart.value,
      subtotal.value,
      tax.value,
      total.value
    );

    alert(`Success! Your order (#${orderId}) has been sent to the grill!`);

    cart.value = [];
    customerName.value = '';
    orderType.value = 'Dine-in';
    isCartModalOpen.value = false;

    await loadMenu();

  } catch (error) {
    console.error("Order failed:", error);
    alert("Failed to place the order. Please try again or ask a staff member.");
  }
}

onMounted(() => {
  loadMenu();
});
</script>

<template>
  <div class="h-full flex flex-col relative max-w-400 mx-auto">

    <div class="flex-1 bg-white rounded-2xl overflow-hidden shadow-sm border border-gray-100 flex flex-col min-h-0 relative">
      <div class="h-full overflow-y-auto p-4 md:p-6 lg:p-8 bg-gray-50/30 pb-32">

        <div class="sticky top-0 z-40 bg-white/95 backdrop-blur -mt-4 md:-mt-6 lg:-mt-8 -mx-4 md:-mx-6 lg:-mx-8 px-4 md:px-6 lg:px-8 pt-4 md:pt-6 lg:pt-8 pb-4 mb-8 border-b border-gray-200 flex justify-between items-center">
          <div>
            <h2 :class="['font-extrabold text-gray-900 tracking-tight', font2xl]">Self-Service Menu</h2>
            <p :class="['font-medium text-gray-500 mt-1', fontSm]">Tap items to add them to your order</p>
          </div>
          <div class="w-12 h-12 bg-blue-600 rounded-xl shadow-lg flex items-center justify-center text-white">
            <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"></path></svg>
          </div>
        </div>

        <div v-if="isLoadingData" class="flex-1 flex items-center justify-center h-64">
          <DataLoader message="Loading fresh menu..." />
        </div>

        <div v-else :class="['grid gap-4 md:gap-5', isMobile ? 'grid-cols-2' : 'grid-cols-3 lg:grid-cols-4 xl:grid-cols-5']">
          <div v-for="item in availableItems" :key="item.prep_item_id" @click="addToCart(item)"
            class="bg-white p-4 md:p-5 rounded-2xl shadow-sm border border-gray-100 cursor-pointer hover:border-blue-500 hover:shadow-lg hover:-translate-y-1 transition-all group relative flex flex-col h-full min-h-40">
            
            <span v-if="item.is_variable_price" class="absolute top-3 right-3 bg-orange-50 text-orange-600 text-[10px] font-bold px-2.5 py-1 rounded-full z-10">
              VAR
            </span>

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

          <div v-if="availableItems.length === 0" class="col-span-full py-12 text-center text-gray-500">
            Sorry, we are currently sold out of all items. Please check back later!
          </div>
        </div>
      </div>
    </div>

    <div v-if="totalCartItems > 0" class="fixed bottom-6 right-6 md:bottom-8 md:right-8 z-50">
      <BaseButton @click="isCartModalOpen = true" variant="primary" class="py-4 shadow-2xl flex items-center gap-4 px-6 rounded-full hover:scale-105 transition-transform">
        <div class="flex items-center justify-center bg-white/20 w-8 h-8 rounded-full text-sm font-bold">
          {{ totalCartItems }}
        </div>
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
          <input 
            v-model="customerName" 
            type="text" 
            placeholder="Enter Your Name or Table #"
            :class="['w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-bold focus:outline-none focus:border-blue-500 mb-4 bg-white transition-colors', fontBase]" 
          />
          
          <div class="flex bg-gray-200/50 rounded-xl p-1.5 w-full mb-4">
            <button @click="orderType = 'Dine-in'"
              :class="[orderType === 'Dine-in' ? 'bg-white shadow text-gray-900' : 'text-gray-500', 'flex-1 py-2 font-bold rounded-lg transition-all', fontBase]">Dine-in</button>
            <button @click="orderType = 'Takeout'"
              :class="[orderType === 'Takeout' ? 'bg-white shadow text-gray-900' : 'text-gray-500', 'flex-1 py-2 font-bold rounded-lg transition-all', fontBase]">Takeout</button>
          </div>

          <div :class="['flex justify-between mb-2 text-gray-500 font-medium', fontSm]">
            <span>Subtotal</span>
            <span>₱{{ subtotal.toFixed(2) }}</span>
          </div>
          <div :class="['flex justify-between mb-5 font-black text-gray-900', font2xl]">
            <span>Total Due</span>
            <span class="text-blue-600">₱{{ total.toFixed(2) }}</span>
          </div>
          
          <BaseButton 
            variant="primary" 
            @click="handlePlaceOrder" 
            :disabled="cart.length === 0 || !customerName.trim()" 
            class="w-full py-4 shadow-lg hover:shadow-xl transition-all rounded-xl"
          >
            <span :class="['font-black uppercase tracking-wider', fontLg]">Finalize Order</span>
          </BaseButton>
          
          <p v-if="cart.length > 0 && !customerName.trim()" class="text-sm text-red-500 text-center mt-3 font-bold animate-pulse">
            * Please enter your name/table above
          </p>
        </div>

      </div>
    </div>

  </div>
</template>