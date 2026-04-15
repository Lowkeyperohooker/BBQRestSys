<script setup lang="ts">
import { ref, watch } from 'vue';
import { logService, type SystemLog } from '../../services/logService';
import DataLoader from './DataLoader.vue';
import BaseButton from './BaseButton.vue';

const props = defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

const posLogs = ref<SystemLog[]>([]);
const isLoading = ref(false);

// Fetch logs specifically when the modal opens
watch(() => props.isOpen, async (newVal) => {
  if (newVal) {
    isLoading.value = true;
    try {
      // Fetch recent logs and filter only the ones related to POS transactions
      const allLogs = await logService.getRecentLogs(50);
      posLogs.value = allLogs.filter(log => log.log_category === 'POS');
    } catch (error) {
      console.error("Error fetching POS logs:", error);
    } finally {
      setTimeout(() => {
        isLoading.value = false;
      }, 400); // Short delay for UI smoothness
    }
  }
});

// Format timestamp for easy reading
function formatTime(timestampStr: string) {
  const date = new Date(timestampStr + 'Z');
  return date.toLocaleString(undefined, {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 bg-gray-900/50 z-50 flex items-center justify-center p-4">
    <div class="bg-white rounded-xl shadow-xl w-full max-w-3xl flex flex-col max-h-[80vh] overflow-hidden">
      
      <div class="px-6 py-4 border-b border-gray-100 flex justify-between items-center bg-gray-50">
        <div>
          <h3 class="text-lg font-bold text-gray-800">Recent POS Transactions</h3>
          <p class="text-sm text-gray-500">Quick view of the last 50 register actions.</p>
        </div>
        <button @click="emit('close')" class="text-gray-400 hover:text-gray-600 transition-colors">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto p-6 bg-white min-h-75 flex flex-col">
        
        <div v-if="isLoading" class="flex-1 flex items-center justify-center">
          <DataLoader message="Loading recent transactions..." />
        </div>

        <div v-else class="overflow-x-auto border border-gray-100 rounded-lg flex-1">
          <table class="w-full text-left border-collapse">
            <thead class="bg-gray-50 sticky top-0">
              <tr class="border-b-2 border-gray-200 text-gray-500 text-sm">
                <th class="p-3 font-semibold">Time</th>
                <th class="p-3 font-semibold">Cashier</th>
                <th class="p-3 font-semibold">Description</th>
                <th class="p-3 font-semibold text-right">Amount / Details</th>
              </tr>
            </thead>
            <tbody class="text-gray-700">
              <tr v-if="posLogs.length === 0">
                <td colspan="4" class="py-8 text-center text-gray-500">No recent POS transactions found.</td>
              </tr>
              <tr v-for="log in posLogs" :key="log.log_id" class="border-b border-gray-100 hover:bg-gray-50">
                <td class="p-3 text-sm whitespace-nowrap">{{ formatTime(log.timestamp) }}</td>
                <td class="p-3 font-medium">{{ log.staff_name || 'System' }}</td>
                <td class="p-3">{{ log.description }}</td>
                <td class="p-3 font-bold text-right text-gray-900">{{ log.details || '-' }}</td>
              </tr>
            </tbody>
          </table>
        </div>

      </div>

      <div class="px-6 py-4 border-t border-gray-100 bg-gray-50 flex justify-end">
        <BaseButton variant="secondary" @click="emit('close')">Close</BaseButton>
      </div>

    </div>
  </div>
</template>