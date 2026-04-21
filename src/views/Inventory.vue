<script setup lang="ts">
import { ref, onMounted } from 'vue';
import type { RawInventoryItem, PreparedInventoryItem } from '../services/inventoryService';
import { inventoryService } from '../services/inventoryService';
import AddStockModal from '../components/ui/AddStockModal.vue';
import EditPriceModal from '../components/ui/EditPriceModal.vue';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import BaseBadge from '../components/ui/BaseBadge.vue';

const rawInventory = ref<RawInventoryItem[]>([]);
const preparedInventory = ref<PreparedInventoryItem[]>([]);
const viewMode = ref('raw'); 

const isLoadingData = ref(true);

const isStockModalOpen = ref(false);
const selectedRawItem = ref<RawInventoryItem | null>(null);

const isPriceModalOpen = ref(false);
const selectedPreparedItem = ref<PreparedInventoryItem | null>(null);

async function loadData() {
  isLoadingData.value = true;
  try {
    rawInventory.value = await inventoryService.getRawInventory();
    preparedInventory.value = await inventoryService.getPreparedInventory();
  } catch (error) {
    console.error("Error loading inventory:", error);
  } finally {
    setTimeout(() => {
      isLoadingData.value = false;
    }, 600);
  }
}

function openAddStockModal(item: RawInventoryItem | null = null) {
  selectedRawItem.value = item;
  isStockModalOpen.value = true;
}

function closeStockModal() {
  isStockModalOpen.value = false;
  selectedRawItem.value = null;
}

// Handle both new items and existing restocks
async function handleSaveStock(data: { isNew: boolean; itemId?: number; kilos: number; category: string; part: string; alertThreshold?: number }) {
  try {
    if (data.isNew) {
      await inventoryService.addNewRawItem(data.category, data.part, data.kilos, data.alertThreshold || 5.0);
    } else if (data.itemId) {
      await inventoryService.addRawStock(data.itemId, data.kilos);
    }
    closeStockModal();
    await loadData(); 
  } catch (error) {
    console.error("Error adding stock:", error);
    alert("Failed to update stock.");
  }
}

function openEditPriceModal(item: PreparedInventoryItem) {
  selectedPreparedItem.value = item;
  isPriceModalOpen.value = true;
}

function closePriceModal() {
  isPriceModalOpen.value = false;
  selectedPreparedItem.value = null;
}

