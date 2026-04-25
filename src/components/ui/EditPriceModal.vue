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
  originalPhotoUrl.value = null; 
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
  <div v-if="isOpen" class="fixed inset-0 bg-black/60 z-50 flex items-center justify-center backdrop-blur-md p-4" @click.self="$emit('close')">
    <div class="bg-surface-container-low border border-outline-variant/20 rounded-2xl shadow-2xl w-full max-w-sm p-6">
      
      <div class="flex justify-between items-center mb-6">
        <div>
          <h3 class="text-lg font-bold text-on-surface tracking-tight">Edit Item Details</h3>
          <p v-if="item" class="text-sm text-on-surface-variant mt-1">{{ item.pos_display_name }}</p>
        </div>
        <button type="button" @click="$emit('close')" class="text-on-surface-variant hover:text-on-surface bg-surface-container hover:bg-surface-container-high p-2 rounded-full transition-colors active:scale-90">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="space-y-5">
        
        <div>
          <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Item Photo</label>
          <div 
            @dragover.prevent="isDragging = true"
            @dragleave.prevent="isDragging = false"
            @drop.prevent="handleDrop"
            @click="!previewUrl ? fileInput?.click() : null"
            :class="[
              'w-full border-2 border-dashed rounded-xl p-3 flex flex-col items-center justify-center transition-colors',
              isDragging ? 'border-primary-container bg-primary-container/10' : 'border-outline-variant/30 hover:bg-surface-container',
              !previewUrl ? 'cursor-pointer min-h-24' : ''
            ]"
          >
            <input type="file" ref="fileInput" class="hidden" accept="image/*" @change="handleFileSelect" />
            
            <div v-if="previewUrl" class="relative w-full">
              <img :src="previewUrl" class="max-h-32 mx-auto rounded-lg shadow-sm" />
              <button type="button" @click.stop="removeFile" class="absolute -top-2 -right-2 bg-error text-on-error rounded-full p-1 hover:bg-error/80 shadow-md">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
              </button>
            </div>

            <div v-else class="text-center">
              <svg class="mx-auto h-6 w-6 text-on-surface-variant" stroke="currentColor" fill="none" viewBox="0 0 48 48"><path d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" /></svg>
              <p class="mt-1 text-xs text-on-surface-variant font-medium">Drag & drop or click</p>
            </div>
          </div>
        </div>

        <div>
          <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Base Unit Price (PHP)</label>
          <input v-model.number="formPrice" type="number" step="0.01" min="0" required class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-4 py-3 font-medium focus:border-primary-container focus:ring-1 focus:ring-primary-container outline-none transition-colors">
        </div>

        <div class="bg-surface-container-low p-4 rounded-xl border border-outline-variant/20">
          <label class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-3">Pricing Type</label>
          <div class="space-y-3">
            <label class="flex items-center gap-3 cursor-pointer p-2 rounded hover:bg-surface-container transition-colors">
              <input v-model="formIsVariable" type="radio" :value="false" class="w-4 h-4 text-primary-container bg-surface border-outline-variant/30 focus:ring-primary-container focus:ring-offset-surface">
              <span class="text-sm font-bold text-on-surface">Fixed Price</span>
            </label>
            <label class="flex items-center gap-3 cursor-pointer p-2 rounded hover:bg-surface-container transition-colors">
              <input v-model="formIsVariable" type="radio" :value="true" class="w-4 h-4 text-primary-container bg-surface border-outline-variant/30 focus:ring-primary-container focus:ring-offset-surface">
              <div>
                <span class="text-sm font-bold text-on-surface block">Variable Price</span>
                <span class="text-xs text-on-surface-variant mt-0.5 block">Cashier inputs price at checkout</span>
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