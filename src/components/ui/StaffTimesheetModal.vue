<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { scheduleService, type Shift } from '../../services/scheduleService';
import DataLoader from './DataLoader.vue';
import { useResponsive } from '../../composables/useResponsive';

const { fontSm, fontBase, fontLg, fontXl } = useResponsive();

const props = defineProps<{
  isOpen: boolean;
  staff: any;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

const shifts = ref<Shift[]>([]);
const isLoading = ref(false);

watch(() => props.isOpen, async (newVal) => {
  if (newVal && props.staff) {
    isLoading.value = true;
    try {
      shifts.value = await scheduleService.getStaffShifts(props.staff.staff_id);
    } catch (error) {
      console.error("Failed to load timesheet:", error);
    } finally {
      isLoading.value = false;
    }
  } else {
    shifts.value = [];
  }
});

const totalHoursRendered = computed(() => {
  return shifts.value.reduce((total, shift) => {
    return total + (shift.total_rendered_hours || 0);
  }, 0);
});

function formatTime(datetimeStr: string | null) {
  if (!datetimeStr) return '--:--';
  const date = new Date(datetimeStr.replace(' ', 'T'));
  return date.toLocaleString([], { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm" @click.self="$emit('close')">
    <div class="bg-white w-full max-w-3xl rounded-2xl shadow-xl flex flex-col max-h-[85vh] overflow-hidden">
      
      <div class="p-5 border-b border-gray-100 bg-gray-50 flex justify-between items-start shrink-0">
        <div>
          <h3 :class="['font-extrabold text-gray-900', fontXl]">{{ staff?.full_name }}</h3>
          <p :class="['font-medium text-gray-500 mt-0.5', fontSm]">Rendered Work History (Last 30 Shifts)</p>
        </div>
        <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600 bg-gray-200/50 hover:bg-gray-200 p-2 rounded-full transition-colors">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto p-5">
        <div v-if="isLoading" class="py-12 flex justify-center">
          <DataLoader message="Loading timesheet data..." />
        </div>

        <div v-else-if="shifts.length === 0" class="py-12 text-center text-gray-500">
          <svg class="w-12 h-12 mx-auto mb-3 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
          <p :class="['font-medium', fontBase]">No shift history found for this employee.</p>
        </div>

        <table v-else class="w-full text-left border-collapse">
          <thead class="bg-gray-50 sticky top-0 z-10">
            <tr :class="['border-b border-gray-200 text-gray-500 uppercase tracking-wider', fontSm]">
              <th class="p-3 font-bold">Shift Date</th>
              <th class="p-3 font-bold">Time In</th>
              <th class="p-3 font-bold">Time Out</th>
              <th class="p-3 font-bold text-right">Hours</th>
            </tr>
          </thead>
          <tbody class="text-gray-700">
            <tr v-for="shift in shifts" :key="shift.shift_id" class="border-b border-gray-100 hover:bg-gray-50 transition-colors">
              <td :class="['p-3 font-bold text-gray-900', fontSm]">{{ new Date(shift.shift_date).toLocaleDateString() }}</td>
              <td :class="['p-3 text-green-700 font-medium', fontSm]">{{ formatTime(shift.clock_in_time) }}</td>
              <td :class="['p-3 text-red-700 font-medium', fontSm]">{{ formatTime(shift.clock_out_time) }}</td>
              <td :class="['p-3 font-black text-gray-900 text-right', fontSm]">
                {{ shift.total_rendered_hours ? shift.total_rendered_hours.toFixed(2) + ' h' : 'Active' }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="p-5 bg-gray-50 border-t border-gray-100 shrink-0 flex justify-between items-center">
        <span :class="['font-bold text-gray-500', fontBase]">Total Rendered (Last 30)</span>
        <span :class="['font-black text-blue-600', fontLg]">{{ totalHoursRendered.toFixed(2) }} Hours</span>
      </div>

    </div>
  </div>
</template>