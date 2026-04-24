<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { inventoryService, type RawInventoryItem, type PreparedInventoryItem, type POSCategory } from '../../services/inventoryService';
import BaseButton from './BaseButton.vue';

const props = defineProps<{ isOpen: boolean }>();
const emit = defineEmits<{ (e: 'close'): void; (e: 'refresh'): void }>();

// Data State
const rawItems = ref<RawInventoryItem[]>([]);
const prepItems = ref<PreparedInventoryItem[]>([]);
const posCategories = ref<POSCategory[]>([]);

const internalTab = ref<'adjust' | 'new_item' | 'categories'>('adjust');

// -- ADJUST STOCK STATE --
const adjustType = ref<'raw' | 'prepared'>('raw');
const adjustItemId = ref<number | "">("");
const adjustAmount = ref<number | "">("");
const adjustReason = ref('Delivery');

// -- NEW ITEM STATE --
const newItemType = ref<'raw' | 'prepared'>('raw');
// Raw
const newRawCategory = ref('');
const newRawPart = ref('');
const newRawAlert = ref(5.0);
// Prepared (Direct Sell like Beverages)
const newPrepCategory = ref('');
const newPrepName = ref('');
const newPrepPrice = ref<number | "">("");
const newPrepVariable = ref(false);

// -- CATEGORY STATE --
const newCategoryName = ref('');

async function loadData() {
  try {
    rawItems.value = await inventoryService.getRawInventory();
    prepItems.value = await inventoryService.getPreparedInventory();
    posCategories.value = await inventoryService.getPosCategories();
  } catch (error) {
    console.error("Failed to load inventory:", error);
  }
}

watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    internalTab.value = 'adjust';
    adjustAmount.value = "";
    adjustItemId.value = "";
    newPrepPrice.value = "";
    loadData();
  }
});

async function handleAdjustStock() {
  if (adjustItemId.value === "" || adjustAmount.value === "") return;
  try {
    await inventoryService.editStock(adjustType.value, Number(adjustItemId.value), Number(adjustAmount.value), adjustReason.value);
    alert("Stock adjusted successfully.");
    adjustAmount.value = "";
    loadData();
    emit('refresh');
  } catch (error) {
    alert("Failed to adjust stock.");
  }
}

async function handleAddNewItem() {
  try {
    if (newItemType.value === 'raw') {
      if (!newRawCategory.value || !newRawPart.value) return;
      await inventoryService.addNewRawItem(newRawCategory.value, newRawPart.value, 0, newRawAlert.value);
    } else {
      if (!newPrepCategory.value || !newPrepName.value || newPrepPrice.value === "") return;
      await inventoryService.addPreparedItem(newPrepCategory.value, newPrepName.value, Number(newPrepPrice.value), newPrepVariable.value);
    }
    alert("Item added successfully.");
    newRawCategory.value = ''; newRawPart.value = ''; newPrepName.value = ''; newPrepPrice.value = '';
    loadData();
    emit('refresh');
  } catch (error) {
    alert("Failed to add new item.");
  }
}

async function handleAddCategory() {
  if (!newCategoryName.value.trim()) return;
  try {
    await inventoryService.addPosCategory(newCategoryName.value.trim());
    newCategoryName.value = '';
    loadData();
    emit('refresh');
  } catch (error) {
    alert("Failed to add category. It may already exist.");
  }
}

