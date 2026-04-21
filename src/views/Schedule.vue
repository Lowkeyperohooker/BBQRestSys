<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { staffService } from '../services/staffService';
import { scheduleService, type Shift } from '../services/scheduleService';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import BaseBadge from '../components/ui/BaseBadge.vue';
import { useResponsive } from '../composables/useResponsive';

const { fontSm, fontBase, fontLg, fontXl, isMobile } = useResponsive();

const isLoadingData = ref(true);
const isProcessing = ref(false);
const staffMembers = ref<any[]>([]);
const todayShifts = ref<Shift[]>([]);

const selectedStaffId = ref<number | ''>('');
const currentActiveShift = ref<Shift | null>(null);

async function loadTimeclockData() {
  isLoadingData.value = true;
  try {
    const [staff, shifts] = await Promise.all([
      staffService.getAllStaff(),
      scheduleService.getTodayShifts()
    ]);

    staffMembers.value = (staff as any[]).filter(s => s.status === 'Active');
    todayShifts.value = shifts;

  } catch (error) {
    console.error("Error loading timeclock data:", error);
  } finally {
    setTimeout(() => {
      isLoadingData.value = false;
    }, 600);
  }
}

watch(selectedStaffId, async (newId) => {
  if (!newId) {
    currentActiveShift.value = null;
    return;
  }
  try {
    currentActiveShift.value = await scheduleService.getActiveShiftForStaff(Number(newId));
  } catch (error) {
    console.error("Failed to check active shift status:", error);
  }
});

function formatTimeOnly(datetimeStr: string | null) {
  if (!datetimeStr) return '--:--';
  const date = new Date(datetimeStr.replace(' ', 'T'));
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

async function handleClockIn() {
  if (!selectedStaffId.value) return;
  isProcessing.value = true;
  try {
    await scheduleService.clockIn(Number(selectedStaffId.value));
    alert("Clocked in successfully!");
    selectedStaffId.value = '';
    await loadTimeclockData();
  } catch (error: any) {
    alert(error.message || "Failed to clock in.");
  } finally {
    isProcessing.value = false;
  }
}

async function handleClockOut() {
  if (!selectedStaffId.value || !currentActiveShift.value) return;
  isProcessing.value = true;
  try {
    await scheduleService.clockOut(currentActiveShift.value.shift_id, Number(selectedStaffId.value));
    alert("Clocked out successfully! Shift recorded.");
    selectedStaffId.value = '';
    await loadTimeclockData();
  } catch (error: any) {
    alert(error.message || "Failed to clock out.");
  } finally {
    isProcessing.value = false;
  }
}

onMounted(() => {
  loadTimeclockData();
});
</script>

<template>
  <div class="h-full flex flex-col space-y-6">

    <div class="bg-white p-4 md:p-6 rounded-xl shadow-sm border border-gray-100 shrink-0">
      <div class="sticky top-0 z-40 bg-white/95 backdrop-blur -mt-4 md:-mt-6 -mx-4 md:-mx-6 px-4 md:px-6 pt-4 md:pt-6 pb-4 mb-6 border-b border-gray-200 rounded-t-xl">
        <h3 :class="['font-bold text-gray-800', fontXl]">Staff Timeclock</h3>
        <p :class="['text-gray-500 mt-1', fontSm]">Select your name to punch in or out for your shift.</p>
      </div>

      <div class="flex flex-col md:flex-row items-end gap-4 max-w-3xl">
        <div class="flex-1 w-full">
          <label :class="['block font-semibold text-gray-700 mb-2', fontSm]">Employee Name</label>
          <select v-model="selectedStaffId"
            :class="['w-full border border-gray-300 rounded-lg px-4 py-3 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white shadow-sm', fontBase]"
            :disabled="isProcessing">
            <option value="" disabled>-- Select Your Name --</option>
            <option v-for="staff in staffMembers" :key="staff.staff_id" :value="staff.staff_id">
              {{ staff.full_name }} ({{ staff.role }})
            </option>
          </select>
        </div>

        <div class="w-full md:w-auto flex gap-3 h-13">
          <BaseButton v-if="!currentActiveShift" variant="success" @click="handleClockIn"
            :disabled="!selectedStaffId || isProcessing" :class="['flex-1 md:w-48 py-3', fontBase]">
            <svg v-if="!isProcessing" class="w-5 h-5 mr-2 inline" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1"></path>
            </svg>
            {{ isProcessing ? 'Processing...' : 'Clock IN' }}
          </BaseButton>

          <BaseButton v-else variant="danger" @click="handleClockOut" :disabled="!selectedStaffId || isProcessing"
            :class="['flex-1 md:w-48 py-3', fontBase]">
            <svg v-if="!isProcessing" class="w-5 h-5 mr-2 inline" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path>
            </svg>
            {{ isProcessing ? 'Processing...' : 'Clock OUT' }}
          </BaseButton>
        </div>
      </div>
    </div>

    <div class="bg-white p-4 md:p-6 rounded-xl shadow-sm border border-gray-100 flex-1 flex flex-col min-h-0">
      <div class="mb-4">
        <h3 :class="['font-bold text-gray-800', fontLg]">Today's Timesheet</h3>
      </div>

      <DataLoader v-if="isLoadingData" message="Loading timesheets..." />

      <div v-else class="flex-1 overflow-auto border border-gray-100 rounded-lg">
        <table class="w-full text-left border-collapse">
          <thead class="bg-gray-50 sticky top-0 z-10">
            <tr :class="['border-b-2 border-gray-200 text-gray-500', fontSm]">
              <th class="p-3 md:p-4 font-semibold">Employee</th>
              <th class="p-3 md:p-4 font-semibold hidden md:table-cell">Role</th>
              <th class="p-3 md:p-4 font-semibold">Time In</th>
              <th class="p-3 md:p-4 font-semibold">Time Out</th>
              <th class="p-3 md:p-4 font-semibold hidden sm:table-cell">Hours</th>
              <th class="p-3 md:p-4 font-semibold text-right">Status</th>
            </tr>
          </thead>
          <tbody class="text-gray-700">
            <tr v-if="todayShifts.length === 0">
              <td colspan="6" class="py-12 text-center text-gray-500">
                No one has clocked in yet today.
              </td>
            </tr>
            <tr v-for="shift in todayShifts" :key="shift.shift_id" class="border-b border-gray-100 hover:bg-gray-50 transition-colors">
              <td :class="['p-3 md:p-4 font-bold text-gray-900', fontBase]">{{ shift.full_name }}</td>
              <td :class="['p-3 md:p-4 text-gray-500 hidden md:table-cell', fontSm]">{{ shift.role }}</td>
              <td :class="['p-3 md:p-4 font-medium text-green-700', fontBase]">{{ formatTimeOnly(shift.clock_in_time) }}</td>
              <td :class="['p-3 md:p-4 font-medium text-red-700', fontBase]">{{ formatTimeOnly(shift.clock_out_time) }}</td>
              <td :class="['p-3 md:p-4 font-bold text-gray-900 hidden sm:table-cell', fontBase]">
                {{ shift.total_rendered_hours ? shift.total_rendered_hours + ' h' : '-' }}
              </td>
              <td class="p-3 md:p-4 text-right">
                <BaseBadge :text="shift.status === 'Active Shift' ? 'On Duty' : 'Done'"
                  :variant="shift.status === 'Active Shift' ? 'success' : 'default'"
                  :class="{ 'animate-pulse': shift.status === 'Active Shift' }" />
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

  </div>
</template>