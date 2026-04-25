<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import BaseButton from './BaseButton.vue';
import { useResponsive } from '../../composables/useResponsive';
import type { ActiveOrder, PosItem, CartItem } from '../../services/posService';
import type { PendingOrder } from '../../services/queueService';

const { fontSm, fontBase, fontLg } = useResponsive();

const props = defineProps<{
  selectedOrder: ActiveOrder | null;
  selectedPending: PendingOrder | null;
  availableItems: PosItem[];
  searchQueueInput: string;
  isSearching: boolean;
  tableNumberInput: string;
}>();

const emit = defineEmits<{
  (e: 'update:searchQueueInput', val: string): void;
  (e: 'update:tableNumberInput', val: string): void;
  (e: 'search'): void;
  (e: 'clear-pending'): void;
  (e: 'reject-pending'): void;
  (e: 'accept-pending'): void;
  (e: 'update-status', status: string): void;
  (e: 'settle-payment'): void;
  (e: 'save-pending-edit', updatedData: PendingOrder): void;
}>();

// Edit State (Restricted to Pending Orders Only)
const isEditing = ref(false);
const editCart = ref<CartItem[]>([]);
const newItemSelection = ref<number | ''>('');
const newItemQty = ref<number>(1);

watch(() => props.selectedOrder, () => { isEditing.value = false; });
watch(() => props.selectedPending, () => { isEditing.value = false; });

function validateTableInput(e: Event) {
  const target = e.target as HTMLInputElement;
  let val = target.value.replace(/\D/g, '');
  if (val !== '') {
    let num = parseInt(val, 10);
    if (num > 100) val = '100';
  }
  target.value = val;
  emit('update:tableNumberInput', val);
}

function startEdit() {
  if (props.selectedPending) {
    editCart.value = JSON.parse(JSON.stringify(props.selectedPending.cart_items || []));
    isEditing.value = true;
  }
}

function addEditItem() {
  if (!newItemSelection.value || newItemQty.value <= 0) return;
  const item = props.availableItems.find(i => i.prep_item_id === newItemSelection.value);
  if (item) {
    const existing = editCart.value.find(i => i.prep_item_id === item.prep_item_id);
    if (existing) {
      existing.qty += newItemQty.value;
    } else {
      editCart.value.push({ ...item, qty: newItemQty.value } as CartItem);
    }
  }
  newItemSelection.value = '';
  newItemQty.value = 1;
}

function incrementEditQty(idx: number) {
  editCart.value[idx].qty++;
}

function decrementEditQty(idx: number) {
  if (editCart.value[idx].qty > 1) {
    editCart.value[idx].qty--;
  } else {
    // If quantity reaches 0, remove the item from the cart
    editCart.value.splice(idx, 1);
  }
}

const editTotal = computed(() => editCart.value.reduce((acc, item) => acc + (item.unit_price * item.qty), 0));

function saveEdit() {
  if (props.selectedPending) {
    emit('save-pending-edit', { 
      ...props.selectedPending, 
      cart_items: editCart.value, 
      total: editTotal.value, 
      subtotal: editTotal.value 
    });
  }
  isEditing.value = false;
}
</script>

