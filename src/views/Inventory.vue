<script setup lang="ts">
import { ref, onMounted } from 'vue';
import type { RawInventoryItem } from '../services/inventoryService';
import { inventoryService } from '../services/inventoryService';
import AddStockModal from '../components/ui/AddStockModal.vue';

const rawInventory = ref<RawInventoryItem[]>([]);
const preparedInventory = ref<any[]>([]);
const viewMode = ref('raw'); // 'raw' or 'skewered'

// Modal State
const isModalOpen = ref(false);
const selectedRawItem = ref<RawInventoryItem | null>(null);

async function loadData() {
  try {
    rawInventory.value = await inventoryService.getRawInventory();
    preparedInventory.value = await inventoryService.getPreparedInventory();
  } catch (error) {
    console.error("Error loading inventory:", error);
  }
}

// Modal Controls
function openAddStockModal(item: RawInventoryItem | null = null) {
  selectedRawItem.value = item;
  isModalOpen.value = true;
}

function closeModal() {
  isModalOpen.value = false;
  selectedRawItem.value = null;
}

// Handle the save event from your new modal
async function handleSaveStock(data: { itemId: number; kilos: number; category: string; part: string }) {
  try {
    await inventoryService.addRawStock(data.itemId, data.kilos);
    closeModal();
    await loadData(); // Refresh the table so the new stock appears!
  } catch (error) {
    console.error("Error adding stock:", error);
    alert("Failed to update stock.");
  }
}

function getStatusBadge(current: number, threshold: number) {
  if (current <= threshold * 0.5) return { text: 'Critically Low', class: 'bg-red-100 text-red-800' };
  if (current <= threshold) return { text: 'Low Stock', class: 'bg-orange-100 text-orange-800' };
  return { text: 'Adequate', class: 'bg-green-100 text-green-800' };
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
          
          <button @click="openAddStockModal(null)" class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded shadow transition-colors text-sm font-semibold">
            Add Stock Delivery
          </button>
        </div>
      </div>

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
                <span :class="getStatusBadge(item.current_stock_kg, item.alert_threshold_kg).class" class="px-3 py-1 rounded-full text-xs font-bold">
                  {{ getStatusBadge(item.current_stock_kg, item.alert_threshold_kg).text }}
                </span>
              </td>
              <td class="py-4 text-right">
                <button @click="openAddStockModal(item)" class="text-blue-600 hover:text-blue-800 font-medium text-sm px-3 py-1">
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
              <th class="pb-3 font-semibold">Category</th>
              <th class="pb-3 font-semibold">Specific Part</th>
              <th class="pb-3 font-semibold">Prepared Stock</th>
              <th class="pb-3 font-semibold">Status</th>
            </tr>
          </thead>
          <tbody class="text-gray-700">
            <tr v-if="preparedInventory.length === 0">
              <td colspan="4" class="py-8 text-center text-gray-500">No prepared inventory found in database.</td>
            </tr>
            <tr v-for="item in preparedInventory" :key="item.prep_item_id" class="border-b border-gray-100 hover:bg-gray-50 transition-colors">
              <td class="py-4 font-semibold text-gray-900">{{ item.category }}</td>
              <td class="py-4">{{ item.pos_display_name }}</td>
              <td class="py-4 font-bold text-gray-800">{{ item.current_stock_pieces }} sticks</td>
              <td class="py-4">
                <span class="bg-green-100 text-green-800 px-3 py-1 rounded-full text-xs font-bold">Adequate</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

    </div>

    <AddStockModal 
      :is-open="isModalOpen" 
      :item="selectedRawItem" 
      @close="closeModal" 
      @save="handleSaveStock" 
    />

  </div>
</template>