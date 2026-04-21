<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { posService, type PosItem, type CartItem, type ActiveOrder } from '../services/posService';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import PosLogsModal from '../components/ui/PosLogsModal.vue';
import { useResponsive } from '../composables/useResponsive';

const { fontSm, fontBase, fontLg, fontXl, isMobile, isTablet } = useResponsive();

const availableItems = ref<PosItem[]>([]);
const cart = ref<CartItem[]>([]);
const activeOrders = ref<ActiveOrder[]>([]);
const currentStaffId = 1;

const isLoadingData = ref(true);
const isLogsModalOpen = ref(false);
const viewMode = ref<'new' | 'active'>('new');

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

const subtotal = computed(() => {
  return cart.value.reduce((sum, item) => sum + (item.unit_price * item.qty), 0);
});
const tax = computed(() => subtotal.value * 0.0);
const total = computed(() => subtotal.value + tax.value);

const totalCartItems = computed(() => {
  return cart.value.reduce((sum, item) => sum + item.qty, 0);
});

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

    cart.value = [];
    customerName.value = '';
    orderType.value = 'Dine-in';

    await loadData();

  } catch (error) {
    console.error("Checkout failed:", error);
    alert("Transaction failed. No inventory was deducted.");
  }
}

async function handleUpdateStatus(order: ActiveOrder, newStatus: string) {
  try {
    await posService.updateOrderStatus(order.order_id, newStatus, currentStaffId);
    await loadData();
  } catch (error) {
    console.error("Failed to update status:", error);
    alert("An error occurred while updating the order status.");
  }
}

