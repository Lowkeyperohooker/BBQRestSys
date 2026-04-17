<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import type { RawInventoryItem } from '../../services/inventoryService';
import { inventoryService } from '../../services/inventoryService';
import BaseButton from './BaseButton.vue';

const props = defineProps<{
  isOpen: boolean;
  item: RawInventoryItem | null;
}>();

const emit = defineEmits<{
  close: [];
  save: [data: { 
    isNew: boolean; 
    itemId?: number; 
    kilos: number; 
    category: string; 
    part: string; 
    alertThreshold?: number 
  }];
}>();

// Form modes
const mode = ref<'existing' | 'new'>('existing');

// Existing Item State
const kilosToAdd = ref<number | "">(""); 
const selectedItemId = ref<number | "">("");
const allItems = ref<RawInventoryItem[]>([]);

// New Item State
const newCategory = ref('');
const newPart = ref('');
const newAlertThreshold = ref<number | "">(5.0);

const uniqueCategories = computed(() => {
  return Array.from(new Set(allItems.value.map(i => i.category)));
});

const selectedItem = computed(() => {
  if (props.item) return props.item;
  if (selectedItemId.value !== "") {
    return allItems.value.find(i => i.raw_item_id === Number(selectedItemId.value)) || null;
  }
  return null;
});

const isFormInvalid = computed(() => {
  const kilos = Number(kilosToAdd.value);
  if (isNaN(kilos) || kilos <= 0) return true;

  if (mode.value === 'existing') {
    return !selectedItem.value;
  } else {
    return !newCategory.value.trim() || !newPart.value.trim() || Number(newAlertThreshold.value) < 0;
  }
});

async function loadItems() {
  try {
    allItems.value = await inventoryService.getRawInventory();
  } catch (error) {
    console.error("Failed to load items:", error);
  }
}

watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    kilosToAdd.value = "";
    selectedItemId.value = "";
    newCategory.value = '';
    newPart.value = '';
    newAlertThreshold.value = 5.0;
    mode.value = props.item ? 'existing' : 'existing';
    loadItems();
  }
});

function handleSubmit() {
  if (isFormInvalid.value) return;
  
  if (mode.value === 'existing' && selectedItem.value) {
    emit('save', {
      isNew: false,
      itemId: selectedItem.value.raw_item_id,
      kilos: Number(kilosToAdd.value),
      category: selectedItem.value.category,
      part: selectedItem.value.specific_part
    });
  } else if (mode.value === 'new') {
    emit('save', {
      isNew: true,
      kilos: Number(kilosToAdd.value),
      category: newCategory.value.trim(),
      part: newPart.value.trim(),
      alertThreshold: Number(newAlertThreshold.value)
    });
  }
}

