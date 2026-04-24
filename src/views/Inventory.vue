<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import type { RawInventoryItem, PreparedInventoryItem, POSCategory } from '../services/inventoryService';
import { inventoryService } from '../services/inventoryService';
import EditInventoryModal from '../components/ui/EditInventoryModal.vue';
import EditPriceModal from '../components/ui/EditPriceModal.vue';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import BaseBadge from '../components/ui/BaseBadge.vue';
import { useResponsive } from '../composables/useResponsive';

const { fontSm, fontBase, fontXl, isMobile } = useResponsive();

const rawInventory = ref<RawInventoryItem[]>([]);
const preparedInventory = ref<PreparedInventoryItem[]>([]);
const posCategories = ref<POSCategory[]>([]);
const viewMode = ref('raw'); 

const isLoadingData = ref(true);

const isInventoryModalOpen = ref(false);
const isPriceModalOpen = ref(false);
const selectedPreparedItem = ref<PreparedInventoryItem | null>(null);

async function loadData() {
  isLoadingData.value = true;
  try {
    rawInventory.value = await inventoryService.getRawInventory();
    preparedInventory.value = await inventoryService.getPreparedInventory();
    posCategories.value = await inventoryService.getPosCategories();
  } catch (error) {
    console.error("Error loading inventory:", error);
  } finally {
    setTimeout(() => { isLoadingData.value = false; }, 600);
  }
}

const currentPreparedItems = computed(() => {
  return preparedInventory.value.filter(item => item.category === viewMode.value);
});

function openEditPriceModal(item: PreparedInventoryItem) {
  selectedPreparedItem.value = item;
  isPriceModalOpen.value = true;
}

// Updated to accept the photoUrl from the modal
async function handleSavePrice(data: { prepItemId: number; unitPrice: number; isVariablePrice: boolean; photoUrl: string | null }) {
  try {
    await inventoryService.updatePreparedItemPricing(data.prepItemId, data.unitPrice, data.isVariablePrice, data.photoUrl);
    isPriceModalOpen.value = false;
    await loadData();
  } catch (error) {
    alert("Failed to update item details.");
  }
}

function getStatusBadge(current: number, threshold: number): 'danger' | 'warning' | 'success' {
  if (current <= threshold * 0.5) return 'danger';
  if (current <= threshold) return 'warning';
  return 'success';
}

function getStatusText(current: number, threshold: number): string {
  if (current <= threshold * 0.5) return 'Critically Low';
  if (current <= threshold) return 'Low Stock';
  return 'Adequate';
}

function handleImageError(event: Event) {
  const target = event.target as HTMLImageElement;
  if (target) {
    target.style.display = 'none';
  }
}

onMounted(() => loadData());
</script>

