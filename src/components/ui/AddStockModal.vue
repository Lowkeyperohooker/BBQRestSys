<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import type { RawInventoryItem } from '../../services/inventoryService';
import { inventoryService } from '../../services/inventoryService';
import BaseButton from './BaseButton.vue';

// Props
const props = defineProps<{
  isOpen: boolean;
  item: RawInventoryItem | null;
}>();

// Emits
const emit = defineEmits<{
  close: [];
  save: [data: { itemId: number; kilos: number; category: string; part: string }];
}>();

// Form state
const kilosToAdd = ref<number | "">(""); 
const notes = ref('');
const allItems = ref<RawInventoryItem[]>([]);
const selectedItemId = ref<number | "">("");

// 1. Bulletproof Item Selection: Automatically updates based on props OR dropdown
const selectedItem = computed(() => {
  if (props.item) return props.item;
  if (selectedItemId.value !== "") {
    return allItems.value.find(i => i.raw_item_id === Number(selectedItemId.value)) || null;
  }
  return null;
});

// 2. Rock-Solid Validation: Disables button if missing item or invalid kilos
const isFormInvalid = computed(() => {
  const kilos = Number(kilosToAdd.value);
  return !selectedItem.value || isNaN(kilos) || kilos <= 0;
});

async function loadItems() {
  try {
    allItems.value = await inventoryService.getRawInventory();
  } catch (error) {
    console.error("Failed to load items:", error);
  }
}

// Reset form perfectly every time the modal opens
watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    kilosToAdd.value = "";
    notes.value = '';
    selectedItemId.value = "";
    loadItems();
  }
});

function handleSubmit() {
  if (isFormInvalid.value || !selectedItem.value) return;
  
  emit('save', {
    itemId: selectedItem.value.raw_item_id,
    kilos: Number(kilosToAdd.value),
    category: selectedItem.value.category,
    part: selectedItem.value.specific_part
  });
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
          <p class="text-sm text-gray-500 mt-1">Restock raw inventory items</p>
        </div>
        <button type="button" @click="$emit('close')" class="text-gray-400 hover:text-gray-600 transition-colors">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="space-y-4">
        
        <div v-if="!props.item" class="mb-2">
          <label class="block text-sm font-medium text-gray-700 mb-2">Select Item to Restock</label>
          <select v-model="selectedItemId" required class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none bg-white">
            <option value="" disabled>Select an item</option>
            <option v-for="i in allItems" :key="i.raw_item_id" :value="i.raw_item_id">
              {{ i.category }} - {{ i.specific_part }} ({{ i.current_stock_kg.toFixed(2) }} kg)
            </option>
          </select>
        </div>
        
        <div v-if="selectedItem" class="space-y-4">
          <div class="bg-blue-50 p-4 rounded-lg border border-blue-100">
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
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Quick Add</label>
            <div class="grid grid-cols-4 gap-2">
              <button type="button" @click="quickAdd(1)" class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors">+1 kg</button>
              <button type="button" @click="quickAdd(5)" class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors">+5 kg</button>
              <button type="button" @click="quickAdd(10)" class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors">+10 kg</button>
              <button type="button" @click="quickAdd(25)" class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors">+25 kg</button>
            </div>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Kilograms to Add <span class="text-red-500">*</span></label>
            <div class="relative">
              <input v-model.number="kilosToAdd" type="number" step="0.1" min="0.1" required class="w-full border border-gray-300 rounded-lg px-4 py-3 text-lg focus:ring-2 focus:ring-blue-500 outline-none" placeholder="Enter amount in kg" />
              <span class="absolute right-4 top-1/2 -translate-y-1/2 text-gray-400 font-medium">kg</span>
            </div>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Notes (Optional)</label>
            <textarea v-model="notes" rows="2" placeholder="e.g., Delivery from supplier" class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none resize-none"></textarea>
          </div>
        </div>
        
        <div class="flex gap-3 pt-6">
          <BaseButton type="button" variant="secondary" class="flex-1" @click="$emit('close')">Cancel</BaseButton>
          <BaseButton type="submit" variant="success" class="flex-1" :disabled="isFormInvalid">
            <svg class="w-5 h-5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            Add Stock
          </BaseButton>
        </div>

      </form>
    </div>
  </div>
</template>