<script setup lang="ts">
defineProps<{
  label?: string;
  modelValue: string | number | null;
  type?: string;
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  min?: string | number;
  max?: string | number;
  step?: string | number;
}>();

defineEmits(['update:modelValue']);
</script>

<template>
  <div>
    <label v-if="label" class="block text-sm font-medium text-gray-700 mb-1">
      {{ label }} <span v-if="required" class="text-red-500">*</span>
    </label>
    
    <input 
      :type="type || 'text'" 
      :value="modelValue"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      :placeholder="placeholder"
      :required="required"
      :disabled="disabled"
      :min="min"
      :max="max"
      :step="step"
      class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white disabled:bg-gray-100 disabled:cursor-not-allowed transition-shadow"
    />
  </div>
</template>