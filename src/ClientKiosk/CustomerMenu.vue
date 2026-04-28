<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { posService, type PosItem, type CartItem } from '../services/posService';
import { queueService } from '../services/queueService';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import CheckOrderModal from '../components/modal/CheckOrderModal.vue';
import MenuItemModal from '../components/modal/MenuItemModal.vue';
import MenuVariantModal from '../components/modal/MenuVariantModal.vue';
import MenuSidebar from '../components/layout/MenuSidebar.vue';
import { useResponsive } from '../composables/useResponsive';

const { fontSm, fontLg, fontXl, font2xl, isMobile } = useResponsive();

const availableItems = ref<PosItem[]>([]);
const cart = ref<CartItem[]>([]);
const isLoadingData = ref(true);

const hasSelectedOrderType = ref(false);
const orderType = ref<'Dine-in' | 'Takeout' | null>(null);
const generatedQueueNumber = ref<number | null>(null);

const selectedCategory = ref<string>('Skewered');

const isCartModalOpen = ref(false);
const isItemModalOpen = ref(false);
const selectedItemForModal = ref<PosItem | null>(null);

const isVariantModalOpen = ref(false);
const selectedGroupForVariant = ref<any>(null);

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

const categories = computed(() => {
  const uniqueCats = Array.from(new Set(availableItems.value.map(item => item.category)));
  const otherCats = uniqueCats.filter(cat => cat !== 'Skewered').sort();
  
  if (uniqueCats.includes('Skewered')) {
    return ['Skewered', ...otherCats];
  }
  return otherCats;
});

watch(categories, (newCats) => {
  if (newCats.length > 0 && !selectedCategory.value) {
    selectedCategory.value = newCats.includes('Skewered') ? 'Skewered' : newCats[0];
  }
});

const filteredItems = computed(() => {
  return availableItems.value.filter(item => item.category === selectedCategory.value);
});

const groupedItems = computed(() => {
  const groups: any[] = [];
  const groupMap = new Map();

  filteredItems.value.forEach(item => {
    if (item.variant_group) {
      if (groupMap.has(item.variant_group)) {
        groupMap.get(item.variant_group).variants.push(item);
      } else {
        const newGroup = {
          is_group: true,
          display_name: item.variant_group,
          category: item.category,
          photo_url: item.photo_url,
          variants: [item],
          get unit_price() { return Math.min(...this.variants.map((v: any) => v.unit_price)); },
          get current_stock_pieces() { return this.variants.reduce((sum: number, v: any) => sum + v.current_stock_pieces, 0); }
        };
        groupMap.set(item.variant_group, newGroup);
        groups.push(newGroup);
      }
    } else {
      groups.push({
        is_group: false,
        display_name: item.pos_display_name,
        category: item.category,
        photo_url: item.photo_url,
        unit_price: item.unit_price,
        current_stock_pieces: item.current_stock_pieces,
        item: item
      });
    }
  });
  return groups;
});

const subtotal = computed(() => cart.value.reduce((sum, item) => sum + (item.unit_price * item.qty), 0));
const tax = computed(() => subtotal.value * 0.0);
const total = computed(() => subtotal.value + tax.value);
const totalCartItems = computed(() => cart.value.reduce((sum, item) => sum + item.qty, 0));

function selectOrderType(type: 'Dine-in' | 'Takeout') {
  orderType.value = type;
  hasSelectedOrderType.value = true;
}

function handleItemClick(group: any) {
  if (group.is_group) {
    selectedGroupForVariant.value = group;
    isVariantModalOpen.value = true;
  } else {
    openItemModal(group.item);
  }
}

function selectVariant(variant: PosItem) {
  isVariantModalOpen.value = false;
  selectedGroupForVariant.value = null;
  openItemModal(variant);
}

function openItemModal(item: PosItem) {
  selectedItemForModal.value = item;
  isItemModalOpen.value = true;
}

function setCategory(cat: string) {
  selectedCategory.value = cat;
}

function handleConfirmItemConfig({ qty, customPrice }: { qty: number, customPrice: number }) {
  if (!selectedItemForModal.value) return;
  const item = selectedItemForModal.value;

  const totalQtyInCart = cart.value
    .filter(c => c.prep_item_id === item.prep_item_id)
    .reduce((sum, c) => sum + c.qty, 0);

  if (totalQtyInCart + qty > item.current_stock_pieces) {
    alert(`Sorry, you can only add ${item.current_stock_pieces - totalQtyInCart} more of this item.`);
    return;
  }

  const existing = cart.value.find(c => c.prep_item_id === item.prep_item_id && c.unit_price === customPrice);

  if (existing) {
    existing.qty += qty;
  } else {
    cart.value.push({ ...item, unit_price: customPrice, qty: qty });
  }

  isItemModalOpen.value = false;
  selectedItemForModal.value = null;
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
  selectedCategory.value = categories.value.includes('Skewered') ? 'Skewered' : 'All';
  isCartModalOpen.value = false;
}

