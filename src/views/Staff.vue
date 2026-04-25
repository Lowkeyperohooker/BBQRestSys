<script setup lang="ts">
import { ref, onMounted } from "vue";
import { staffService } from "../services/staffService";
import StaffModal from "../components/ui/StaffModal.vue";
import StaffTimesheetModal from "../components/ui/StaffTimesheetModal.vue";
import DataLoader from "../components/ui/DataLoader.vue";
import BaseButton from "../components/ui/BaseButton.vue";
import { useResponsive } from '../composables/useResponsive';

const { fontSm, fontBase, fontXl, isMobile } = useResponsive();

const staffMembers = ref<any[]>([]);
const isModalOpen = ref(false);
const isTimesheetModalOpen = ref(false);
const selectedStaff = ref<any>(null);

const isLoadingData = ref(true);

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

async function handleSaveStaff(staffData: any) {
  try {
    if (staffData.id) {
      await staffService.updateStaff(staffData.id, staffData);
    } else {
      await staffService.createStaff(staffData);
    }

    closeModal();
    await loadStaff(); 
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
      await loadStaff(); 
    } catch (error) {
      console.error("Failed to delete staff:", error);
      alert("Error: Cannot delete this staff member because they are linked to existing transactions or prep logs.");
    }
  }
}

function openModal(staff?: any) {
  selectedStaff.value = staff || null;
  isModalOpen.value = true;
}

function openTimesheetModal(staff: any) {
  selectedStaff.value = staff;
  isTimesheetModalOpen.value = true;
}

function closeModal() {
  isModalOpen.value = false;
  isTimesheetModalOpen.value = false;
  selectedStaff.value = null;
}

onMounted(() => {
  loadStaff();
});
</script>

<template>
  <div class="h-full flex flex-col">
    <div class="bg-white p-4 md:p-6 rounded-xl shadow-sm border border-gray-100 flex-1 flex flex-col min-h-0">

      <div class="sticky top-0 z-40 bg-white/95 backdrop-blur -mt-4 md:-mt-6 -mx-4 md:-mx-6 px-4 md:px-6 pt-4 md:pt-6 pb-4 mb-6 border-b border-gray-200 flex flex-col md:flex-row justify-between items-start md:items-center gap-4 rounded-t-xl shrink-0">
        <div>
          <h3 :class="['font-bold text-gray-800', fontXl]">Staff Directory</h3>
          <p :class="['text-gray-500 mt-1', fontSm]">Manage employee records and system access</p>
        </div>
        <BaseButton variant="primary" @click="openModal()" :class="['w-full md:w-auto flex justify-center', fontBase]">
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
          </svg>
          Add Staff Member
        </BaseButton>
      </div>

      <div class="flex-1 overflow-y-auto">
        <DataLoader v-if="isLoadingData" message="Loading staff records..." />

        <div v-else>
          <table class="w-full text-left border-collapse min-w-150">
            <thead class="sticky top-0 bg-white z-10 shadow-sm">
              <tr :class="['border-b-2 border-gray-200 text-gray-500', fontSm]">
                <th class="py-3 font-semibold px-2">Name</th>
                <th class="py-3 font-semibold px-2">Role</th>
                <th class="py-3 font-semibold px-2">Contact Number</th>
                <th class="py-3 font-semibold px-2">Account Status</th>
                <th class="py-3 font-semibold text-right px-2">Actions</th>
              </tr>
            </thead>
            <tbody class="text-gray-700">
              <tr v-if="staffMembers.length === 0">
                <td colspan="5" class="py-12 text-center text-gray-500">No staff records found. Add one to begin.</td>
              </tr>
              <tr v-for="staff in staffMembers" :key="staff.staff_id" class="border-b border-gray-100 hover:bg-gray-50 transition-colors">
                <td :class="['py-4 font-medium text-gray-900 px-2', fontBase]">{{ staff.full_name }}</td>
                <td :class="['py-4 text-gray-600 px-2', fontBase]">{{ staff.role }}</td>
                <td :class="['py-4 text-gray-500 px-2', fontBase]">{{ staff.phone_number || 'N/A' }}</td>
                <td class="py-4 px-2">
                  <span :class="staff.status === 'Active' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-600'" class="px-3 py-1 rounded-full text-xs font-bold">
                    {{ staff.status }}
                  </span>
                </td>
                <td class="py-4 text-right px-2">
                  <div v-if="staff.role !== 'Super Admin'" class="flex justify-end gap-2">
                    <button @click="openTimesheetModal(staff)" class="p-2 border border-purple-200 text-purple-600 rounded-lg bg-purple-50 hover:bg-purple-100 transition-colors" title="View Rendered Hours">
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                    </button>

                    <button @click="openModal(staff)" class="p-2 border border-blue-200 text-blue-600 rounded-lg bg-blue-50 hover:bg-blue-100 transition-colors" title="Edit">
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
                    </button>

                    <button @click="handleDeleteStaff(staff.staff_id, staff.full_name)" class="p-2 border border-red-200 text-red-500 rounded-lg bg-red-50 hover:bg-red-100 transition-colors" title="Delete">
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
                    </button>
                  </div>
                  <div v-else class="flex justify-end">
                    <span class="bg-gray-100 text-gray-400 text-[10px] font-bold px-2 py-1 rounded border border-gray-200 uppercase tracking-widest cursor-not-allowed">
                      System Protected
                    </span>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

    </div>

    <StaffModal :isOpen="isModalOpen" :staffData="selectedStaff" @close="closeModal" @save="handleSaveStaff" />
    <StaffTimesheetModal :isOpen="isTimesheetModalOpen" :staff="selectedStaff" @close="closeModal" />

  </div>
</template>