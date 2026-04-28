<script setup lang="ts">
import { ref, watch } from 'vue';
import BaseButton from '../ui/BaseButton.vue';
import { useAuth } from '../../stores/authStore';

const props = defineProps({
  isOpen: Boolean,
  staffData: Object as () => any | null
});

const emit = defineEmits(['close', 'save']);

const authStore = useAuth();

const formId = ref<number | null>(null);
const formName = ref("");
const formRole = ref("Cashier");
const formPhone = ref("");
const formStatus = ref("Active");
const formPasscode = ref("");

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
  formPasscode.value = "";
});

function handleSubmit() {
  const payload: any = {
    id: formId.value,
    name: formName.value,
    role: formRole.value,
    phone: formPhone.value,
    status: formStatus.value
  };

  if (formPasscode.value.trim()) {
    payload.passcode = formPasscode.value.trim();
  }

  emit('save', payload);
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 bg-black/60 z-50 flex items-center justify-center backdrop-blur-md p-4" @click.self="$emit('close')">
    <div class="bg-surface-container-low border border-outline-variant/20 rounded-2xl shadow-2xl w-full max-w-sm p-6 overflow-y-auto max-h-[90vh]">
      
      <div class="flex justify-between items-center mb-6">
        <h3 class="text-lg font-bold text-on-surface tracking-tight">{{ formId ? 'Edit Staff Member' : 'Add New Staff' }}</h3>
        <button type="button" @click="$emit('close')" class="text-on-surface-variant hover:text-on-surface bg-surface-container hover:bg-surface-container-high p-2 rounded-full transition-colors active:scale-90">
           <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="space-y-4">
        <div>
          <label class="block text-xs font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Full Name</label>
          <input v-model="formName" type="text" required class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-3 py-2.5 font-medium focus:border-primary-container outline-none transition-colors">
        </div>
        
        <div class="flex gap-3">
          <div class="flex-1">
            <label class="block text-xs font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Role</label>
            <select v-model="formRole" class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-3 py-2.5 font-medium focus:border-primary-container outline-none transition-colors">
              <option value="Cashier">Cashier</option>
              <option value="Prep Station">Prep Station</option>
              <option value="Grill Cook">Grill Cook</option>
              <option value="Admin">Admin</option>
              <option value="Super Admin">Super Admin</option>
            </select>
          </div>
          <div class="flex-1">
            <label class="block text-xs font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Status</label>
            <select v-model="formStatus" class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-3 py-2.5 font-medium focus:border-primary-container outline-none transition-colors">
              <option value="Active">Active</option>
              <option value="Inactive">Inactive</option>
            </select>
          </div>
        </div>
        
        <div>
          <label class="block text-xs font-bold text-on-surface-variant uppercase tracking-widest mb-1.5">Phone Number</label>
          <input v-model="formPhone" type="tel" placeholder="Optional" class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-3 py-2.5 font-medium focus:border-primary-container outline-none transition-colors">
        </div>

        <div class="bg-primary-container/5 border border-primary-container/30 p-3.5 rounded-xl mt-2">
          <label class="block text-xs font-bold text-primary uppercase tracking-widest mb-1.5">
            {{ formId ? 'Reset Passcode (Optional)' : 'Access Passcode' }}
          </label>
          <input v-model="formPasscode" type="password" :required="!formId" placeholder="••••" class="w-full bg-surface-container text-on-surface border border-outline-variant/30 rounded-xl px-3 py-2.5 font-bold focus:border-primary-container outline-none transition-colors tracking-widest">
          <p v-if="formId" class="text-[10px] text-on-surface-variant mt-2 leading-tight">Leave blank to keep the current passcode.</p>
        </div>

        <div class="pt-2 flex gap-3">
          <BaseButton type="button" variant="secondary" class="flex-1 py-3 text-sm" @click="$emit('close')">Cancel</BaseButton>
          <BaseButton type="submit" variant="primary" class="flex-1 py-3 text-sm">Save Details</BaseButton>
        </div>
      </form>

    </div>
  </div>
</template>