<template>
  <div class="flex-1 bg-white rounded-2xl overflow-hidden shadow-sm border border-gray-100 flex flex-col min-h-0 relative">
    
    <div class="sticky top-0 z-40 bg-white/95 backdrop-blur px-5 pt-5 pb-3 border-b border-gray-200 flex flex-col lg:flex-row justify-between items-start lg:items-center gap-3 shrink-0">
      <div>
        <h2 :class="['font-extrabold text-gray-900 tracking-tight', fontLg]">Order Details</h2>
        <p :class="['font-medium text-gray-500', fontSm]">Manage and fulfill selected tab</p>
      </div>
      
      <div class="flex items-center gap-2 w-full lg:w-auto">
        <div class="relative flex-1 lg:w-40">
          <input 
            :value="searchQueueInput" 
            @input="emit('update:searchQueueInput', ($event.target as HTMLInputElement).value)"
            @keyup.enter="emit('search')"
            type="number" 
            placeholder="Queue #" 
            :class="['w-full border-2 border-gray-200 rounded-xl pl-9 pr-3 py-1.5 font-bold focus:outline-none focus:border-blue-500 transition-colors', fontSm]"
          />
          <svg class="w-4 h-4 text-gray-400 absolute left-3 top-1/2 -translate-y-1/2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
        </div>
        <BaseButton variant="primary" @click="emit('search')" :disabled="!searchQueueInput || isSearching" class="py-1.5 px-3">
          <span :class="fontSm">{{ isSearching ? '...' : 'Search' }}</span>
        </BaseButton>
      </div>
    </div>

    <div class="flex-1 p-4 bg-gray-50/30 flex flex-col min-h-0">
      
      <div v-if="!selectedOrder && !selectedPending" class="h-full flex flex-col items-center justify-center text-gray-400 space-y-3 m-auto">
        <svg class="w-12 h-12 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path></svg>
        <p :class="['font-medium text-center', fontBase]">Select an active tab from the right,<br/>or search a Customer Queue # above.</p>
      </div>

      <div v-else-if="selectedPending" class="h-full max-w-2xl w-full mx-auto flex flex-col gap-3 animate-in fade-in duration-300">
        <div class="bg-white border border-gray-200 rounded-xl p-4 shadow-sm relative flex flex-col flex-1 min-h-0">
          <div class="bg-blue-400 absolute top-0 left-0 w-full h-1 animate-pulse"></div>
          
          <div class="flex justify-between items-start mb-3 shrink-0">
            <div>
              <span class="text-blue-500 font-bold tracking-wide uppercase text-[10px]">Kiosk Order Ready</span>
              <h3 :class="['font-black text-gray-900 mt-0.5', fontLg]">Queue #{{ selectedPending.queue_number }}</h3>
              <p :class="['font-medium text-gray-500', fontSm]">{{ selectedPending.order_type }}</p>
            </div>
            <button v-if="!isEditing" @click="emit('clear-pending')" class="text-gray-400 hover:bg-gray-100 p-1 rounded-full transition-colors">
               <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
            </button>
          </div>

          <div class="flex-1 overflow-y-auto border-t border-b border-gray-100 py-2 my-2 pr-2">
            <div class="flex justify-between items-center mb-2 sticky top-0 bg-white z-10 py-1">
              <h4 class="font-bold text-gray-800 text-xs uppercase tracking-wide">Customer Cart</h4>
              <button v-if="!isEditing" @click="startEdit" class="text-blue-600 text-xs font-bold underline">Edit Order</button>
            </div>
            
            <div v-if="!isEditing" class="space-y-1.5">
              <div v-for="(item, idx) in selectedPending.cart_items" :key="idx" class="flex justify-between items-center bg-gray-50 py-1.5 px-2 rounded border border-gray-100">
                <p :class="['font-bold text-gray-800', fontSm]">{{ item.qty }}x {{ item.pos_display_name }}</p>
                <span :class="['font-bold text-gray-600', fontSm]">₱{{ (item.unit_price * item.qty).toFixed(2) }}</span>
              </div>
            </div>

            <div v-else class="space-y-3">
              <div v-for="(item, idx) in editCart" :key="idx" class="flex justify-between items-center bg-white p-2 border border-gray-200 rounded">
                <span class="font-bold text-sm text-gray-800">{{ item.pos_display_name }}</span>
                <div class="flex items-center gap-4">
                  <span class="font-bold text-gray-600 text-sm">₱{{ (item.unit_price * item.qty).toFixed(2) }}</span>
                  <div class="flex items-center border border-gray-300 rounded shadow-sm">
                    <button type="button" @click="decrementEditQty(idx)" class="px-2.5 py-1 text-gray-600 hover:bg-gray-100 font-black transition-colors">-</button>
                    <span class="px-2 text-sm font-bold w-8 text-center border-l border-r border-gray-300">{{ item.qty }}</span>
                    <button type="button" @click="incrementEditQty(idx)" class="px-2.5 py-1 text-gray-600 hover:bg-gray-100 font-black transition-colors">+</button>
                  </div>
                </div>
              </div>
              <div class="flex gap-2 mt-2 bg-gray-50 p-2 rounded border border-gray-200 items-center">
                <select v-model="newItemSelection" class="flex-1 border border-gray-300 rounded px-2 py-1.5 text-sm font-bold bg-white focus:outline-none focus:border-blue-500">
                  <option value="" disabled>Select Item</option>
                  <option v-for="ai in availableItems" :key="ai.prep_item_id" :value="ai.prep_item_id">{{ ai.pos_display_name }} (₱{{ ai.unit_price }})</option>
                </select>
                <input v-model.number="newItemQty" type="number" min="1" class="w-16 border border-gray-300 rounded px-2 py-1.5 text-sm font-bold text-center focus:outline-none focus:border-blue-500">
                <BaseButton variant="primary" @click="addEditItem" type="button" class="px-4 py-1.5 text-sm">Add</BaseButton>
              </div>
            </div>
          </div>

          <div class="pt-2 flex justify-between items-center shrink-0">
            <span :class="['font-bold text-gray-800', fontSm]">Total Value</span>
            <span :class="['font-black text-gray-900', fontLg]">₱{{ isEditing ? editTotal.toFixed(2) : selectedPending.total.toFixed(2) }}</span>
          </div>
        </div>

        <div class="bg-gray-50 p-3 rounded-xl border border-gray-200 shrink-0">
          <label :class="['block font-bold text-gray-700 mb-1.5', fontSm]">Assign Table Number (1-100)</label>
          <input 
            :value="tableNumberInput" 
            @input="validateTableInput"
            type="text" 
            pattern="[0-9]*"
            placeholder="e.g., 5" 
            :class="['w-full border-2 border-gray-300 rounded-lg px-3 py-2 font-bold focus:outline-none focus:border-blue-500 transition-colors', fontBase]"
          />
        </div>

        <div class="flex flex-col sm:flex-row gap-2 shrink-0">
          <template v-if="isEditing">
            <BaseButton variant="secondary" @click="isEditing = false" class="flex-1 py-2">Cancel Edit</BaseButton>
            <BaseButton variant="success" @click="saveEdit" class="flex-1 py-2 shadow-sm">Save Changes</BaseButton>
          </template>
          <template v-else>
            <BaseButton variant="danger" @click="emit('reject-pending')" class="flex-none py-2 px-4 shadow-sm bg-white border border-red-200 text-red-600 hover:bg-red-50 hover:border-red-300">
              <span :class="fontSm">Reject</span>
            </BaseButton>
            <BaseButton variant="primary" @click="emit('accept-pending')" :disabled="!tableNumberInput" class="flex-1 py-2 shadow-sm hover:shadow-md transition-all">
              <span :class="fontBase">Accept & Send</span>
            </BaseButton>
          </template>
        </div>
      </div>

      <div v-else-if="selectedOrder" class="h-full max-w-2xl w-full mx-auto flex flex-col gap-3 animate-in fade-in duration-300">
        <div class="bg-white border border-gray-200 rounded-xl p-4 shadow-sm relative flex flex-col flex-1 min-h-0">
          <div :class="selectedOrder.status === 'Cooking' ? 'bg-orange-400' : 'bg-green-400'" class="absolute top-0 left-0 w-full h-1"></div>
          
          <div class="flex justify-between items-start mb-3 shrink-0">
            <div>
              <span class="text-gray-500 font-bold tracking-wide uppercase text-[10px]">Order #{{ selectedOrder.order_id }}</span>
              <h3 :class="['font-black text-gray-900 mt-0.5', fontLg]">{{ selectedOrder.customer_identifier }}</h3>
              <p :class="['font-medium text-gray-500', fontSm]">{{ selectedOrder.order_type }}</p>
            </div>
            <span :class="[selectedOrder.status === 'Cooking' ? 'bg-orange-50 text-orange-600 animate-pulse' : 'bg-green-50 text-green-600', 'px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-wider']">
              {{ selectedOrder.status }}
            </span>
          </div>

          <div class="flex-1 overflow-y-auto border-t border-b border-gray-100 py-2 my-2 pr-2">
            <div class="flex justify-between items-center mb-2 sticky top-0 bg-white z-10 py-1">
              <h4 class="font-bold text-gray-800 text-xs uppercase tracking-wide">Order Items</h4>
            </div>
            
            <div v-if="selectedOrder.cart_items && selectedOrder.cart_items.length > 0" class="space-y-1.5">
              <div v-for="(item, idx) in selectedOrder.cart_items" :key="idx" class="flex justify-between items-center bg-gray-50 py-1.5 px-2 rounded border border-gray-100">
                <p :class="['font-bold text-gray-800', fontSm]">{{ item.qty }}x {{ item.pos_display_name || 'Item' }}</p>
                <span :class="['font-bold text-gray-600', fontSm]">₱{{ (item.unit_price * item.qty).toFixed(2) }}</span>
              </div>
            </div>

            <div v-else class="h-full flex flex-col justify-center items-center py-6">
               <p class="text-gray-400 text-sm font-medium text-center">No item details found.</p>
            </div>
          </div>

          <div class="pt-2 flex justify-between items-center shrink-0">
            <span :class="['font-bold text-gray-800', fontSm]">Total Due</span>
            <span :class="['font-black text-blue-600', fontLg]">₱{{ selectedOrder.total_amount.toFixed(2) }}</span>
          </div>
        </div>

        <div class="flex flex-col sm:flex-row gap-2 shrink-0">
          <BaseButton v-if="selectedOrder.status === 'Cooking'" variant="secondary" @click="emit('update-status', 'Cooked')" class="flex-1 py-2 shadow-sm">
            <span :class="fontBase">Mark Cooked</span>
          </BaseButton>
          <BaseButton variant="success" @click="emit('settle-payment')" class="flex-1 py-2 shadow-sm hover:shadow-md transition-all">
            <span :class="fontBase">Settle Payment</span>
          </BaseButton>
        </div>
      </div>

    </div>
  </div>
</template>