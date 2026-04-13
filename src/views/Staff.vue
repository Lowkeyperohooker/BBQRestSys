<script setup lang="ts">
import { ref, onMounted } from "vue";
import { staffService } from "../services/staffService";

const staffMembers = ref<any[]>([]);
const isModalOpen = ref(false);

const formId = ref<number | null>(null);
const formName = ref("");
const formRole = ref("Cashier");
const formPhone = ref("");
const formStatus = ref("Active");

// Load data using our clean service
async function loadStaff() {
  try {
    const result = await staffService.getAllStaff();
    staffMembers.value = result as any[];
  } catch (error) {
    console.error("Failed to load staff:", error);
  }
}

// Save or Update using our clean service
async function saveStaff() {
  try {
    const staffData = {
      name: formName.value,
      role: formRole.value,
      phone: formPhone.value,
      status: formStatus.value
    };

    if (formId.value) {
      await staffService.updateStaff(formId.value, staffData);
    } else {
      await staffService.createStaff(staffData);
    }
    
    closeModal();
    await loadStaff(); // Refresh the table automatically
  } catch (error) {
    console.error("Failed to save staff:", error);
    alert("Error saving staff member.");
  }
}

function openModal(staff?: any) {
  if (staff) {
    formId.value = staff.staff_id;
    formName.value = staff.full_name;
    formRole.value = staff.role;
    formPhone.value = staff.phone_number;
    formStatus.value = staff.status;
  } else {
    formId.value = null;
    formName.value = "";
    formRole.value = "Cashier";
    formPhone.value = "";
    formStatus.value = "Active";
  }
  isModalOpen.value = true;
}

function closeModal() {
  isModalOpen.value = false;
}

onMounted(() => {
  loadStaff();
});
</script>

<template>
  <div class="space-y-6 h-full">
    <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 h-full flex flex-col">
      <div class="flex justify-between items-center mb-6">
        <div>
          <h3 class="text-xl font-semibold text-gray-800">Staff Directory</h3>
          <p class="text-sm text-gray-500">Manage employee records and system access</p>
        </div>
        <button @click="openModal()" class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded shadow transition-colors font-semibold flex items-center gap-2">
          Add Staff Member
        </button>
      </div>
      
      <div class="overflow-x-auto flex-1">
        <table class="w-full text-left border-collapse">
          <thead>
            <tr class="border-b-2 border-gray-200 text-gray-500 text-sm">
              <th class="pb-3 font-semibold">Name</th>
              <th class="pb-3 font-semibold">Role</th>
              <th class="pb-3 font-semibold">Contact Number</th>
              <th class="pb-3 font-semibold">Status</th>
              <th class="pb-3 font-semibold text-right">Actions</th>
            </tr>
          </thead>
          <tbody class="text-gray-700">
            <tr v-if="staffMembers.length === 0">
              <td colspan="5" class="py-8 text-center text-gray-500">No staff records found. Add one to begin.</td>
            </tr>
            <tr v-for="staff in staffMembers" :key="staff.staff_id" class="border-b border-gray-100 hover:bg-gray-50 transition-colors">
              <td class="py-4 font-medium text-gray-900">{{ staff.full_name }}</td>
              <td class="py-4 text-gray-600">{{ staff.role }}</td>
              <td class="py-4 text-gray-500">{{ staff.phone_number || 'N/A' }}</td>
              <td class="py-4">
                <span :class="staff.status === 'Active' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-600'" class="px-3 py-1 rounded-full text-xs font-bold">
                  {{ staff.status }}
                </span>
              </td>
              <td class="py-4 text-right">
                <button @click="openModal(staff)" class="text-blue-600 hover:text-blue-800 font-medium text-sm px-3 py-1">
                  Edit
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div v-if="isModalOpen" class="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
      <div class="bg-white rounded-xl shadow-xl w-full max-w-md p-6">
        <div class="flex justify-between items-center mb-6">
          <h3 class="text-lg font-bold text-gray-800">{{ formId ? 'Edit Staff Member' : 'Add New Staff' }}</h3>
          <button @click="closeModal" class="text-gray-400 hover:text-gray-600 text-xl font-bold">&times;</button>
        </div>
        
        <form @submit.prevent="saveStaff" class="space-y-4">
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
            <button type="button" @click="closeModal" class="flex-1 bg-white border border-gray-300 text-gray-700 px-4 py-2 rounded-lg hover:bg-gray-50 transition-colors font-medium">Cancel</button>
            <button type="submit" class="flex-1 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors font-semibold">Save Details</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>