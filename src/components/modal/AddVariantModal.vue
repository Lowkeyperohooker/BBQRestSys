<script setup lang="ts">
import { ref, watch } from 'vue';
import { inventoryService } from '../../services/inventoryService';
import BaseButton from '../ui/BaseButton.vue';

const props = defineProps<{ isOpen: boolean; baseItemName: string | null }>();
const emit = defineEmits<{ (e: 'close'): void; (e: 'save', payload: { name: string; price: number; photoUrl: string | null }): void; }>();

const variantName = ref('');
const variantPrice = ref<number | "">("");

const fileInput = ref<HTMLInputElement | null>(null);
const selectedFile = ref<File | null>(null);
const previewUrl = ref<string | null>(null);

watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    variantName.value = '';
    variantPrice.value = '';
    removeFile();
  }
});

function handleFileSelect(e: Event) {
  const target = e.target as HTMLInputElement;
  if (target.files && target.files[0]) setFile(target.files[0]);
}

function setFile(file: File) {
  selectedFile.value = file;
  previewUrl.value = URL.createObjectURL(file);
}

function removeFile() {
  selectedFile.value = null;
  previewUrl.value = null;
  if (fileInput.value) fileInput.value.value = '';
}

async function handleSubmit() {
  if (!variantName.value || variantPrice.value === "") return;
  
  let finalPhotoUrl = null;
  if (selectedFile.value) {
    try {
      finalPhotoUrl = await inventoryService.uploadPhoto(selectedFile.value);
    } catch(e) {
      alert("Failed to upload photo.");
      return;
    }
  }

  emit('save', { name: variantName.value.trim(), price: Number(variantPrice.value), photoUrl: finalPhotoUrl });
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 bg-black/60 z-50 flex items-center justify-center backdrop-blur-md p-4" @click.self="$emit('close')">
    <div class="bg-surface-container-low border border-outline-variant/20 rounded-2xl shadow-2xl w-full max-w-sm p-6">
      
      <div class="flex justify-between items-center mb-6">
        <div>
          <h3 class="text-lg font-bold text-on-surface tracking-tight">Add Variant</h3>
          <p class="text-sm text-on-surface-variant mt-1">To {{ baseItemName }}</p>
        </div>
        <button type="button" @click="$emit('close')" class="text-on-surface-variant hover:text-on-surface bg-surface-container hover:bg-surface-container-high p-2 rounded-full transition-colors active:scale-90">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="space-y-5">
        <div>
          <label class="block text-xs font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Variant Name</label>
          <input v-model="variantName" type="text" placeholder="e.g. Large or Spicy" required class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-4 py-3 font-medium focus:border-primary-container outline-none transition-colors">
        </div>

        <div>
          <label class="block text-xs font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Price (PHP)</label>
          <input v-model.number="variantPrice" type="number" step="0.01" min="0" required class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-4 py-3 font-medium focus:border-primary-container outline-none transition-colors">
        </div>

        <div>
          <label class="block text-xs font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Photo (Optional)</label>
          <div @click="!previewUrl ? fileInput?.click() : null" :class="['w-full border-2 border-dashed rounded-xl p-3 flex flex-col items-center justify-center transition-colors', !previewUrl ? 'cursor-pointer border-outline-variant/30 hover:bg-surface-container min-h-24' : 'border-transparent']">
            <input type="file" ref="fileInput" class="hidden" accept="image/*" @change="handleFileSelect" />
            <div v-if="previewUrl" class="relative w-full">
              <img :src="previewUrl" class="max-h-32 mx-auto rounded-lg shadow-sm" />
              <button type="button" @click.stop="removeFile" class="absolute -top-2 -right-2 bg-error text-on-error rounded-full p-1 hover:bg-error/80 shadow-md">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
              </button>
            </div>
            <div v-else class="text-center">
              <p class="text-xs text-on-surface-variant font-medium">Click to upload</p>
            </div>
          </div>
        </div>

        <BaseButton type="submit" variant="primary" class="w-full py-3">Save Variant</BaseButton>
      </form>
    </div>
  </div>
</template>