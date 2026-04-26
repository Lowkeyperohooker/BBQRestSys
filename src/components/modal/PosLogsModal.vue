<script setup lang="ts">
import { ref, watch } from 'vue';
import { logService, type SystemLog } from '../../services/logService';
import DataLoader from '../ui/DataLoader.vue';
import BaseButton from '../ui/BaseButton.vue';

const props = defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

const posLogs = ref<SystemLog[]>([]);
const isLoading = ref(false);

watch(() => props.isOpen, async (newVal) => {
  if (newVal) {
    isLoading.value = true;
    try {
      const allLogs = await logService.getRecentLogs(50);
      posLogs.value = allLogs.filter(log => log.log_category === 'POS');
    } catch (error) {
      console.error("Error fetching POS logs:", error);
    } finally {
      setTimeout(() => {
        isLoading.value = false;
      }, 400); 
    }
  }
});

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
  <div v-if="isOpen" class="fixed inset-0 bg-black/60 z-50 flex items-center justify-center p-4 backdrop-blur-md">
    <div class="bg-surface-container-low border border-outline-variant/20 rounded-2xl shadow-2xl w-full max-w-3xl flex flex-col max-h-[80vh] overflow-hidden">
      
      <div class="px-6 py-5 border-b border-outline-variant/10 flex justify-between items-center bg-surface-container-highest/30">
        <div>
          <h3 class="text-xl font-black text-on-surface tracking-tight">Recent POS Transactions</h3>
          <p class="text-sm text-on-surface-variant mt-1">Quick view of the last 50 register actions.</p>
        </div>
        <button @click="emit('close')" class="text-on-surface-variant hover:text-on-surface bg-surface-container hover:bg-surface-container-high p-2 rounded-full transition-colors active:scale-90">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto p-6 bg-surface min-h-75 flex flex-col">
        
        <div v-if="isLoading" class="flex-1 flex items-center justify-center">
          <DataLoader message="Loading recent transactions..." />
        </div>

        <div v-else class="overflow-x-auto border border-outline-variant/15 rounded-xl flex-1 bg-surface-container-low">
          <table class="w-full text-left border-collapse">
            <thead class="bg-surface-container sticky top-0 z-10">
              <tr class="border-b border-outline-variant/20 text-on-surface-variant text-xs uppercase tracking-widest font-bold">
                <th class="p-4">Time</th>
                <th class="p-4">Cashier</th>
                <th class="p-4">Description</th>
                <th class="p-4 text-right">Amount / Details</th>
              </tr>
            </thead>
            <tbody class="text-on-surface">
              <tr v-if="posLogs.length === 0">
                <td colspan="4" class="py-8 text-center text-on-surface-variant">No recent POS transactions found.</td>
              </tr>
              <tr v-for="log in posLogs" :key="log.log_id" class="border-b border-outline-variant/10 hover:bg-surface-container-high transition-colors">
                <td class="p-4 text-sm whitespace-nowrap">{{ formatTime(log.timestamp) }}</td>
                <td class="p-4 font-bold">{{ log.staff_name || 'System' }}</td>
                <td class="p-4 text-sm">{{ log.description }}</td>
                <td class="p-4 font-black text-right text-primary">{{ log.details || '-' }}</td>
              </tr>
            </tbody>
          </table>
        </div>

      </div>

      <div class="px-6 py-5 border-t border-outline-variant/10 bg-surface-container-highest/20 flex justify-end">
        <BaseButton variant="secondary" @click="emit('close')">Close</BaseButton>
      </div>

    </div>
  </div>
</template>