async function handleFinalizeOrder() {
  if (cart.value.length === 0 || !orderType.value) return;

  try {
    const queueNum = await queueService.getNextQueueNumber();

    const pendingOrder = {
      queue_number: queueNum,
      order_type: orderType.value,
      cart_items: cart.value,
      subtotal: subtotal.value,
      tax: tax.value,
      total: total.value,
      timestamp: new Date().toISOString()
    };

    await queueService.addToQueue(pendingOrder);
    
    generatedQueueNumber.value = queueNum;
    isCartModalOpen.value = false;

  } catch (error) {
    console.error("Failed to save pending order:", error);
    alert("Failed to process your order. Please call a staff member.");
  }
}

function handleImageError(event: Event) {
  const target = event.target as HTMLImageElement;
  if (target) {
    target.style.display = 'none';
  }
}
</script>

<template>
  <div class="h-full flex flex-col relative max-w-7xl mx-auto">

    <div v-if="!hasSelectedOrderType" class="flex-1 flex flex-col items-center justify-center p-4 md:p-6 bg-surface-container-low rounded-2xl shadow-sm border border-outline-variant/15">
      <h1 :class="['font-black text-on-surface tracking-tight mb-2', font2xl]">WELCOME TO</h1>
      <div class="w-24 h-24 md:w-28 md:h-28 text-primary mb-6">
        <img src="../assets/bbq-icon.png" class="w-full h-full object-contain" alt="lock icon">
      </div>
      <p :class="['text-on-surface-variant font-medium mb-8', fontLg]">How would you like your order today?</p>
      
      <div class="flex flex-col sm:flex-row gap-4 w-full max-w-sm sm:max-w-md md:max-w-lg">
        <button @click="selectOrderType('Dine-in')" class="flex-1 bg-surface-container hover:bg-surface-container-high border-2 border-outline-variant/20 hover:border-primary-container rounded-3xl p-5 md:p-6 flex flex-col items-center gap-3 transition-all shadow-sm group">
          <svg class="w-12 h-12 md:w-14 md:h-14 text-on-surface-variant group-hover:text-primary transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 15.546c-.523 0-1.046.151-1.5.454a2.704 2.704 0 01-3 0 2.704 2.704 0 00-3 0 2.704 2.704 0 01-3 0 2.704 2.704 0 00-3 0 2.704 2.704 0 01-3 0 2.701 2.701 0 00-1.5-.454M9 6v2m3-2v2m3-2v2M9 3h.01M12 3h.01M15 3h.01M6.75 21A3.75 3.75 0 013 17.25V14.16M21 14.16v3.09a3.75 3.75 0 01-3.75 3.75M6.75 21H17.25"></path></svg>
          <span :class="['font-black text-on-surface group-hover:text-primary-container', fontLg]">Dine-In</span>
        </button>
        
        <button @click="selectOrderType('Takeout')" class="flex-1 bg-surface-container hover:bg-surface-container-high border-2 border-outline-variant/20 hover:border-primary-container rounded-3xl p-5 md:p-6 flex flex-col items-center gap-3 transition-all shadow-sm group">
          <svg class="w-12 h-12 md:w-14 md:h-14 text-on-surface-variant group-hover:text-primary transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"></path></svg>
          <span :class="['font-black text-on-surface group-hover:text-primary-container', fontLg]">Takeout</span>
        </button>
      </div>
    </div>

    <div v-else-if="!generatedQueueNumber" class="flex-1 bg-surface-container-low rounded-2xl overflow-hidden border border-outline-variant/15 flex flex-col min-h-0 relative">
      <div class="z-40 bg-surface-container-low border-b border-outline-variant/20 px-5 py-4 md:px-6 md:py-5 shrink-0 flex justify-between items-center">
        <div>
          <div class="flex items-center gap-4 mb-2">
            <span class="bg-primary-container/10 border border-primary-container/20 text-primary-container text-xs font-bold px-3 py-1 rounded-full uppercase tracking-widest">{{ orderType }}</span>
            <button @click="resetKiosk" class="text-on-surface-variant hover:text-error text-xs font-bold hover:underline transition-colors tracking-wide uppercase">Change</button>
          </div>
          <h2 :class="['font-black text-on-surface tracking-tight', font2xl]">Self-Service Menu</h2>
          <p :class="['font-medium text-on-surface-variant mt-1', fontSm]">Tap items to add them to your order</p>
        </div>
      </div>

      <div class="flex-1 flex flex-col md:flex-row overflow-hidden bg-surface">
        <MenuSidebar 
          :categories="categories" 
          :selected-category="selectedCategory" 
          @select="setCategory" 
        />

        <div class="flex-1 overflow-y-auto p-4 md:p-6 lg:p-8 pb-32">
          <div v-if="isLoadingData" class="flex-1 flex items-center justify-center h-64">
            <DataLoader message="Loading fresh menu..." />
          </div>

          <div v-else>
            <div v-if="groupedItems.length === 0" class="flex flex-col items-center justify-center h-64 text-on-surface-variant">
              <svg class="w-16 h-16 mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"></path></svg>
              <p class="text-lg font-medium">No items available in this category.</p>
            </div>

            <div v-else :class="['grid gap-4 md:gap-5', isMobile ? 'grid-cols-2' : 'grid-cols-2 lg:grid-cols-3 xl:grid-cols-4']">
              <div v-for="(group, idx) in groupedItems" :key="idx" @click="handleItemClick(group)"
                class="bg-surface-container-low p-4 md:p-5 rounded-2xl shadow-sm border border-outline-variant/15 cursor-pointer hover:border-primary hover:shadow-lg hover:-translate-y-1 transition-all relative flex flex-col h-full min-h-55 group-hover">

                <div class="h-28 md:h-36 bg-surface-container rounded-xl mb-3 md:mb-4 flex items-center justify-center text-on-surface-variant overflow-hidden border border-outline-variant/10 relative opacity-100 transition-opacity hover:opacity-90">
                  <img 
                    v-if="group.photo_url" 
                    :src="`http://localhost:3000${group.photo_url}`" 
                    class="w-full h-full object-cover" 
                    @error="handleImageError" 
                  />
                  <svg v-else class="w-10 h-10 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                </div>
                
                <h4 :class="['font-black text-on-surface mb-1 leading-tight tracking-tight', fontLg]" :title="group.display_name">{{ group.display_name }}</h4>
                
                <div class="flex justify-between items-end mt-auto pt-3">
                  <div class="flex flex-col">
                    <span v-if="group.is_group" class="text-[9px] text-on-surface-variant font-medium uppercase tracking-widest mb-0.5">Starts at</span>
                    <span :class="['text-primary-container font-black leading-none', fontXl]">₱{{ group.unit_price.toFixed(2) }}</span>
                  </div>
                  <p v-if="group.current_stock_pieces <= 0" class="text-[10px] md:text-xs text-error font-black uppercase tracking-wider">Sold Out</p>
                  <p v-else-if="group.is_group" class="text-[10px] md:text-xs text-primary font-bold uppercase tracking-wider">Options</p>
                  <p v-else-if="group.current_stock_pieces < 10" class="text-[10px] md:text-xs text-tertiary-container font-bold uppercase tracking-wider">Only {{ group.current_stock_pieces }} left</p>
                  <p v-else class="text-[10px] md:text-xs text-on-surface-variant font-medium uppercase tracking-wider">Available</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="flex-1 flex flex-col items-center justify-center p-6 bg-surface-container-low rounded-2xl shadow-sm border border-outline-variant/15 text-center">
      <div class="w-24 h-24 bg-tertiary/10 border border-tertiary/20 rounded-full flex items-center justify-center text-tertiary mb-8 shadow-[0_0_24px_rgba(240,190,116,0.15)]">
        <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path></svg>
      </div>
      <h2 :class="['font-black text-on-surface mb-2 tracking-tight', font2xl]">Order Placed!</h2>
      <p :class="['text-on-surface-variant font-medium mb-8', fontLg]">Please proceed to the cashier and present this number:</p>
      
      <div class="bg-surface-container border border-dashed border-outline-variant/30 rounded-3xl px-16 py-8 mb-12 shadow-inner">
        <span class="text-7xl font-black text-primary-container tracking-widest">{{ generatedQueueNumber }}</span>
      </div>

      <BaseButton @click="resetKiosk" variant="primary" class="py-4 px-12 shadow-lg hover:shadow-xl rounded-full">
        <span :class="['font-black uppercase tracking-widest', fontLg]">Done</span>
      </BaseButton>
    </div>

    <div v-if="totalCartItems > 0 && hasSelectedOrderType && !generatedQueueNumber" class="fixed bottom-6 right-6 md:bottom-8 md:right-8 z-50">
      <BaseButton @click="isCartModalOpen = true" variant="primary" class="py-4 shadow-[0_0_32px_rgba(255,109,0,0.3)] flex items-center gap-4 px-6 rounded-full hover:scale-105 transition-transform border border-primary-container/50">
        <div class="flex items-center justify-center bg-background/30 w-8 h-8 rounded-full text-sm font-black text-primary-fixed">{{ totalCartItems }}</div>
        <span :class="['font-black uppercase tracking-widest text-on-primary-container', fontLg]">Check Order</span>
        <span :class="['font-black ml-2 text-on-primary-container', fontLg]">₱{{ total.toFixed(2) }}</span>
      </BaseButton>
    </div>

    <MenuVariantModal
      :is-open="isVariantModalOpen"
      :group="selectedGroupForVariant"
      @close="isVariantModalOpen = false; selectedGroupForVariant = null"
      @select-variant="selectVariant"
    />

    <CheckOrderModal 
      :is-open="isCartModalOpen"
      :cart="cart"
      :subtotal="subtotal"
      :total="total"
      @close="isCartModalOpen = false"
      @remove-item="removeFromCart"
      @finalize-order="handleFinalizeOrder"
    />

    <MenuItemModal 
      :is-open="isItemModalOpen"
      :item="selectedItemForModal"
      @close="isItemModalOpen = false; selectedItemForModal = null"
      @confirm="handleConfirmItemConfig"
    />

  </div>
</template>