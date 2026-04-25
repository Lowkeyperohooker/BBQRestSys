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
    <label v-if="label" class="block text-sm font-bold text-on-surface-variant uppercase tracking-widest mb-2">
      {{ label }} <span v-if="required" class="text-error">*</span>
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
      class="w-full bg-surface-container text-on-surface placeholder-on-surface-variant/50 border border-outline-variant/30 rounded-lg px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary-container focus:border-primary-container disabled:opacity-50 disabled:cursor-not-allowed transition-all"
    />
  </div>
</template>