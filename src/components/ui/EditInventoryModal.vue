<script setup lang="ts">
import { ref, watch } from 'vue';
import { inventoryService, type RawInventoryItem, type PreparedInventoryItem, type POSCategory } from '../../services/inventoryService';
import BaseButton from './BaseButton.vue';

const props = defineProps<{ isOpen: boolean }>();
const emit = defineEmits<{ (e: 'close'): void; (e: 'refresh'): void }>();

const rawItems = ref<RawInventoryItem[]>([]);
const prepItems = ref<PreparedInventoryItem[]>([]);
const posCategories = ref<POSCategory[]>([]);
const internalTab = ref<'adjust' | 'new_item' | 'categories'>('adjust');

const adjustType = ref<'raw' | 'prepared'>('raw');
const adjustItemId = ref<number | "">("");
const adjustAmount = ref<number | "">("");
const adjustReason = ref('Delivery');

const newItemType = ref<'raw' | 'prepared'>('raw');
const newRawCategory = ref('');
const newRawPart = ref('');
const newRawAlert = ref(5.0);

const newPrepCategory = ref('');
const newPrepName = ref('');
const newPrepPrice = ref<number | "">("");
const newPrepVariable = ref(false);

const fileInput = ref<HTMLInputElement | null>(null);
const selectedFile = ref<File | null>(null);
const previewUrl = ref<string | null>(null);
const isDragging = ref(false);

function handleFileSelect(event: Event) {
  const target = event.target as HTMLInputElement;
  if (target.files && target.files[0]) {
    setFile(target.files[0]);
  }
}

function handleDrop(event: DragEvent) {
  isDragging.value = false;
  if (event.dataTransfer?.files && event.dataTransfer.files[0]) {
    setFile(event.dataTransfer.files[0]);
  }
}

function setFile(file: File) {
  if (!file.type.startsWith('image/')) {
    alert('Please select an image file.');
    return;
  }
  selectedFile.value = file;
  previewUrl.value = URL.createObjectURL(file);
}

function removeFile() {
  selectedFile.value = null;
  previewUrl.value = null;
  if (fileInput.value) fileInput.value.value = '';
}

const newCategoryName = ref('');

async function loadData() {
  try {
    rawItems.value = await inventoryService.getRawInventory();
    prepItems.value = await inventoryService.getPreparedInventory();
    posCategories.value = await inventoryService.getPosCategories();
  } catch (error) { console.error("Failed to load inventory:", error); }
}

watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    internalTab.value = 'adjust';
    adjustAmount.value = "";
    adjustItemId.value = "";
    newPrepPrice.value = "";
    removeFile();
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
  } catch (error) { alert("Failed to adjust stock."); }
}

async function handleAddNewItem() {
  try {
    if (newItemType.value === 'raw') {
      if (!newRawCategory.value || !newRawPart.value) return;
      await inventoryService.addNewRawItem(newRawCategory.value, newRawPart.value, 0, newRawAlert.value);
    } else {
      if (!newPrepCategory.value || !newPrepName.value || newPrepPrice.value === "") return;
      
      let finalPhotoUrl = null;
      if (selectedFile.value) {
        finalPhotoUrl = await inventoryService.uploadPhoto(selectedFile.value);
      }

      await inventoryService.addPreparedItem(
        newPrepCategory.value, 
        newPrepName.value, 
        Number(newPrepPrice.value), 
        newPrepVariable.value,
        finalPhotoUrl
      );
    }
    alert("Item added successfully.");
    newRawCategory.value = ''; newRawPart.value = ''; newPrepName.value = ''; newPrepPrice.value = ''; 
    removeFile();
    loadData();
    emit('refresh');
  } catch (error) { alert("Failed to add new item."); }
}

async function handleAddCategory() {
  if (!newCategoryName.value.trim()) return;
  try {
    await inventoryService.addPosCategory(newCategoryName.value.trim());
    newCategoryName.value = '';
    loadData();
    emit('refresh');
  } catch (error) { alert("Failed to add category."); }
}

