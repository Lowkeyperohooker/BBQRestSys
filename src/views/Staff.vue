<script setup lang="ts">
import { ref, onMounted } from "vue";
import { staffService } from "../services/staffService";
import StaffModal from "../components/ui/StaffModal.vue";
import DataLoader from "../components/ui/DataLoader.vue";
import BaseButton from "../components/ui/BaseButton.vue";

const staffMembers = ref<any[]>([]);
const isModalOpen = ref(false);
const selectedStaff = ref<any>(null);

// Loading State
const isLoadingData = ref(true);

// 1. Fetch data from SQLite
async function loadStaff() {
  isLoadingData.value = true;
  try {
    const result = await staffService.getAllStaff();
    staffMembers.value = result as any[];
  } catch (error) {
    console.error("Failed to load staff:", error);
  } finally {
    setTimeout(() => {
      isLoadingData.value = false;
    }, 600);
  }
}

// 2. Save data to SQLite (Triggered by the Modal)
async function handleSaveStaff(staffData: any) {
  try {
    if (staffData.id) {
      await staffService.updateStaff(staffData.id, staffData);
    } else {
      await staffService.createStaff(staffData);
    }
    
    closeModal();
    await loadStaff(); // Trigger the loader again while fetching!
  } catch (error) {
    console.error("Failed to save staff:", error);
    alert("Error saving staff member.");
  }
}

async function handleDeleteStaff(id: number, name: string) {
  const isConfirmed = confirm(`Are you sure you want to permanently delete ${name}?`);
  
  if (isConfirmed) {
    try {
      await staffService.deleteStaff(id);
      await loadStaff(); // Trigger the loader again
    } catch (error) {
      console.error("Failed to delete staff:", error);
      alert("Error: Cannot delete this staff member because they are linked to existing transactions or prep logs.");
    }
  }
}

// 3. Modal Controls
function openModal(staff?: any) {
  selectedStaff.value = staff || null;
  isModalOpen.value = true;
}

function closeModal() {
  isModalOpen.value = false;
  selectedStaff.value = null;
}

onMounted(() => {
  loadStaff();
});
</script>

<template>
  <div class="h-full flex flex-col">
    <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex-1">
      
      <div class="flex justify-between items-center mb-6">
        <div>
          <h3 class="text-xl font-semibold text-gray-800">Staff Directory</h3>
          <p class="text-sm text-gray-500">Manage employee records and system access</p>
        </div>
        <BaseButton variant="primary" @click="openModal()">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path></svg>
          Add Staff Member
        </BaseButton>
      </div>
      
      <DataLoader v-if="isLoadingData" message="Loading staff records..." />

      <div v-else class="overflow-x-auto">
        <table class="w-full text-left border-collapse">
          <thead>
            <tr class="border-b-2 border-gray-200 text-gray-500 text-sm">
              <th class="pb-3 font-semibold">Name</th>
              <th class="pb-3 font-semibold">Role</th>
              <th class="pb-3 font-semibold">Contact Number</th>
              <th class="pb-3 font-semibold">Account Status</th> 
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
                <div class="flex justify-end gap-2">
                  <button @click="openModal(staff)" class="p-1.5 border border-blue-200 text-blue-600 rounded bg-blue-50 hover:bg-blue-100 transition-colors" title="Edit">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
                  </button>
                  <button @click="handleDeleteStaff(staff.staff_id, staff.full_name)" class="p-1.5 border border-red-200 text-red-500 rounded bg-red-50 hover:bg-red-100 transition-colors" title="Delete">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

    </div>

    <StaffModal 
      :isOpen="isModalOpen" 
      :staffData="selectedStaff" 
      @close="closeModal" 
      @save="handleSaveStaff" 
    />
    
  </div>
</template>