async function handleSettlePayment(order: ActiveOrder) {
  const confirmPayment = confirm(`Settle payment of PHP ${order.total_amount.toFixed(2)} for ${order.customer_identifier}?`);
  if (!confirmPayment) return;

  try {
    await posService.settlePayment(order.order_id, currentStaffId);
    alert(`Payment settled for ${order.customer_identifier}!`);
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
  <div :class="['h-full flex max-w-400 mx-auto', isMobile || isTablet ? 'flex-col gap-4' : 'flex-row gap-6']">

    <div class="flex-1 bg-white rounded-2xl overflow-hidden shadow-sm border border-gray-100 flex flex-col min-h-0 relative">
      <div class="h-full overflow-y-auto p-4 md:p-6 lg:p-8 bg-gray-50/30">

        <div class="sticky top-0 z-40 bg-white/95 backdrop-blur -mt-4 md:-mt-6 lg:-mt-8 -mx-4 md:-mx-6 lg:-mx-8 px-4 md:px-6 lg:px-8 pt-4 md:pt-6 lg:pt-8 pb-4 mb-8 border-b border-gray-200 flex flex-col lg:flex-row justify-between items-start lg:items-center gap-4">
          <div>
            <h2 :class="['font-extrabold text-gray-900 tracking-tight', fontXl]">Point of Sale</h2>
            <p :class="['font-medium text-gray-500 mt-1', fontSm]">Manage orders and customer tabs.</p>
          </div>

          <div :class="['flex items-center gap-4 w-full lg:w-auto', isMobile ? 'flex-col items-stretch' : 'flex-row']">
            <div class="flex bg-gray-100/80 rounded-xl p-1 border border-gray-200/50 w-full lg:w-auto">
              <button @click="viewMode = 'new'"
                :class="[viewMode === 'new' ? 'bg-white shadow-sm text-gray-900' : 'text-gray-500 hover:text-gray-700', 'flex-1 lg:flex-none px-5 py-2 font-semibold rounded-lg transition-all', fontSm]">
                Take Order
              </button>
              <button @click="viewMode = 'active'"
                :class="[viewMode === 'active' ? 'bg-white shadow-sm text-gray-900' : 'text-gray-500 hover:text-gray-700', 'flex-1 lg:flex-none px-5 py-2 font-semibold rounded-lg transition-all flex items-center justify-center gap-2', fontSm]">
                Active Tabs
                <span v-if="activeOrders.length > 0"
                  class="bg-orange-500 text-white text-[10px] font-bold px-1.5 py-0.5 rounded-full">
                  {{ activeOrders.length }}
                </span>
              </button>
            </div>

            <BaseButton variant="secondary" @click="isLogsModalOpen = true" :class="isMobile ? 'w-full py-2.5' : 'py-2.5'">
              View Logs
            </BaseButton>
          </div>
        </div>

        <div v-if="isLoadingData" class="flex-1 flex items-center justify-center h-64">
          <DataLoader message="Loading register data..." />
        </div>

        <div v-else-if="viewMode === 'new'" :class="['grid gap-4 md:gap-5', isMobile ? 'grid-cols-2' : 'grid-cols-2 lg:grid-cols-3 xl:grid-cols-4']">
          <div v-for="item in availableItems" :key="item.prep_item_id" @click="addToCart(item)"
            class="bg-white p-4 md:p-5 rounded-2xl shadow-sm border border-gray-100 cursor-pointer hover:border-blue-300 transition-all hover:shadow-md group relative flex flex-col h-full min-h-35">
            <span v-if="item.is_variable_price" class="absolute top-3 right-3 bg-orange-50 text-orange-600 text-[10px] font-bold px-2.5 py-1 rounded-full z-10">
              VAR
            </span>

            <div class="h-16 md:h-20 bg-blue-50/50 rounded-xl mb-3 md:mb-4 flex items-center justify-center text-blue-500 group-hover:scale-105 transition-transform duration-300">
              <svg class="w-6 h-6 md:w-8 md:h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"></path></svg>
            </div>
            <h4 :class="['font-bold text-gray-900 truncate mb-1', fontBase]" :title="item.pos_display_name">{{ item.pos_display_name }}</h4>
            <div class="flex justify-between items-end mt-auto pt-2">
              <div class="flex flex-col">
                <span v-if="item.is_variable_price" class="text-[9px] text-gray-400 font-medium uppercase tracking-wider mb-0.5">Starts at</span>
                <span :class="['text-blue-600 font-extrabold leading-none', fontLg]">₱{{ item.unit_price.toFixed(2) }}</span>
              </div>
              <p class="text-[10px] md:text-xs text-gray-400 font-medium">{{ item.current_stock_pieces }} left</p>
            </div>
          </div>

          <div v-if="availableItems.length === 0" class="col-span-full py-12 text-center text-gray-500">
            No items currently available. Please prepare skewers in the Prep Station first.
          </div>
        </div>

        <div v-else-if="viewMode === 'active'" class="flex-1">
          <div v-if="activeOrders.length === 0" class="flex flex-col items-center justify-center h-64 text-gray-400 space-y-4">
            <svg class="w-16 h-16 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path></svg>
            <p :class="['font-medium', fontLg]">No active tabs currently open.</p>
          </div>

          <div v-else class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-5">
            <div v-for="order in activeOrders" :key="order.order_id" class="bg-white border border-gray-200 rounded-2xl p-5 md:p-6 shadow-sm flex flex-col relative overflow-hidden">
              <div :class="order.status === 'Cooking' ? 'bg-orange-400' : 'bg-green-400'" class="absolute top-0 left-0 w-full h-1"></div>

              <div class="flex justify-between items-start mb-4">
                <span :class="['text-gray-500 font-semibold tracking-wide', fontSm]">Order #{{ order.order_id }}</span>
                <span :class="[order.status === 'Cooking' ? 'bg-orange-50 text-orange-600 animate-pulse' : 'bg-green-50 text-green-600', 'px-2.5 py-1 rounded-md font-bold', fontSm]">
                  {{ order.status }}
                </span>
              </div>

              <h3 :class="['font-extrabold text-gray-900 truncate mb-1', fontXl]" :title="order.customer_identifier">
                {{ order.customer_identifier }}
              </h3>
              <p :class="['font-medium text-gray-500 mb-6', fontSm]">{{ order.order_type }}</p>

              <div class="mt-auto flex flex-col gap-3 border-t border-gray-50 pt-4">
                <div class="flex justify-between items-center">
                  <span :class="['text-gray-500 font-medium', fontSm]">Total</span>
                  <span :class="['font-black text-gray-900', fontXl]">₱{{ order.total_amount.toFixed(2) }}</span>
                </div>
                <div class="flex gap-2 w-full mt-2">
                  <BaseButton v-if="order.status === 'Cooking'" variant="secondary" @click="handleUpdateStatus(order, 'Cooked')" :class="['flex-1 py-2', fontSm]">
                    Mark Cooked
                  </BaseButton>
                  <BaseButton variant="success" @click="handleSettlePayment(order)" :class="['flex-1 py-2', fontSm]">
                    Settle Payment
                  </BaseButton>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-show="viewMode === 'new'" :class="['bg-white border border-gray-100 rounded-2xl shadow-sm flex flex-col shrink-0', isMobile || isTablet ? 'h-96' : 'w-80 lg:w-96']">

      <div class="p-4 md:p-6 border-b border-gray-50 bg-gray-50/50 rounded-t-2xl">
        <h3 :class="['font-extrabold text-gray-900 mb-4', fontLg]">
          Order Details
          <span v-if="totalCartItems > 0" :class="['font-medium text-blue-500 ml-2', fontSm]">({{ totalCartItems }} items)</span>
        </h3>

        <input v-model="customerName" type="text" placeholder="Table # or Name"
          :class="['w-full border border-gray-200 rounded-xl px-4 py-3 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-4 bg-white', fontSm]" />
        <div class="flex bg-gray-100 rounded-xl p-1 w-full border border-gray-200/50">
          <button @click="orderType = 'Dine-in'"
            :class="[orderType === 'Dine-in' ? 'bg-white shadow-sm text-gray-900' : 'text-gray-500', 'flex-1 py-2 font-semibold rounded-lg transition-all', fontSm]">Dine-in</button>
          <button @click="orderType = 'Takeout'"
            :class="[orderType === 'Takeout' ? 'bg-white shadow-sm text-gray-900' : 'text-gray-500', 'flex-1 py-2 font-semibold rounded-lg transition-all', fontSm]">Takeout</button>
        </div>
      </div>

      <div class="flex-1 p-4 md:p-6 overflow-y-auto">
        <div v-if="cart.length === 0" class="h-full flex flex-col items-center justify-center text-gray-400">
          <svg class="w-12 h-12 mb-3 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
          <p :class="['font-medium', fontBase]">Cart is empty</p>
        </div>

        <div v-else class="space-y-4 md:space-y-5">
          <div v-for="(cartItem, index) in cart" :key="index" class="flex justify-between items-center group">
            <div class="flex-1 pr-2">
              <p :class="['font-bold text-gray-900 line-clamp-1', fontBase]">{{ cartItem.pos_display_name }}</p>
              <p :class="['font-medium text-gray-500', fontSm]">₱{{ cartItem.unit_price.toFixed(2) }} x {{ cartItem.qty }}</p>
            </div>
            <div class="flex items-center gap-3 md:gap-4">
              <span :class="['font-extrabold text-gray-900', fontBase]">₱{{ (cartItem.unit_price * cartItem.qty).toFixed(2) }}</span>
              <button @click="removeFromCart(index)" class="text-gray-300 hover:text-red-500 transition-colors p-1">
                <svg class="w-4 h-4 md:w-5 md:h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="p-4 md:p-6 bg-white border-t border-gray-100 rounded-b-2xl">
        <div :class="['flex justify-between mb-3 text-gray-500 font-medium', fontSm]">
          <span>Subtotal</span>
          <span>₱{{ subtotal.toFixed(2) }}</span>
        </div>
        <div :class="['flex justify-between mb-5 md:mb-6 font-black text-gray-900', fontXl]">
          <span>Total</span>
          <span>₱{{ total.toFixed(2) }}</span>
        </div>
        <BaseButton variant="primary" @click="handleSendToGrill" :disabled="cart.length === 0 || !customerName.trim()" class="w-full py-3 md:py-4 tracking-wide">
          <span :class="['font-bold', fontBase]">Send to Grill</span>
        </BaseButton>
        <p v-if="cart.length > 0 && !customerName.trim()" class="text-xs text-red-500 text-center mt-3 font-medium">
          * Please enter customer name/table
        </p>
      </div>

    </div>

    <PosLogsModal :is-open="isLogsModalOpen" @close="isLogsModalOpen = false" />
  </div>
</template>