async function handleSavePrice(data: { prepItemId: number; unitPrice: number; isVariablePrice: boolean }) {
  try {
    await inventoryService.updatePreparedItemPricing(data.prepItemId, data.unitPrice, data.isVariablePrice);
    closePriceModal();
    await loadData();
  } catch (error) {
    console.error("Error updating price:", error);
    alert("Failed to update pricing.");
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

onMounted(() => {
  loadData();
});
</script>

<template>
  <div class="h-full flex flex-col space-y-6">
    <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
      
      <div class="flex flex-col md:flex-row justify-between items-start md:items-center mb-6 gap-4">
        <div>
          <h3 class="text-lg font-semibold text-gray-800">Live Component Tracking</h3>
          <p class="text-sm text-gray-500">
            {{ viewMode === 'raw' ? 'Monitored in Kilograms (kg)' : 'Monitored in Sticks/Pieces' }}
          </p>
        </div>
        
        <div class="flex items-center gap-4">
          <div class="flex bg-gray-100 rounded-lg p-1">
            <button 
              @click="viewMode = 'raw'"
              :class="viewMode === 'raw' ? 'bg-white shadow text-gray-800' : 'text-gray-500 hover:text-gray-700'"
              class="px-4 py-1.5 text-sm font-medium rounded-md transition-all"
            >
              Raw Meats
            </button>
            <button 
              @click="viewMode = 'skewered'"
              :class="viewMode === 'skewered' ? 'bg-white shadow text-gray-800' : 'text-gray-500 hover:text-gray-700'"
              class="px-4 py-1.5 text-sm font-medium rounded-md transition-all"
            >
              Skewered
            </button>
          </div>
          
          <BaseButton v-if="viewMode === 'raw'" variant="primary" @click="openAddStockModal(null)">
            Add Stock Delivery
          </BaseButton>
        </div>
      </div>

      <DataLoader v-if="isLoadingData" message="Fetching latest inventory records..." />

      <div v-else>
        <div v-if="viewMode === 'raw'" class="overflow-x-auto">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr class="border-b-2 border-gray-200 text-gray-500 text-sm">
                <th class="pb-3 font-semibold">Category</th>
                <th class="pb-3 font-semibold">Specific Part</th>
                <th class="pb-3 font-semibold">Current Stock</th>
                <th class="pb-3 font-semibold">Alert Threshold</th>
                <th class="pb-3 font-semibold">Status</th>
                <th class="pb-3 font-semibold text-right">Actions</th>
              </tr>
            </thead>
            <tbody class="text-gray-700">
              <tr v-if="rawInventory.length === 0">
                <td colspan="6" class="py-8 text-center text-gray-500">No raw inventory found in database.</td>
              </tr>
              <tr v-for="item in rawInventory" :key="item.raw_item_id" class="border-b border-gray-100 hover:bg-gray-50 transition-colors">
                <td class="py-4 font-semibold text-gray-900">{{ item.category }}</td>
                <td class="py-4">{{ item.specific_part }}</td>
                <td class="py-4 font-bold" :class="item.current_stock_kg <= item.alert_threshold_kg ? 'text-red-600' : 'text-gray-800'">
                  {{ item.current_stock_kg }} kg
                </td>
                <td class="py-4 text-gray-400">{{ item.alert_threshold_kg }} kg</td>
                <td class="py-4">
                  <BaseBadge 
                    :text="getStatusText(item.current_stock_kg, item.alert_threshold_kg)"
                    :variant="getStatusBadge(item.current_stock_kg, item.alert_threshold_kg)"
                  />
                </td>
                <td class="py-4 text-right">
                  <button @click="openAddStockModal(item)" class="text-blue-600 hover:text-blue-800 font-medium text-sm px-3 py-1 transition-colors">
                    + Add Stock
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div v-if="viewMode === 'skewered'" class="overflow-x-auto">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr class="border-b-2 border-gray-200 text-gray-500 text-sm">
                <th class="pb-3 font-semibold">Item Name</th>
                <th class="pb-3 font-semibold">Prepared Stock</th>
                <th class="pb-3 font-semibold">Price (PHP)</th>
                <th class="pb-3 font-semibold">Pricing Type</th>
                <th class="pb-3 font-semibold text-right">Actions</th>
              </tr>
            </thead>
            <tbody class="text-gray-700">
              <tr v-if="preparedInventory.length === 0">
                <td colspan="5" class="py-8 text-center text-gray-500">No prepared inventory found in database.</td>
              </tr>
              <tr v-for="item in preparedInventory" :key="item.prep_item_id" class="border-b border-gray-100 hover:bg-gray-50 transition-colors">
                <td class="py-4 font-semibold text-gray-900">{{ item.pos_display_name }}</td>
                <td class="py-4 font-bold text-gray-800">{{ item.current_stock_pieces }} sticks</td>
                <td class="py-4 font-bold text-gray-900">{{ item.unit_price.toFixed(2) }}</td>
                <td class="py-4">
                  <BaseBadge 
                    :text="item.is_variable_price ? 'Variable' : 'Fixed'" 
                    :variant="item.is_variable_price ? 'warning' : 'info'" 
                  />
                </td>
                <td class="py-4 text-right">
                  <button @click="openEditPriceModal(item)" class="text-blue-600 hover:text-blue-800 font-medium text-sm px-3 py-1 transition-colors">
                    Edit Price
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div> 

    </div>

    <AddStockModal 
      :is-open="isStockModalOpen" 
      :item="selectedRawItem" 
      @close="closeStockModal" 
      @save="handleSaveStock" 
    />

    <EditPriceModal 
      :is-open="isPriceModalOpen"
      :item="selectedPreparedItem"
      @close="closePriceModal"
      @save="handleSavePrice"
    />

  </div>
</template>