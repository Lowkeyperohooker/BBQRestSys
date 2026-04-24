<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue';
import { inventoryService, type RawInventoryItem } from '../services/inventoryService';
import { staffService } from '../services/staffService';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import { useResponsive } from '../composables/useResponsive';

const { fontSm, fontBase, fontLg } = useResponsive();

// Form State
const selectedCategory = ref('');
const selectedPart = ref('');
const rawKilos = ref<number | null>(null);
const skewersProduced = ref<number | null>(null);
const selectedStaff = ref('');

// Data from Database
const staffMembers = ref<any[]>([]);
const availableCategories = ref<string[]>([]);
const availableParts = ref<RawInventoryItem[]>([]);
const currentStockInfo = ref<RawInventoryItem | null>(null);

// Loading states
const isLoadingData = ref(true);
const isLoadingParts = ref(false);

const maxKilosAllowed = computed(() => {
  if (!currentStockInfo.value) return 0;
  return currentStockInfo.value.current_stock_kg;
});

const canPrep = computed(() => {
  if (!selectedCategory.value || !selectedPart.value || !rawKilos.value || !skewersProduced.value || !selectedStaff.value) {
    return false;
  }
  if (!currentStockInfo.value) return false;
  return rawKilos.value > 0 && rawKilos.value <= currentStockInfo.value.current_stock_kg && skewersProduced.value > 0;
});

const stockStatus = computed(() => {
  if (!currentStockInfo.value) return { text: 'No stock info', class: 'text-gray-500' };
  const stock = currentStockInfo.value.current_stock_kg;
  if (stock === 0) return { text: 'Out of stock', class: 'text-red-600' };
  if (stock < 1) return { text: 'Critically low', class: 'text-orange-600' };
  return { text: `${stock.toFixed(2)} kg available`, class: 'text-green-600' };
});

async function loadStaff() {
  const result = await staffService.getAllStaff();
  staffMembers.value = (result as any[]).filter(staff => staff.status === 'Active');
  if (staffMembers.value.length > 0) {
    selectedStaff.value = staffMembers.value[0].full_name;
  }
}

async function loadCategories() {
  try {
    availableCategories.value = await inventoryService.getAvailableCategories();
    if (availableCategories.value.length > 0) {
      selectedCategory.value = availableCategories.value[0];
    }
  } catch (error) {
    console.error("Failed to load categories:", error);
  }
}

async function loadParts(category: string) {
  if (!category) {
    availableParts.value = [];
    selectedPart.value = '';
    return;
  }
  
  isLoadingParts.value = true;
  try {
    availableParts.value = await inventoryService.getAvailableParts(category);
    if (availableParts.value.length > 0) {
      selectedPart.value = availableParts.value[0].specific_part;
    } else {
      selectedPart.value = '';
    }
  } catch (error) {
    console.error("Failed to load parts:", error);
  } finally {
    isLoadingParts.value = false;
  }
}

function updateCurrentStockInfo() {
  if (selectedCategory.value && selectedPart.value) {
    currentStockInfo.value = availableParts.value.find(
      p => p.specific_part === selectedPart.value
    ) || null;
  } else {
    currentStockInfo.value = null;
  }
}

async function handleSavePrep() {
  if (!canPrep.value) {
    if (rawKilos.value && currentStockInfo.value && rawKilos.value > currentStockInfo.value.current_stock_kg) {
      alert(`Insufficient stock! Available: ${currentStockInfo.value.current_stock_kg.toFixed(2)}kg, Requested: ${rawKilos.value}kg`);
    } else {
      alert("Please fill out all fields correctly.");
    }
    return;
  }

  try {
    await inventoryService.logPrepTransaction(
      selectedCategory.value, 
      selectedPart.value, 
      rawKilos.value!, 
      skewersProduced.value!,
      selectedStaff.value
    );

    await loadCategories();
    await loadParts(selectedCategory.value);
    
    rawKilos.value = null;
    skewersProduced.value = null;
    
    alert("Prep log successfully saved! Inventory has been updated.");

  } catch (error) {
    console.error("Failed to log prep:", error);
    alert(`Error: ${error instanceof Error ? error.message : 'Database error occurred'}`);
  }
}

watch(selectedCategory, async (newCategory) => {
  if (newCategory) {
    await loadParts(newCategory);
  }
});

watch([selectedPart, availableParts], () => {
  updateCurrentStockInfo();
});

onMounted(async () => {
  isLoadingData.value = true;
  try {
    await loadStaff();
    await loadCategories();
  } finally {
    setTimeout(() => {
      isLoadingData.value = false;
    }, 600);
  }
});
</script>

