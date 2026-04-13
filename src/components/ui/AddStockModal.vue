<script setup lang="ts">
import { ref, watch } from 'vue';
import type { RawInventoryItem } from '../../services/inventoryService';

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
const kilosToAdd = ref<number>(0);
const notes = ref('');

// Reset form when modal opens
watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    kilosToAdd.value = 0;
    notes.value = '';
  }
});

// Handle submit
function handleSubmit() {
  if (!props.item || kilosToAdd.value <= 0) {
    return;
  }
  
  emit('save', {
    itemId: props.item.raw_item_id,
    kilos: kilosToAdd.value,
    category: props.item.category,
    part: props.item.specific_part
  });
}

// Handle close
function handleClose() {
  emit('close');
}

// Quick add buttons
function quickAdd(amount: number) {
  kilosToAdd.value = amount;
}
</script>

<template>
  <div 
    v-if="isOpen" 
    class="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center backdrop-blur-sm"
    @click.self="handleClose"
  >
    <div class="bg-white rounded-xl shadow-xl w-full max-w-md p-6 m-4">
      
      <!-- Header -->
      <div class="flex justify-between items-center mb-6">
        <div>
          <h3 class="text-lg font-bold text-gray-800">Add Raw Meat Stock</h3>
          <p class="text-sm text-gray-500 mt-1">Restock raw inventory items</p>
        </div>
        <button 
          @click="handleClose" 
          class="text-gray-400 hover:text-gray-600 transition-colors"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div>
      
      <!-- Item Info -->
      <div v-if="item" class="space-y-4">
        <!-- Selected Item Card -->
        <div class="bg-linear-to-r from-blue-50 to-indigo-50 p-4 rounded-lg border border-blue-100">
          <p class="text-xs text-blue-600 font-medium uppercase tracking-wide mb-2">Adding Stock To</p>
          <div class="flex items-center justify-between">
            <div>
              <p class="text-lg font-bold text-gray-900">{{ item.category }}</p>
              <p class="text-base text-gray-700">{{ item.specific_part }}</p>
            </div>
            <div class="text-right">
              <p class="text-xs text-gray-600">Current Stock</p>
              <p class="text-xl font-bold" :class="item.current_stock_kg <= item.alert_threshold_kg ? 'text-red-600' : 'text-green-600'">
                {{ item.current_stock_kg.toFixed(2) }} kg
              </p>
            </div>
          </div>
          <div class="mt-3 pt-3 border-t border-blue-200">
            <p class="text-sm text-gray-600">
              <span class="font-medium">Alert Threshold:</span> {{ item.alert_threshold_kg }} kg
            </p>
          </div>
        </div>
        
        <!-- Quick Add Buttons -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Quick Add</label>
          <div class="grid grid-cols-4 gap-2">
            <button 
              type="button"
              @click="quickAdd(1)"
              class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors"
            >
              +1 kg
            </button>
            <button 
              type="button"
              @click="quickAdd(5)"
              class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors"
            >
              +5 kg
            </button>
            <button 
              type="button"
              @click="quickAdd(10)"
              class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors"
            >
              +10 kg
            </button>
            <button 
              type="button"
              @click="quickAdd(25)"
              class="px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors"
            >
              +25 kg
            </button>
          </div>
        </div>
        
        <!-- Kilos Input -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Kilograms to Add <span class="text-red-500">*</span>
          </label>
          <div class="relative">
            <input 
              v-model.number="kilosToAdd" 
              type="number" 
              step="0.1"
              min="0.1"
              required
              class="w-full border border-gray-300 rounded-lg px-4 py-3 text-lg focus:ring-2 focus:ring-blue-500 outline-none"
              placeholder="Enter amount in kg"
            />
            <span class="absolute right-4 top-1/2 -translate-y-1/2 text-gray-400 font-medium">kg</span>
          </div>
        </div>
        
        <!-- New Stock Preview -->
        <div v-if="kilosToAdd > 0" class="bg-green-50 p-4 rounded-lg border border-green-200">
          <p class="text-sm text-green-700 font-medium mb-1">New Stock Level</p>
          <div class="flex items-center justify-between">
            <span class="text-gray-600">After adding:</span>
            <span class="text-2xl font-bold text-green-700">
              {{ (item.current_stock_kg + kilosToAdd).toFixed(2) }} kg
            </span>
          </div>
        </div>
        
        <!-- Optional Notes -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Notes (Optional)
          </label>
          <textarea 
            v-model="notes" 
            rows="2"
            placeholder="e.g., Delivery from supplier, invoice #123"
            class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none resize-none"
          ></textarea>
        </div>
      </div>
      
      <!-- Footer Actions -->
      <div class="flex gap-3 pt-6">
        <button 
          type="button"
          @click="handleClose" 
          class="flex-1 bg-white border border-gray-300 text-gray-700 px-4 py-3 rounded-lg hover:bg-gray-50 transition-colors font-medium"
        >
          Cancel
        </button>
        <button 
          type="button"
          @click="handleSubmit" 
          :disabled="!item || kilosToAdd <= 0"
          class="flex-1 bg-linear-to-r from-green-600 to-emerald-600 hover:from-green-700 hover:to-emerald-700 text-white px-4 py-3 rounded-lg transition-all font-semibold disabled:opacity-50 disabled:cursor-not-allowed shadow-md hover:shadow-lg"
        >
          <span class="flex items-center justify-center gap-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            Add Stock
          </span>
        </button>
      </div>
      
      <!-- Help Text -->
      <p class="text-xs text-gray-500 text-center mt-4">
        Raw meat inventory is measured in kilograms (kg).<br>
        Prepared skewers can only be added through the Prep Station.
      </p>
    </div>
  </div>
</template>