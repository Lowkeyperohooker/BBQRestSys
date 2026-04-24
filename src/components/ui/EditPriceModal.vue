<script setup lang="ts">
import { ref, watch } from 'vue';
import { inventoryService, type PreparedInventoryItem } from '../../services/inventoryService';
import BaseButton from './BaseButton.vue';

const props = defineProps<{
  isOpen: boolean;
  item: PreparedInventoryItem | null;
}>();

const emit = defineEmits<{
  close: [];
  save: [data: { prepItemId: number; unitPrice: number; isVariablePrice: boolean; photoUrl: string | null }];
}>();

const formPrice = ref<number | "">("");
const formIsVariable = ref<boolean>(false);

const fileInput = ref<HTMLInputElement | null>(null);
const selectedFile = ref<File | null>(null);
const previewUrl = ref<string | null>(null);
const originalPhotoUrl = ref<string | null>(null);
const isDragging = ref(false);

watch(() => props.isOpen, (newVal) => {
  if (newVal && props.item) {
    formPrice.value = props.item.unit_price;
    formIsVariable.value = props.item.is_variable_price;
    
    selectedFile.value = null;
    const existingPhoto = props.item.photo_url || null;
    originalPhotoUrl.value = existingPhoto;
    
    // If the image is coming from the backend, prefix it with localhost:3000 to preview correctly
    previewUrl.value = existingPhoto ? `http://localhost:3000${existingPhoto}` : null;
  }
});

function handleFileSelect(event: Event) {
  const target = event.target as HTMLInputElement;
  if (target.files && target.files[0]) setFile(target.files[0]);
}

function handleDrop(event: DragEvent) {
  isDragging.value = false;
  if (event.dataTransfer?.files && event.dataTransfer.files[0]) setFile(event.dataTransfer.files[0]);
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
  originalPhotoUrl.value = null; // Removing implies deleting from DB too
  if (fileInput.value) fileInput.value.value = '';
}

async function handleSubmit() {
  if (!props.item || formPrice.value === "") return;
  
  let finalPhotoUrl = originalPhotoUrl.value;
  
  if (selectedFile.value) {
    try {
      finalPhotoUrl = await inventoryService.uploadPhoto(selectedFile.value);
    } catch(e) {
      alert("Failed to upload new photo.");
      return;
    }
  }

  emit('save', {
    prepItemId: props.item.prep_item_id,
    unitPrice: Number(formPrice.value),
    isVariablePrice: formIsVariable.value,
    photoUrl: finalPhotoUrl
  });
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 bg-gray-900/50 z-50 flex items-center justify-center backdrop-blur-sm p-4" @click.self="$emit('close')">
    <div class="bg-white rounded-xl shadow-xl w-full max-w-sm p-6">
      
      <div class="flex justify-between items-center mb-6">
        <div>
          <h3 class="text-lg font-bold text-gray-800">Edit Item Details</h3>
          <p v-if="item" class="text-sm text-gray-500 mt-1">{{ item.pos_display_name }}</p>
        </div>
        <button type="button" @click="$emit('close')" class="text-gray-400 hover:text-gray-600 transition-colors">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="space-y-5">
        
        <div>
          <label class="block text-sm font-bold text-gray-700 mb-1.5">Item Photo</label>
          <div 
            @dragover.prevent="isDragging = true"
            @dragleave.prevent="isDragging = false"
            @drop.prevent="handleDrop"
            @click="!previewUrl ? fileInput?.click() : null"
            :class="[
              'w-full border-2 border-dashed rounded-xl p-3 flex flex-col items-center justify-center transition-colors',
              isDragging ? 'border-blue-500 bg-blue-50' : 'border-gray-300 hover:bg-gray-50',
              !previewUrl ? 'cursor-pointer min-h-24' : ''
            ]"
          >
            <input type="file" ref="fileInput" class="hidden" accept="image/*" @change="handleFileSelect" />
            
            <div v-if="previewUrl" class="relative w-full">
              <img :src="previewUrl" class="max-h-32 mx-auto rounded-lg shadow-sm" />
              <button type="button" @click.stop="removeFile" class="absolute -top-2 -right-2 bg-red-500 text-white rounded-full p-1 hover:bg-red-600 shadow-md">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
              </button>
            </div>

            <div v-else class="text-center">
              <svg class="mx-auto h-6 w-6 text-gray-400" stroke="currentColor" fill="none" viewBox="0 0 48 48"><path d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" /></svg>
              <p class="mt-1 text-xs text-gray-500 font-medium">Drag & drop or click</p>
            </div>
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Base Unit Price (PHP)</label>
          <input v-model.number="formPrice" type="number" step="0.01" min="0" required class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none bg-white">
        </div>

        <div class="bg-gray-50 p-4 rounded-lg border border-gray-200">
          <label class="block text-sm font-medium text-gray-700 mb-2">Pricing Type</label>
          <div class="space-y-2">
            <label class="flex items-center gap-3 cursor-pointer">
              <input v-model="formIsVariable" type="radio" :value="false" class="w-4 h-4 text-blue-600 focus:ring-blue-500">
              <span class="text-sm text-gray-800">Fixed Price</span>
            </label>
            <label class="flex items-center gap-3 cursor-pointer">
              <input v-model="formIsVariable" type="radio" :value="true" class="w-4 h-4 text-blue-600 focus:ring-blue-500">
              <div>
                <span class="text-sm text-gray-800 block">Variable Price</span>
                <span class="text-xs text-gray-500">Cashier inputs price at checkout</span>
              </div>
            </label>
          </div>
        </div>

        <div class="flex gap-3 pt-2">
          <BaseButton type="button" variant="secondary" class="flex-1" @click="$emit('close')">Cancel</BaseButton>
          <BaseButton type="submit" variant="primary" class="flex-1">Save Changes</BaseButton>
        </div>

      </form>
    </div>
  </div>
</template>