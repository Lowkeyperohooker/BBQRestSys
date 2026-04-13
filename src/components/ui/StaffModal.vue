<script setup lang="ts">
import { ref, watch } from 'vue';

// These 'props' allow the main page to tell the modal if it should be open, 
// and pass in existing staff data if we are editing someone.
const props = defineProps({
  isOpen: Boolean,
  staffData: Object
});

// These 'emits' allow the modal to talk back to the main page
const emit = defineEmits(['close', 'save']);

const formId = ref<number | null>(null);
const formName = ref("");
const formRole = ref("Cashier");
const formPhone = ref("");
const formStatus = ref("Active");

// Watch for when the modal opens to populate the form correctly
watch(() => props.isOpen, (newVal) => {
  if (newVal && props.staffData) {
    // Editing existing staff
    formId.value = props.staffData.staff_id;
    formName.value = props.staffData.full_name;
    formRole.value = props.staffData.role;
    formPhone.value = props.staffData.phone_number;
    formStatus.value = props.staffData.status;
  } else if (newVal) {
    // Adding new staff: clear the form
    formId.value = null;
    formName.value = "";
    formRole.value = "Cashier";
    formPhone.value = "";
    formStatus.value = "Active";
  }
});

// Send the data back to the main page when the user clicks Save
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
  <div v-if="isOpen" class="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center backdrop-blur-sm">
    <div class="bg-white rounded-xl shadow-xl w-full max-w-md p-6">
      
      <div class="flex justify-between items-center mb-6">
        <h3 class="text-lg font-bold text-gray-800">{{ formId ? 'Edit Staff Member' : 'Add New Staff' }}</h3>
        <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600 transition-colors">
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
          <button type="button" @click="$emit('close')" class="flex-1 bg-white border border-gray-300 text-gray-700 px-4 py-2 rounded-lg hover:bg-gray-50 transition-colors font-medium">Cancel</button>
          <button type="submit" class="flex-1 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors font-semibold">Save Details</button>
        </div>
      </form>

    </div>
  </div>
</template>