function quickAdd(amount: number) {
  const current = Number(kilosToAdd.value) || 0;
  kilosToAdd.value = current + amount;
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 bg-gray-900/50 z-50 flex items-center justify-center backdrop-blur-sm" @click.self="$emit('close')">
    <div class="bg-white rounded-xl shadow-xl w-full max-w-md p-6 m-4 max-h-[90vh] overflow-y-auto">
      
      <div class="flex justify-between items-center mb-6">
        <div>
          <h3 class="text-lg font-bold text-gray-800">Add Raw Meat Stock</h3>
          <p class="text-sm text-gray-500 mt-1">Restock or register raw inventory</p>
        </div>
        <button type="button" @click="$emit('close')" class="text-gray-400 hover:text-gray-600 transition-colors">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="space-y-5">
        
        <div v-if="!props.item" class="flex bg-gray-100 rounded-lg p-1">
          <button 
            type="button"
            @click="mode = 'existing'"
            :class="mode === 'existing' ? 'bg-white shadow text-gray-800' : 'text-gray-500 hover:text-gray-700'"
            class="flex-1 px-4 py-2 text-sm font-medium rounded-md transition-all"
          >
            Restock Existing
          </button>
          <button 
            type="button"
            @click="mode = 'new'"
            :class="mode === 'new' ? 'bg-white shadow text-gray-800' : 'text-gray-500 hover:text-gray-700'"
            class="flex-1 px-4 py-2 text-sm font-medium rounded-md transition-all"
          >
            Create New Item
          </button>
        </div>

        <div v-if="mode === 'existing'" class="space-y-4">
          <div v-if="!props.item">
            <label class="block text-sm font-medium text-gray-700 mb-2">Select Item to Restock</label>
            <select v-model="selectedItemId" required class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none bg-white">
              <option value="" disabled>Select an item</option>
              <option v-for="i in allItems" :key="i.raw_item_id" :value="i.raw_item_id">
                {{ i.category }} - {{ i.specific_part }} ({{ i.current_stock_kg.toFixed(2) }} kg)
              </option>
            </select>
          </div>
          
          <div v-if="selectedItem" class="bg-blue-50 p-4 rounded-lg border border-blue-100">
            <p class="text-xs text-blue-600 font-medium uppercase tracking-wide mb-2">Adding Stock To</p>
            <div class="flex items-center justify-between">
              <div>
                <p class="text-lg font-bold text-gray-900">{{ selectedItem.category }}</p>
                <p class="text-base text-gray-700">{{ selectedItem.specific_part }}</p>
              </div>
              <div class="text-right">
                <p class="text-xs text-gray-600">Current Stock</p>
                <p class="text-xl font-bold" :class="selectedItem.current_stock_kg <= selectedItem.alert_threshold_kg ? 'text-red-600' : 'text-green-600'">
                  {{ selectedItem.current_stock_kg.toFixed(2) }} kg
                </p>
              </div>
            </div>
          </div>
        </div>

        <div v-else class="space-y-4 bg-gray-50 p-4 rounded-lg border border-gray-200">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Meat Category <span class="text-red-500">*</span></label>
            <input v-model="newCategory" type="text" list="category-list" required placeholder="e.g., Beef, Chicken" class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none bg-white">
            <datalist id="category-list">
              <option v-for="cat in uniqueCategories" :key="cat" :value="cat"></option>
            </datalist>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Specific Part / Cut <span class="text-red-500">*</span></label>
            <input v-model="newPart" type="text" required placeholder="e.g., Brisket, Wings" class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none bg-white">
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Low Stock Alert Threshold (kg)</label>
            <input v-model.number="newAlertThreshold" type="number" step="0.1" min="0" required class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none bg-white">
            <p class="text-xs text-gray-500 mt-1">System will alert when stock falls below this number.</p>
          </div>
        </div>
        
        <div class="pt-2 border-t border-gray-100">
          <div class="mb-3">
            <label class="block text-sm font-medium text-gray-700 mb-2">Quick Add</label>
            <div class="grid grid-cols-4 gap-2">
              <button type="button" @click="quickAdd(1)" class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors">+1 kg</button>
              <button type="button" @click="quickAdd(5)" class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors">+5 kg</button>
              <button type="button" @click="quickAdd(10)" class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors">+10 kg</button>
              <button type="button" @click="quickAdd(25)" class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors">+25 kg</button>
            </div>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">{{ mode === 'new' ? 'Initial Stock (kg)' : 'Kilograms to Add' }} <span class="text-red-500">*</span></label>
            <div class="relative">
              <input v-model.number="kilosToAdd" type="number" step="0.1" min="0.1" required class="w-full border border-gray-300 rounded-lg px-4 py-3 text-lg focus:ring-2 focus:ring-blue-500 outline-none" placeholder="Enter amount in kg" />
              <span class="absolute right-4 top-1/2 -translate-y-1/2 text-gray-400 font-medium">kg</span>
            </div>
          </div>
        </div>
        
        <div class="flex gap-3 pt-4">
          <BaseButton type="button" variant="secondary" class="flex-1" @click="$emit('close')">Cancel</BaseButton>
          <BaseButton type="submit" variant="success" class="flex-1" :disabled="isFormInvalid">
            <svg class="w-5 h-5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            {{ mode === 'new' ? 'Create & Add Stock' : 'Add Stock' }}
          </BaseButton>
        </div>

      </form>
    </div>
  </div>
</template>