async function handleRemoveCategory(name: string) {
  if (!confirm(`Permanently delete the ${name} category? Items inside must be deleted first.`)) return;
  try {
    await inventoryService.removePosCategory(name);
    loadData();
    emit('refresh');
  } catch (error) {
    alert("Failed to remove category. Ensure no items are using this category first.");
  }
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 bg-gray-900/60 z-50 flex items-center justify-center backdrop-blur-sm p-4" @click.self="$emit('close')">
    <div class="bg-white rounded-2xl shadow-2xl w-full max-w-lg overflow-hidden flex flex-col max-h-[90vh]">
      
      <div class="p-5 border-b border-gray-100 bg-gray-50 flex justify-between items-center shrink-0">
        <div>
          <h3 class="text-xl font-black text-gray-900 tracking-tight">Edit Inventory</h3>
          <p class="text-sm font-medium text-gray-500 mt-0.5">Adjust stock or modify catalog</p>
        </div>
        <button type="button" @click="$emit('close')" class="text-gray-400 hover:text-gray-600 hover:bg-gray-200 p-2 rounded-full transition-colors">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>
      
      <div class="flex border-b border-gray-200 bg-white shrink-0">
        <button @click="internalTab = 'adjust'" :class="[internalTab === 'adjust' ? 'border-blue-600 text-blue-600 font-bold' : 'border-transparent text-gray-500 hover:text-gray-700', 'flex-1 py-3 border-b-2 text-sm transition-colors']">Adjust Stock</button>
        <button @click="internalTab = 'new_item'" :class="[internalTab === 'new_item' ? 'border-blue-600 text-blue-600 font-bold' : 'border-transparent text-gray-500 hover:text-gray-700', 'flex-1 py-3 border-b-2 text-sm transition-colors']">Add Item</button>
        <button @click="internalTab = 'categories'" :class="[internalTab === 'categories' ? 'border-blue-600 text-blue-600 font-bold' : 'border-transparent text-gray-500 hover:text-gray-700', 'flex-1 py-3 border-b-2 text-sm transition-colors']">Manage Tabs</button>
      </div>

      <div class="flex-1 overflow-y-auto p-6 bg-white">
        
        <form v-if="internalTab === 'adjust'" @submit.prevent="handleAdjustStock" class="space-y-5">
          <div class="flex gap-2 p-1 bg-gray-100 rounded-lg">
            <button type="button" @click="adjustType = 'raw'; adjustItemId = ''" :class="[adjustType === 'raw' ? 'bg-white shadow text-gray-900 font-bold' : 'text-gray-500', 'flex-1 py-2 text-sm rounded-md transition-all']">Raw Meat (kg)</button>
            <button type="button" @click="adjustType = 'prepared'; adjustItemId = ''" :class="[adjustType === 'prepared' ? 'bg-white shadow text-gray-900 font-bold' : 'text-gray-500', 'flex-1 py-2 text-sm rounded-md transition-all']">POS Item (pcs)</button>
          </div>

          <div>
            <label class="block text-sm font-bold text-gray-700 mb-1.5">Select Item</label>
            <select v-model="adjustItemId" required class="w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-medium focus:border-blue-500 outline-none">
              <option value="" disabled>-- Select --</option>
              <template v-if="adjustType === 'raw'">
                <option v-for="i in rawItems" :key="i.raw_item_id" :value="i.raw_item_id">{{ i.category }} - {{ i.specific_part }} ({{ i.current_stock_kg }} kg)</option>
              </template>
              <template v-else>
                <option v-for="i in prepItems" :key="i.prep_item_id" :value="i.prep_item_id">{{ i.category }} > {{ i.pos_display_name }} ({{ i.current_stock_pieces }} pcs)</option>
              </template>
            </select>
          </div>

          <div class="flex gap-4">
            <div class="flex-1">
              <label class="block text-sm font-bold text-gray-700 mb-1.5">Amount (+ or -)</label>
              <input v-model.number="adjustAmount" type="number" step="0.1" required placeholder="e.g. 5 or -2" class="w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-bold focus:border-blue-500 outline-none" />
            </div>
            <div class="flex-1">
              <label class="block text-sm font-bold text-gray-700 mb-1.5">Reason</label>
              <select v-model="adjustReason" required class="w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-medium focus:border-blue-500 outline-none">
                <option>Delivery</option>
                <option>Manual Correction</option>
                <option>Spoilage / Waste</option>
              </select>
            </div>
          </div>
          
          <BaseButton type="submit" variant="primary" class="w-full py-3.5 mt-2">Save Adjustment</BaseButton>
        </form>

        <form v-else-if="internalTab === 'new_item'" @submit.prevent="handleAddNewItem" class="space-y-5">
          <div class="flex gap-2 p-1 bg-gray-100 rounded-lg">
            <button type="button" @click="newItemType = 'raw'" :class="[newItemType === 'raw' ? 'bg-white shadow text-gray-900 font-bold' : 'text-gray-500', 'flex-1 py-2 text-sm rounded-md transition-all']">Raw Meat Ingredient</button>
            <button type="button" @click="newItemType = 'prepared'" :class="[newItemType === 'prepared' ? 'bg-white shadow text-gray-900 font-bold' : 'text-gray-500', 'flex-1 py-2 text-sm rounded-md transition-all']">Direct POS Item</button>
          </div>

          <template v-if="newItemType === 'raw'">
            <div>
              <label class="block text-sm font-bold text-gray-700 mb-1.5">Category (e.g. Pork, Beef)</label>
              <input v-model="newRawCategory" type="text" required class="w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-medium focus:border-blue-500 outline-none" />
            </div>
            <div>
              <label class="block text-sm font-bold text-gray-700 mb-1.5">Specific Cut</label>
              <input v-model="newRawPart" type="text" required class="w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-medium focus:border-blue-500 outline-none" />
            </div>
            <div>
              <label class="block text-sm font-bold text-gray-700 mb-1.5">Alert Threshold (kg)</label>
              <input v-model.number="newRawAlert" type="number" step="0.1" required class="w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-medium focus:border-blue-500 outline-none" />
            </div>
          </template>

          <template v-else>
            <div>
              <label class="block text-sm font-bold text-gray-700 mb-1.5">Menu Tab (Category)</label>
              <select v-model="newPrepCategory" required class="w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-medium focus:border-blue-500 outline-none">
                <option value="" disabled>-- Select Tab --</option>
                <option v-for="cat in posCategories" :key="cat.category_name" :value="cat.category_name">{{ cat.category_name }}</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-bold text-gray-700 mb-1.5">Display Name</label>
              <input v-model="newPrepName" type="text" required placeholder="e.g. Bottled Coke" class="w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-medium focus:border-blue-500 outline-none" />
            </div>
            <div class="flex items-center gap-4">
              <div class="flex-1">
                <label class="block text-sm font-bold text-gray-700 mb-1.5">Base Price (PHP)</label>
                <input v-model.number="newPrepPrice" type="number" step="0.01" required class="w-full border-2 border-gray-200 rounded-xl px-4 py-3 font-medium focus:border-blue-500 outline-none" />
              </div>
              <div class="flex items-center mt-6">
                <input v-model="newPrepVariable" type="checkbox" id="varPrice" class="w-5 h-5 text-blue-600 rounded focus:ring-blue-500" />
                <label for="varPrice" class="ml-2 text-sm font-bold text-gray-700">Variable Price</label>
              </div>
            </div>
          </template>

          <BaseButton type="submit" variant="primary" class="w-full py-3.5 mt-2">Add Item</BaseButton>
        </form>

        <div v-else class="space-y-6">
          <div>
            <label class="block text-sm font-bold text-gray-700 mb-2">Existing Menu Tabs</label>
            <div class="border-2 border-gray-100 rounded-xl divide-y divide-gray-100">
              <div v-for="cat in posCategories" :key="cat.category_name" class="flex justify-between items-center p-3 hover:bg-gray-50">
                <span class="font-bold text-gray-800">{{ cat.category_name }}</span>
                <button v-if="cat.is_removable" @click="handleRemoveCategory(cat.category_name)" class="text-xs font-bold text-red-500 hover:text-red-700 bg-red-50 px-2 py-1 rounded">Delete</button>
                <span v-else class="text-xs font-bold text-gray-400 bg-gray-100 px-2 py-1 rounded">Locked</span>
              </div>
            </div>
          </div>
          
          <form @submit.prevent="handleAddCategory" class="pt-4 border-t border-gray-100">
            <label class="block text-sm font-bold text-gray-700 mb-1.5">Create New Tab</label>
            <div class="flex gap-2">
              <input v-model="newCategoryName" type="text" required placeholder="e.g. Beverages" class="flex-1 border-2 border-gray-200 rounded-xl px-4 py-3 font-medium focus:border-blue-500 outline-none" />
              <BaseButton type="submit" variant="success" class="px-6">Add</BaseButton>
            </div>
          </form>
        </div>

      </div>
    </div>
  </div>
</template>