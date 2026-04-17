<script setup lang="ts">
import { ref, watch } from 'vue';
import type { PreparedInventoryItem } from '../../services/inventoryService';
import BaseButton from './BaseButton.vue';

const props = defineProps<{
  isOpen: boolean;
  item: PreparedInventoryItem | null;
}>();

const emit = defineEmits<{
  close: [];
  save: [data: { prepItemId: number; unitPrice: number; isVariablePrice: number }];
}>();

const formPrice = ref<number>(0);
const formIsVariable = ref<number>(0);

watch(() => props.isOpen, (newVal) => {
  if (newVal && props.item) {
    formPrice.value = props.item.unit_price;
    formIsVariable.value = props.item.is_variable_price;
  }
});

function handleSubmit() {
  if (!props.item) return;
  emit('save', {
    prepItemId: props.item.prep_item_id,
    unitPrice: Number(formPrice.value),
    isVariablePrice: Number(formIsVariable.value)
  });
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 bg-gray-900/50 z-50 flex items-center justify-center backdrop-blur-sm" @click.self="$emit('close')">
    <div class="bg-white rounded-xl shadow-xl w-full max-w-sm p-6 m-4">
      
      <div class="flex justify-between items-center mb-6">
        <div>
          <h3 class="text-lg font-bold text-gray-800">Edit Pricing</h3>
          <p v-if="item" class="text-sm text-gray-500 mt-1">{{ item.pos_display_name }}</p>
        </div>
        <button type="button" @click="$emit('close')" class="text-gray-400 hover:text-gray-600 transition-colors">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="space-y-5">
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Base Unit Price (PHP)</label>
          <input v-model.number="formPrice" type="number" step="0.01" min="0" required class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none bg-white">
        </div>

        <div class="bg-gray-50 p-4 rounded-lg border border-gray-200">
          <label class="block text-sm font-medium text-gray-700 mb-2">Pricing Type</label>
          <div class="space-y-2">
            <label class="flex items-center gap-3 cursor-pointer">
              <input v-model="formIsVariable" type="radio" :value="0" class="w-4 h-4 text-blue-600 focus:ring-blue-500">
              <span class="text-sm text-gray-800">Fixed Price (Standard)</span>
            </label>
            <label class="flex items-center gap-3 cursor-pointer">
              <input v-model="formIsVariable" type="radio" :value="1" class="w-4 h-4 text-blue-600 focus:ring-blue-500">
              <div>
                <span class="text-sm text-gray-800 block">Variable Price</span>
                <span class="text-xs text-gray-500">Cashier must input price at checkout</span>
              </div>
            </label>
          </div>
        </div>

        <div class="flex gap-3 pt-2">
          <BaseButton type="button" variant="secondary" class="flex-1" @click="$emit('close')">Cancel</BaseButton>
          <BaseButton type="submit" variant="primary" class="flex-1">Save Price</BaseButton>
        </div>

      </form>
    </div>
  </div>
</template>