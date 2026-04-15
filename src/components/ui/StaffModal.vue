<script setup lang="ts">
import { ref, watch } from 'vue';
import BaseButton from './BaseButton.vue';

const props = defineProps({
  isOpen: Boolean,
  staffData: Object as () => any | null
});

const emit = defineEmits(['close', 'save']);

const formId = ref<number | null>(null);
const formName = ref("");
const formRole = ref("Cashier");
const formPhone = ref("");
const formStatus = ref("Active");

watch(() => props.isOpen, (newVal) => {
  if (newVal && props.staffData) {
    formId.value = props.staffData.staff_id;
    formName.value = props.staffData.full_name;
    formRole.value = props.staffData.role;
    formPhone.value = props.staffData.phone_number || "";
    formStatus.value = props.staffData.status;
  } else if (newVal) {
    formId.value = null;
    formName.value = "";
    formRole.value = "Cashier";
    formPhone.value = "";
    formStatus.value = "Active";
  }
});

function handleSubmit() {
  emit('save', {
    id: formId.value,
    name: formName.value,
    role: formRole.value,
    phone: formPhone.value,
    status: formStatus.value
  });
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 bg-gray-900/50 z-50 flex items-center justify-center backdrop-blur-sm" @click.self="$emit('close')">
    <div class="bg-white rounded-xl shadow-xl w-full max-w-md p-6">
      
      <div class="flex justify-between items-center mb-6">
        <h3 class="text-lg font-bold text-gray-800">{{ formId ? 'Edit Staff Member' : 'Add New Staff' }}</h3>
        <button type="button" @click="$emit('close')" class="text-gray-400 hover:text-gray-600 transition-colors">
           <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Full Name</label>
          <input v-model="formName" type="text" required class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none">
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Primary Role</label>
          <select v-model="formRole" class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none bg-white">
            <option value="Cashier">Cashier</option>
            <option value="Prep Station">Prep Station</option>
            <option value="Grill Cook">Grill Cook</option>
            <option value="Manager">Manager</option>
          </select>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Phone Number</label>
          <input v-model="formPhone" type="tel" class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none">
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Status</label>
          <select v-model="formStatus" class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 outline-none bg-white">
            <option value="Active">Active</option>
            <option value="Inactive">Inactive</option>
          </select>
        </div>
        <div class="pt-4 flex gap-3">
          <BaseButton type="button" variant="secondary" class="flex-1" @click="$emit('close')">Cancel</BaseButton>
          <BaseButton type="submit" variant="primary" class="flex-1">Save Details</BaseButton>
        </div>
      </form>

    </div>
  </div>
</template>