async function handleRemoveCategory(name: string) {
  if (!confirm(`Permanently delete the ${name} category?`)) return;
  try {
    await inventoryService.removePosCategory(name);
    loadData();
    emit('refresh');
  } catch (error) { alert("Failed to remove category."); }
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 bg-black/60 z-50 flex items-center justify-center backdrop-blur-md p-4" @click.self="$emit('close')">
    <div class="bg-surface-container-low border border-outline-variant/20 rounded-2xl shadow-2xl w-full max-w-lg overflow-hidden flex flex-col max-h-[90vh]">
      
      <div class="p-5 border-b border-outline-variant/10 bg-surface-container-highest/30 flex justify-between items-center shrink-0">
        <div>
          <h3 class="text-xl font-black text-on-surface tracking-tight">Edit Inventory</h3>
        </div>
        <button type="button" @click="$emit('close')" class="text-on-surface-variant hover:text-on-surface bg-surface-container hover:bg-surface-container-high p-2 rounded-full transition-colors active:scale-90">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>
      
      <div class="flex border-b border-outline-variant/20 bg-surface-container-low shrink-0">
        <button @click="internalTab = 'adjust'" :class="[internalTab === 'adjust' ? 'border-primary text-primary font-bold' : 'border-transparent text-on-surface-variant hover:text-on-surface', 'flex-1 py-3 border-b-2 text-sm transition-colors uppercase tracking-widest']">Adjust Stock</button>
        <button @click="internalTab = 'new_item'" :class="[internalTab === 'new_item' ? 'border-primary text-primary font-bold' : 'border-transparent text-on-surface-variant hover:text-on-surface', 'flex-1 py-3 border-b-2 text-sm transition-colors uppercase tracking-widest']">Add Item</button>
        <button @click="internalTab = 'categories'" :class="[internalTab === 'categories' ? 'border-primary text-primary font-bold' : 'border-transparent text-on-surface-variant hover:text-on-surface', 'flex-1 py-3 border-b-2 text-sm transition-colors uppercase tracking-widest']">Manage Tabs</button>
      </div>

      <div class="flex-1 overflow-y-auto p-6 bg-surface">
        
        <form v-if="internalTab === 'adjust'" @submit.prevent="handleAdjustStock" class="space-y-5">
          <div class="flex gap-2 p-1 bg-surface-container rounded-lg border border-outline-variant/10">
            <button type="button" @click="adjustType = 'raw'; adjustItemId = ''" :class="[adjustType === 'raw' ? 'bg-surface-container-high text-on-surface font-bold border border-outline-variant/20 shadow-sm' : 'text-on-surface-variant hover:text-on-surface', 'flex-1 py-2 text-sm rounded-md transition-all']">Raw Meat (kg)</button>
            <button type="button" @click="adjustType = 'prepared'; adjustItemId = ''" :class="[adjustType === 'prepared' ? 'bg-surface-container-high text-on-surface font-bold border border-outline-variant/20 shadow-sm' : 'text-on-surface-variant hover:text-on-surface', 'flex-1 py-2 text-sm rounded-md transition-all']">POS Item (pcs)</button>
          </div>

          <div>
            <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Select Item</label>
            <select v-model="adjustItemId" required class="w-full bg-surface-container text-on-surface placeholder-on-surface-variant/50 border border-outline-variant/30 rounded-xl px-4 py-3 font-medium focus:border-primary-container focus:ring-1 focus:ring-primary-container outline-none transition-colors">
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
              <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Amount (+ or -)</label>
              <input v-model.number="adjustAmount" type="number" step="0.1" required placeholder="e.g. 5 or -2" class="w-full bg-surface-container text-on-surface placeholder-on-surface-variant/50 border border-outline-variant/30 rounded-xl px-4 py-3 font-bold focus:border-primary-container focus:ring-1 focus:ring-primary-container outline-none transition-colors" />
            </div>
            <div class="flex-1">
              <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Reason</label>
              <select v-model="adjustReason" required class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-4 py-3 font-medium focus:border-primary-container focus:ring-1 focus:ring-primary-container outline-none transition-colors">
                <option>Delivery</option>
                <option>Manual Correction</option>
                <option>Spoilage / Waste</option>
              </select>
            </div>
          </div>
          <BaseButton type="submit" variant="primary" class="w-full py-4 mt-2">Save Adjustment</BaseButton>
        </form>

        <form v-else-if="internalTab === 'new_item'" @submit.prevent="handleAddNewItem" class="space-y-5">
          <div class="flex gap-2 p-1 bg-surface-container rounded-lg border border-outline-variant/10">
            <button type="button" @click="newItemType = 'raw'" :class="[newItemType === 'raw' ? 'bg-surface-container-high text-on-surface font-bold border border-outline-variant/20 shadow-sm' : 'text-on-surface-variant hover:text-on-surface', 'flex-1 py-2 text-sm rounded-md transition-all']">Raw Meat</button>
            <button type="button" @click="newItemType = 'prepared'" :class="[newItemType === 'prepared' ? 'bg-surface-container-high text-on-surface font-bold border border-outline-variant/20 shadow-sm' : 'text-on-surface-variant hover:text-on-surface', 'flex-1 py-2 text-sm rounded-md transition-all']">POS Item</button>
          </div>

          <template v-if="newItemType === 'raw'">
            <div>
              <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Category (e.g. Pork)</label>
              <input v-model="newRawCategory" type="text" required class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-4 py-3 font-medium focus:border-primary-container focus:ring-1 focus:ring-primary-container outline-none transition-colors" />
            </div>
            <div>
              <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Specific Cut</label>
              <input v-model="newRawPart" type="text" required class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-4 py-3 font-medium focus:border-primary-container focus:ring-1 focus:ring-primary-container outline-none transition-colors" />
            </div>
            <div>
              <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Alert Threshold (kg)</label>
              <input v-model.number="newRawAlert" type="number" step="0.1" required class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-4 py-3 font-medium focus:border-primary-container focus:ring-1 focus:ring-primary-container outline-none transition-colors" />
            </div>
          </template>

          <template v-else>
            <div>
              <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Menu Tab</label>
              <select v-model="newPrepCategory" required class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-4 py-3 font-medium focus:border-primary-container focus:ring-1 focus:ring-primary-container outline-none transition-colors">
                <option value="" disabled>-- Select Tab --</option>
                <option v-for="cat in posCategories" :key="cat.category_name" :value="cat.category_name">{{ cat.category_name }}</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Display Name</label>
              <input v-model="newPrepName" type="text" required placeholder="e.g. Bottled Coke" class="w-full bg-surface-container text-on-surface placeholder-on-surface-variant/50 border border-outline-variant/30 rounded-xl px-4 py-3 font-medium focus:border-primary-container focus:ring-1 focus:ring-primary-container outline-none transition-colors" />
            </div>

            <div>
              <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Item Photo</label>
              <div 
                @dragover.prevent="isDragging = true"
                @dragleave.prevent="isDragging = false"
                @drop.prevent="handleDrop"
                @click="!previewUrl ? fileInput?.click() : null"
                :class="[
                  'w-full border-2 border-dashed rounded-xl p-4 flex flex-col items-center justify-center transition-colors',
                  isDragging ? 'border-primary-container bg-primary-container/10' : 'border-outline-variant/30 hover:bg-surface-container',
                  !previewUrl ? 'cursor-pointer min-h-32' : ''
                ]"
              >
                <input type="file" ref="fileInput" class="hidden" accept="image/*" @change="handleFileSelect" />
                
                <div v-if="previewUrl" class="relative w-full">
                  <img :src="previewUrl" class="max-h-40 mx-auto rounded-lg shadow-sm" />
                  <button type="button" @click.stop="removeFile" class="absolute -top-2 -right-2 bg-error text-on-error rounded-full p-1.5 hover:bg-error/80 shadow-md">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                  </button>
                </div>

                <div v-else class="text-center">
                  <svg class="mx-auto h-8 w-8 text-on-surface-variant" stroke="currentColor" fill="none" viewBox="0 0 48 48"><path d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" /></svg>
                  <p class="mt-2 text-sm text-on-surface-variant font-medium">Drag & drop or click to browse</p>
                </div>
              </div>
            </div>

            <div class="flex items-center gap-4">
              <div class="flex-1">
                <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Base Price (PHP)</label>
                <input v-model.number="newPrepPrice" type="number" step="0.01" required class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-4 py-3 font-medium focus:border-primary-container focus:ring-1 focus:ring-primary-container outline-none transition-colors" />
              </div>
              <div class="flex items-center mt-6">
                <input v-model="newPrepVariable" type="checkbox" id="varPrice" class="w-5 h-5 text-primary-container bg-surface-container border-outline-variant/30 rounded focus:ring-primary-container focus:ring-offset-surface" />
                <label for="varPrice" class="ml-2 text-sm font-bold text-on-surface">Variable Price</label>
              </div>
            </div>
          </template>

          <BaseButton type="submit" variant="primary" class="w-full py-4 mt-2">Add Item</BaseButton>
        </form>

        <div v-else class="space-y-6">
          <div>
            <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-2">Existing Menu Tabs</label>
            <div class="border border-outline-variant/20 bg-surface-container-low rounded-xl divide-y divide-outline-variant/10">
              <div v-for="cat in posCategories" :key="cat.category_name" class="flex justify-between items-center p-3 hover:bg-surface-container transition-colors">
                <span class="font-bold text-on-surface">{{ cat.category_name }}</span>
                <button v-if="cat.is_removable" @click="handleRemoveCategory(cat.category_name)" class="text-xs font-bold text-error hover:text-error/80 bg-error/10 border border-error/20 px-3 py-1.5 rounded uppercase tracking-wider">Delete</button>
                <span v-else class="text-xs font-bold text-on-surface-variant bg-surface-container-highest border border-outline-variant/30 px-3 py-1.5 rounded uppercase tracking-wider">Locked</span>
              </div>
            </div>
          </div>
          
          <form @submit.prevent="handleAddCategory" class="pt-4 border-t border-outline-variant/20">
            <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Create New Tab</label>
            <div class="flex gap-2">
              <input v-model="newCategoryName" type="text" required placeholder="e.g. Beverages" class="flex-1 bg-surface-container text-on-surface placeholder-on-surface-variant/50 border border-outline-variant/30 rounded-xl px-4 py-3 font-medium focus:border-primary-container focus:ring-1 focus:ring-primary-container outline-none transition-colors" />
              <BaseButton type="submit" variant="success" class="px-6">Add</BaseButton>
            </div>
          </form>
        </div>

      </div>
    </div>
  </div>
</template>
