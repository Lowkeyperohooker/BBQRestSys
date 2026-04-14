<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue';
import { inventoryService, type RawInventoryItem, type PrepLog } from '../services/inventoryService';
import { staffService } from '../services/staffService';
import DataLoader from '../components/ui/DataLoader.vue';

// Form State
const selectedCategory = ref('');
const selectedPart = ref('');
const rawKilos = ref<number | null>(null);
const skewersProduced = ref<number | null>(null);
const selectedStaff = ref('');

// Data from Database
const staffMembers = ref<any[]>([]);
const recentLogs = ref<Array<any>>([]);
const availableCategories = ref<string[]>([]);
const availableParts = ref<RawInventoryItem[]>([]);
const currentStockInfo = ref<RawInventoryItem | null>(null);

// Loading states
const isLoadingData = ref(true);
const isLoadingParts = ref(false);

// Computed properties for validation
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

async function loadRecentLogs() {
  try {
    const logs = await inventoryService.getRecentPrepLogs(10);
    recentLogs.value = logs.map((log: PrepLog) => ({
      time: new Date(log.timestamp).toLocaleString([], {
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      }),
      staff: log.staff_name,
      part: `${log.category} - ${log.specific_part}`,
      kilos: `-${log.kilos_deducted.toFixed(1)} kg`,
      sticks: `+${log.skewers_added} sticks`
    }));
  } catch (error) {
    console.error("Failed to load recent logs:", error);
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

    // Refresh data
    await loadCategories();
    await loadParts(selectedCategory.value);
    await loadRecentLogs();
    
    // Clear the numbers for the next task
    rawKilos.value = null;
    skewersProduced.value = null;
    
    alert("Prep log successfully saved! Inventory has been updated.");

  } catch (error) {
    console.error("Failed to log prep:", error);
    alert(`Error: ${error instanceof Error ? error.message : 'Database error occurred'}`);
  }
}

// Watch for category changes
watch(selectedCategory, async (newCategory) => {
  if (newCategory) {
    await loadParts(newCategory);
  }
});

// Watch for part changes
watch([selectedPart, availableParts], () => {
  updateCurrentStockInfo();
});

onMounted(async () => {
  isLoadingData.value = true;
  try {
    await loadStaff();
    await loadCategories();
    await loadRecentLogs();
  } finally {
    setTimeout(() => {
      isLoadingData.value = false;
    }, 600);
  }
});
</script>

<template>
  <div class="h-full flex flex-col">
    
    <div v-if="isLoadingData" class="flex-1 bg-white rounded-xl shadow-sm border border-gray-100 flex items-center justify-center">
      <DataLoader message="Loading preparation environment..." />
    </div>

    <div v-else class="grid grid-cols-1 lg:grid-cols-3 gap-6 flex-1 overflow-y-auto pb-8">
        
      <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 lg:col-span-1 h-fit">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">Log Skewering Task</h3>
        
        <form @submit.prevent="handleSavePrep" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Meat Category</label>
            <select 
              v-model="selectedCategory" 
              :disabled="availableCategories.length === 0"
              class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white disabled:bg-gray-100"
            >
              <option value="">Select Category</option>
              <option v-for="cat in availableCategories" :key="cat" :value="cat">{{ cat }}</option>
            </select>
            <p v-if="availableCategories.length === 0" class="text-sm text-orange-600 mt-1">
              No categories with available stock
            </p>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Specific Part / Cut</label>
            <select 
              v-model="selectedPart" 
              :disabled="!selectedCategory || isLoadingParts || availableParts.length === 0"
              class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white disabled:bg-gray-100"
            >
              <option value="">Select Part</option>
              <option v-for="part in availableParts" :key="part.raw_item_id" :value="part.specific_part">
                {{ part.specific_part }} ({{ part.current_stock_kg.toFixed(2) }} kg available)
              </option>
            </select>
            <p v-if="isLoadingParts" class="text-sm text-gray-500 mt-1">Loading parts...</p>
          </div>
          
          <div v-if="currentStockInfo" class="bg-blue-50 p-3 rounded-lg">
            <p class="text-sm font-medium" :class="stockStatus.class">{{ stockStatus.text }}</p>
            <p class="text-xs text-gray-600 mt-1">Alert threshold: {{ currentStockInfo.alert_threshold_kg }} kg</p>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Raw Kilos Used 
              <span v-if="maxKilosAllowed > 0" class="text-gray-500 font-normal">
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
              class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100"
            />
            <p v-if="rawKilos && currentStockInfo && rawKilos > currentStockInfo.current_stock_kg" class="text-sm text-red-600 mt-1">
              Exceeds available stock.
            </p>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Skewers Produced</label>
            <input 
              v-model.number="skewersProduced" 
              type="number"
              min="1"
              :disabled="!currentStockInfo || currentStockInfo.current_stock_kg === 0"
              required 
              placeholder="e.g., 120" 
              class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100"
            />
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Staff Member</label>
            <select 
              v-model="selectedStaff" 
              required 
              class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
            >
              <option v-for="staff in staffMembers" :key="staff.staff_id" :value="staff.full_name">
                {{ staff.full_name }} ({{ staff.role }})
              </option>
            </select>
          </div>
          
          <button 
            type="submit" 
            :disabled="!canPrep"
            class="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 mt-4 rounded-lg transition-colors disabled:bg-gray-400 disabled:cursor-not-allowed"
          >
            Save Prep Log
          </button>
        </form>
      </div>

      <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 lg:col-span-2">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">Recent Skewering Activity</h3>
        <div class="overflow-x-auto">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr class="border-b-2 border-gray-200 text-gray-500 text-sm">
                <th class="pb-3 font-semibold">Time</th>
                <th class="pb-3 font-semibold">Staff Member</th>
                <th class="pb-3 font-semibold">Part Used</th>
                <th class="pb-3 font-semibold">Kilos Deducted</th>
                <th class="pb-3 font-semibold">Skewers Added</th>
              </tr>
            </thead>
            <tbody class="text-gray-700">
              <tr v-if="recentLogs.length === 0">
                <td colspan="5" class="py-8 text-center text-gray-500">No prep activity logged yet today.</td>
              </tr>
              <tr v-for="(log, index) in recentLogs" :key="index" class="border-b border-gray-100 hover:bg-gray-50">
                <td class="py-4 text-sm">{{ log.time }}</td>
                <td class="py-4 font-medium">{{ log.staff }}</td>
                <td class="py-4">{{ log.part }}</td>
                <td class="py-4 text-red-600 font-semibold">{{ log.kilos }}</td>
                <td class="py-4 text-green-600 font-semibold">{{ log.sticks }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

    </div>
  </div>
</template>