<template>
  <div class="h-full flex flex-col space-y-6">
    <div class="bg-white p-3 rounded-xl shadow-sm border border-gray-100">
      
      <div class="sticky top-0 z-40 bg-gray-50/95 backdrop-blur -mt-3 md:-mt-4 -mx-3 md:-mx-4 px-3 md:px-4 pt-3 md:pt-4 pb-4 mb-6 border-b border-gray-200 flex flex-col md:flex-row justify-between items-start md:items-center gap-4 rounded-t-xl">
        <div>
          <h3 :class="['font-bold text-gray-800', fontXl]">Inventory Management</h3>
          <p :class="['text-gray-500 mt-1', fontSm]">
            {{ viewMode === 'raw' ? 'Monitored in Kilograms (kg)' : 'Monitored in Sticks/Pieces' }}
          </p>
        </div>
        
        <BaseButton variant="primary" @click="isInventoryModalOpen = true" :class="isMobile ? 'w-full' : ''">
          <svg class="w-4 h-4 mr-2 inline" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>
          Edit Inventory
        </BaseButton>
      </div>

      <div class="flex overflow-x-auto pb-2 mb-4 gap-2 border-b border-gray-100">
        <button @click="viewMode = 'raw'" :class="[viewMode === 'raw' ? 'bg-gray-800 text-white shadow' : 'bg-gray-100 text-gray-600 hover:bg-gray-200', 'px-5 py-2 font-bold rounded-lg transition-colors shrink-0', fontSm]">
          Raw Meats
        </button>
        <button v-for="cat in posCategories" :key="cat.category_name" @click="viewMode = cat.category_name" :class="[viewMode === cat.category_name ? 'bg-blue-600 text-white shadow' : 'bg-gray-100 text-gray-600 hover:bg-gray-200', 'px-5 py-2 font-bold rounded-lg transition-colors shrink-0', fontSm]">
          {{ cat.category_name }}
        </button>
      </div>

      <DataLoader v-if="isLoadingData" message="Fetching latest inventory records..." />

      <div v-else>
        <div v-if="viewMode === 'raw'" class="overflow-x-auto">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr :class="['border-b-2 border-gray-200 text-gray-500 uppercase tracking-wider', fontSm]">
                <th class="pb-3 font-bold px-2">Category</th>
                <th class="pb-3 font-bold px-2">Specific Part</th>
                <th class="pb-3 font-bold px-2">Current Stock</th>
                <th class="pb-3 font-bold px-2 hidden md:table-cell">Threshold</th>
                <th class="pb-3 font-bold px-2">Status</th>
              </tr>
            </thead>
            <tbody class="text-gray-700">
              <tr v-if="rawInventory.length === 0">
                <td colspan="5" class="py-8 text-center text-gray-500">No raw inventory found.</td>
              </tr>
              <tr v-for="item in rawInventory" :key="item.raw_item_id" class="border-b border-gray-100 hover:bg-gray-50 transition-colors">
                <td :class="['py-4 font-bold text-gray-900 px-2', fontBase]">{{ item.category }}</td>
                <td :class="['py-4 font-medium px-2', fontBase]">{{ item.specific_part }}</td>
                <td :class="['py-4 font-black px-2', item.current_stock_kg <= item.alert_threshold_kg ? 'text-red-600' : 'text-gray-800', fontBase]">
                  {{ item.current_stock_kg.toFixed(2) }} kg
                </td>
                <td :class="['py-4 text-gray-400 font-medium px-2 hidden md:table-cell', fontBase]">{{ item.alert_threshold_kg }} kg</td>
                <td class="py-4 px-2">
                  <BaseBadge :text="getStatusText(item.current_stock_kg, item.alert_threshold_kg)" :variant="getStatusBadge(item.current_stock_kg, item.alert_threshold_kg)" />
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div v-else class="overflow-x-auto">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr :class="['border-b-2 border-gray-200 text-gray-500 uppercase tracking-wider', fontSm]">
                <th class="pb-3 font-bold px-2">Item Name</th>
                <th class="pb-3 font-bold px-2">Stock Count</th>
                <th class="pb-3 font-bold px-2">Price (PHP)</th>
                <th class="pb-3 font-bold px-2 hidden md:table-cell">Pricing Type</th>
                <th class="pb-3 font-bold px-2 text-right">Actions</th>
              </tr>
            </thead>
            <tbody class="text-gray-700">
              <tr v-if="currentPreparedItems.length === 0">
                <td colspan="5" class="py-8 text-center text-gray-500">No items found in this category.</td>
              </tr>
              <tr v-for="item in currentPreparedItems" :key="item.prep_item_id" class="border-b border-gray-100 hover:bg-gray-50 transition-colors">
                
                <td :class="['py-3 px-2', fontBase]">
                  <div class="flex items-center gap-3">
                    <div v-if="item.photo_url" class="w-10 h-10 rounded-lg bg-gray-100 flex items-center justify-center overflow-hidden shrink-0 border border-gray-200">
                      <img :src="`http://localhost:3000${item.photo_url}`" class="w-full h-full object-cover" @error="handleImageError" />
                    </div>
                    <div v-else class="w-10 h-10 rounded-lg bg-gray-50 flex items-center justify-center shrink-0 border border-gray-200 text-gray-300">
                      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                    </div>
                    <span class="font-bold text-gray-900">{{ item.pos_display_name }}</span>
                  </div>
                </td>

                <td :class="['py-3 font-black text-gray-800 px-2', fontBase]">{{ item.current_stock_pieces }}</td>
                <td :class="['py-3 font-black text-blue-600 px-2', fontBase]">₱{{ item.unit_price.toFixed(2) }}</td>
                <td class="py-3 px-2 hidden md:table-cell">
                  <BaseBadge :text="item.is_variable_price ? 'Variable' : 'Fixed'" :variant="item.is_variable_price ? 'warning' : 'info'" />
                </td>
                <td class="py-3 px-2 text-right">
                  <button @click="openEditPriceModal(item)" :class="['text-blue-600 hover:text-blue-800 font-bold px-3 py-1 transition-colors', fontSm]">Edit Details</button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div> 
    </div>

    <EditInventoryModal :is-open="isInventoryModalOpen" @close="isInventoryModalOpen = false" @refresh="loadData" />
    <EditPriceModal :is-open="isPriceModalOpen" :item="selectedPreparedItem" @close="isPriceModalOpen = false; selectedPreparedItem = null" @save="handleSavePrice" />
  </div>
</template>