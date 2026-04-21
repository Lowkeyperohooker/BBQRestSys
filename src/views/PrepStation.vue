<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue';
import { inventoryService, type RawInventoryItem, type PrepLog } from '../services/inventoryService';
import { staffService } from '../services/staffService';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import { useResponsive } from '../composables/useResponsive';

const { fontSm, fontBase, fontLg, fontXl } = useResponsive();

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

    await loadCategories();
    await loadParts(selectedCategory.value);
    await loadRecentLogs();
    
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

    <div v-else class="flex-1 bg-white rounded-xl shadow-sm border border-gray-100 flex flex-col min-h-0 overflow-hidden">
      <div class="h-full overflow-y-auto p-4 md:p-6 lg:p-8">

        <div class="sticky top-0 z-40 bg-white/95 backdrop-blur -mt-4 md:-mt-6 lg:-mt-8 -mx-4 md:-mx-6 lg:-mx-8 px-4 md:px-6 lg:px-8 pt-4 md:pt-6 lg:pt-8 pb-4 mb-8 border-b border-gray-200 rounded-t-xl">
          <h3 :class="['font-bold text-gray-800', fontXl]">Prep Station</h3>
          <p :class="['text-gray-500 mt-1', fontSm]">Convert bulk raw inventory into skewered POS items.</p>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 pb-4">
            
          <div class="bg-gray-50/50 p-5 rounded-xl border border-gray-100 lg:col-span-1 h-fit">
            <h3 :class="['font-semibold text-gray-800 mb-5', fontLg]">Log Skewering Task</h3>
            
            <form @submit.prevent="handleSavePrep" class="space-y-4 md:space-y-5">
              <div>
                <label :class="['block font-medium text-gray-700 mb-1.5', fontSm]">Meat Category</label>
                <select 
                  v-model="selectedCategory" 
                  :disabled="availableCategories.length === 0"
                  :class="['w-full border border-gray-300 rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white disabled:bg-gray-100', fontBase]"
                >
                  <option value="">Select Category</option>
                  <option v-for="cat in availableCategories" :key="cat" :value="cat">{{ cat }}</option>
                </select>
                <p v-if="availableCategories.length === 0" :class="['text-orange-600 mt-1.5', fontSm]">
                  No categories with available stock
                </p>
              </div>
              
              <div>
                <label :class="['block font-medium text-gray-700 mb-1.5', fontSm]">Specific Part / Cut</label>
                <select 
                  v-model="selectedPart" 
                  :disabled="!selectedCategory || isLoadingParts || availableParts.length === 0"
                  :class="['w-full border border-gray-300 rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white disabled:bg-gray-100', fontBase]"
                >
                  <option value="">Select Part</option>
                  <option v-for="part in availableParts" :key="part.raw_item_id" :value="part.specific_part">
                    {{ part.specific_part }} ({{ part.current_stock_kg.toFixed(2) }} kg available)
                  </option>
                </select>
                <p v-if="isLoadingParts" :class="['text-gray-500 mt-1.5', fontSm]">Loading parts...</p>
              </div>
              
              <div v-if="currentStockInfo" class="bg-blue-50 border border-blue-100 p-3.5 rounded-lg">
                <p :class="['font-semibold', stockStatus.class, fontSm]">{{ stockStatus.text }}</p>
                <p :class="['text-gray-600 mt-1', fontSm]">Alert threshold: {{ currentStockInfo.alert_threshold_kg }} kg</p>
              </div>
              
              <div>
                <label :class="['block font-medium text-gray-700 mb-1.5', fontSm]">
                  Raw Kilos Used 
                  <span v-if="maxKilosAllowed > 0" class="text-gray-400 font-normal ml-1">
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
                  :class="['w-full border border-gray-300 rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100 bg-white', fontBase]"
                />
              </div>
              
              <div>
                <label :class="['block font-medium text-gray-700 mb-1.5', fontSm]">Skewers Produced</label>
                <input 
                  v-model.number="skewersProduced" 
                  type="number"
                  min="1"
                  :disabled="!currentStockInfo || currentStockInfo.current_stock_kg === 0"
                  required 
                  placeholder="e.g., 120" 
                  :class="['w-full border border-gray-300 rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100 bg-white', fontBase]"
                />
              </div>
              
              <div>
                <label :class="['block font-medium text-gray-700 mb-1.5', fontSm]">Staff Member</label>
                <select 
                  v-model="selectedStaff" 
                  required 
                  :class="['w-full border border-gray-300 rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white', fontBase]"
                >
                  <option v-for="staff in staffMembers" :key="staff.staff_id" :value="staff.full_name">
                    {{ staff.full_name }} ({{ staff.role }})
                  </option>
                </select>
              </div>
              
              <BaseButton 
                type="submit" 
                variant="primary"
                :disabled="!canPrep"
                :class="['w-full py-3.5 mt-2', fontBase]"
              >
                Save Prep Log
              </BaseButton>
            </form>
          </div>

          <div class="bg-white border border-gray-100 rounded-xl lg:col-span-2 flex flex-col overflow-hidden shadow-sm">
            <div class="p-5 border-b border-gray-100 bg-gray-50/50">
              <h3 :class="['font-semibold text-gray-800', fontLg]">Recent Skewering Activity</h3>
            </div>
            
            <div class="overflow-x-auto flex-1">
              <table class="w-full text-left border-collapse min-w-125">
                <thead class="bg-gray-50">
                  <tr :class="['border-b border-gray-200 text-gray-500', fontSm]">
                    <th class="py-3 px-4 font-semibold">Time</th>
                    <th class="py-3 px-4 font-semibold">Staff Member</th>
                    <th class="py-3 px-4 font-semibold">Part Used</th>
                    <th class="py-3 px-4 font-semibold">Kilos Deducted</th>
                    <th class="py-3 px-4 font-semibold text-right">Skewers Added</th>
                  </tr>
                </thead>
                <tbody class="text-gray-700">
                  <tr v-if="recentLogs.length === 0">
                    <td colspan="5" class="py-12 text-center text-gray-500">No prep activity logged yet today.</td>
                  </tr>
                  <tr v-for="(log, index) in recentLogs" :key="index" class="border-b border-gray-50 hover:bg-gray-50/80 transition-colors">
                    <td :class="['py-3.5 px-4 text-gray-500', fontSm]">{{ log.time }}</td>
                    <td :class="['py-3.5 px-4 font-medium text-gray-900', fontBase]">{{ log.staff }}</td>
                    <td :class="['py-3.5 px-4', fontBase]">{{ log.part }}</td>
                    <td :class="['py-3.5 px-4 text-red-600 font-bold', fontBase]">{{ log.kilos }}</td>
                    <td :class="['py-3.5 px-4 text-green-600 font-bold text-right', fontBase]">{{ log.sticks }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

        </div>
      </div>
    </div>
  </div>
</template>