<template>
  <div class="h-full flex flex-col items-center justify-center p-4 md:p-6">
    
    <div v-if="isLoadingData" class="w-full max-w-3xl bg-white rounded-2xl shadow-sm border border-gray-100 flex items-center justify-center h-64">
      <DataLoader message="Loading preparation environment..." />
    </div>

    <div v-else class="w-full max-w-3xl bg-white border border-gray-100 rounded-2xl shadow-sm flex flex-col overflow-hidden max-h-full">
      <div class="p-6 md:px-8 md:py-6 border-b border-gray-100 shrink-0 bg-gray-50/50 flex justify-between items-center">
        <div>
          <h2 :class="['font-extrabold text-gray-900 tracking-tight', fontLg]">Prep Station</h2>
          <p :class="['font-medium text-gray-500 mt-1', fontSm]">Log skewering tasks</p>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto p-6 md:p-8">
        <form @submit.prevent="handleSavePrep" class="grid grid-cols-1 md:grid-cols-2 gap-6">
          
          <div class="col-span-1">
            <label :class="['block font-bold text-gray-700 mb-1.5', fontSm]">Meat Category</label>
            <select 
              v-model="selectedCategory" 
              :disabled="availableCategories.length === 0"
              :class="['w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-bold focus:outline-none focus:border-blue-500 bg-white disabled:bg-gray-50 transition-colors', fontBase]"
            >
              <option value="">Select Category</option>
              <option v-for="cat in availableCategories" :key="cat" :value="cat">{{ cat }}</option>
            </select>
            <p v-if="availableCategories.length === 0" :class="['text-orange-600 mt-2 font-bold', fontSm]">
              No categories with available stock
            </p>
          </div>
          
          <div class="col-span-1">
            <label :class="['block font-bold text-gray-700 mb-1.5', fontSm]">Specific Part / Cut</label>
            <select 
              v-model="selectedPart" 
              :disabled="!selectedCategory || isLoadingParts || availableParts.length === 0"
              :class="['w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-bold focus:outline-none focus:border-blue-500 bg-white disabled:bg-gray-50 transition-colors', fontBase]"
            >
              <option value="">Select Part</option>
              <option v-for="part in availableParts" :key="part.raw_item_id" :value="part.specific_part">
                {{ part.specific_part }} ({{ part.current_stock_kg.toFixed(2) }} kg available)
              </option>
            </select>
            <p v-if="isLoadingParts" :class="['text-gray-500 font-bold mt-2', fontSm]">Loading parts...</p>
          </div>
          
          <div v-if="currentStockInfo" class="md:col-span-2 bg-blue-50/50 border border-blue-100 p-4 rounded-xl flex items-center justify-between">
            <div>
              <p :class="['font-bold', stockStatus.class, fontBase]">{{ stockStatus.text }}</p>
              <p :class="['text-gray-500 font-medium mt-1', fontSm]">Alert threshold: {{ currentStockInfo.alert_threshold_kg }} kg</p>
            </div>
            <div class="hidden md:block text-blue-300">
              <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
            </div>
          </div>
          
          <div class="col-span-1">
            <label :class="['block font-bold text-gray-700 mb-1.5', fontSm]">
              Raw Kilos Used 
              <span v-if="maxKilosAllowed > 0" class="text-gray-400 font-medium ml-1">
                (Max: {{ maxKilosAllowed.toFixed(2) }} kg)
              </span>
            </label>
            <input 
              v-model.number="rawKilos" 
              type="number" 
              step="0.1"
              min="0.1"
              :max="maxKilosAllowed"
              :disabled="!currentStockInfo || currentStockInfo.current_stock_kg === 0"
              required 
              placeholder="e.g., 5.0" 
              :class="['w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-bold focus:outline-none focus:border-blue-500 disabled:bg-gray-50 bg-white transition-colors', fontBase]"
            />
          </div>
          
          <div class="col-span-1">
            <label :class="['block font-bold text-gray-700 mb-1.5', fontSm]">Skewers Produced</label>
            <input 
              v-model.number="skewersProduced" 
              type="number"
              min="1"
              :disabled="!currentStockInfo || currentStockInfo.current_stock_kg === 0"
              required 
              placeholder="e.g., 120" 
              :class="['w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-bold focus:outline-none focus:border-blue-500 disabled:bg-gray-50 bg-white transition-colors', fontBase]"
            />
          </div>
          
          <div class="col-span-1">
            <label :class="['block font-bold text-gray-700 mb-1.5', fontSm]">Staff Member</label>
            <select 
              v-model="selectedStaff" 
              required 
              :class="['w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-bold focus:outline-none focus:border-blue-500 bg-white transition-colors', fontBase]"
            >
              <option v-for="staff in staffMembers" :key="staff.staff_id" :value="staff.full_name">
                {{ staff.full_name }} ({{ staff.role }})
              </option>
            </select>
          </div>
          
          <div class="col-span-1 flex items-end mt-2 md:mt-0">
            <BaseButton 
              type="submit" 
              variant="primary"
              :disabled="!canPrep"
              :class="['w-full py-3 shadow-md hover:shadow-lg transition-all h-13', fontBase]"
            >
              Save Prep Log
            </BaseButton>
          </div>
        </form>
      </div>
    </div>

  </div>
</template>