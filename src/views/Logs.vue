<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { logService, type SystemLog } from '../services/logService';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import BaseBadge from '../components/ui/BaseBadge.vue';
import ViewLogModal from '../components/ui/ViewLogModal.vue';

const logs = ref<SystemLog[]>([]);
const isLoadingData = ref(true);
const filterCategory = ref('All');

// Modal State
const isLogModalOpen = ref(false);
const selectedLog = ref<SystemLog | null>(null);

async function loadLogs() {
  isLoadingData.value = true;
  try {
    logs.value = await logService.getRecentLogs(200);
  } catch (error) {
    console.error("Failed to load system logs:", error);
  } finally {
    setTimeout(() => {
      isLoadingData.value = false;
    }, 600);
  }
}

const availableCategories = computed(() => {
  const categories = new Set(logs.value.map(log => log.log_category));
  return ['All', ...Array.from(categories)];
});

const filteredLogs = computed(() => {
  if (filterCategory.value === 'All') return logs.value;
  return logs.value.filter(log => log.log_category === filterCategory.value);
});

function formatTime(timestampStr: string) {
  // Pass the ISO 8601 string directly from the Rust backend into the Date object
  const date = new Date(timestampStr);

  // Fallback safety check
  if (isNaN(date.getTime())) {
    return 'Invalid Date';
  }

  return date.toLocaleString(undefined, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  });
}

function getCategoryVariant(category: string): 'info' | 'warning' | 'success' | 'danger' | 'default' {
  if (category === 'POS') return 'info';
  if (category === 'PREP') return 'warning';
  if (category === 'INVENTORY') return 'success';
  if (category === 'ADMIN') return 'danger';
  return 'default';
}

function openLogDetails(log: SystemLog) {
  selectedLog.value = log;
  isLogModalOpen.value = true;
}

function closeLogDetails() {
  isLogModalOpen.value = false;
  selectedLog.value = null;
}

onMounted(() => {
  loadLogs();
});
</script>

<template>
  <div class="h-full flex flex-col relative">
    <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex-1 flex flex-col">

      <div
        class="sticky top-0 z-40 bg-gray-50/95 backdrop-blur -mt-3 md:-mt-4 -mx-3 md:-mx-4 px-3 md:px-4 pt-3 md:pt-4 pb-4 mb-6 border-b border-gray-200 flex flex-col md:flex-row justify-between items-start md:items-center gap-4 rounded-t-xl">
        <div>
          <h3 class="text-xl font-bold text-gray-800">System Logs</h3>
          <p class="text-sm text-gray-500 mt-1">Immutable audit trail of all system transactions.</p>
        </div>

        <div class="flex items-center gap-3">
          <label class="text-sm font-medium text-gray-600">Filter by Event:</label>
          <select v-model="filterCategory"
            class="border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white shadow-sm">
            <option v-for="category in availableCategories" :key="category" :value="category">
              {{ category }}
            </option>
          </select>
          <BaseButton variant="secondary" @click="loadLogs" title="Refresh Logs" class="px-3">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15">
              </path>
            </svg>
          </BaseButton>
        </div>
      </div>

      <DataLoader v-if="isLoadingData" message="Retrieving secure audit trails..." />

      <div v-else class="flex-1 overflow-auto border border-gray-100 rounded-lg">
        <table class="w-full text-left border-collapse">
          <thead class="bg-gray-50 sticky top-0 z-10">
            <tr class="border-b-2 border-gray-200 text-gray-500 text-sm">
              <th class="p-4 font-semibold w-48">Timestamp</th>
              <th class="p-4 font-semibold w-32">Category</th>
              <th class="p-4 font-semibold w-48">User</th>
              <th class="p-4 font-semibold">Action Description</th>
              <th class="p-4 font-semibold text-right">Details</th>
            </tr>
          </thead>
          <tbody class="text-gray-700">
            <tr v-if="filteredLogs.length === 0">
              <td colspan="5" class="py-12 text-center text-gray-500">
                No system activity recorded yet.
              </td>
            </tr>
            <tr v-for="log in filteredLogs" :key="log.log_id" @click="openLogDetails(log)"
              class="border-b border-gray-100 hover:bg-blue-50 transition-colors cursor-pointer group"
              title="Click to view full details">
              <td class="p-4 text-sm whitespace-nowrap">{{ formatTime(log.timestamp) }}</td>
              <td class="p-4">
                <BaseBadge :text="log.log_category" :variant="getCategoryVariant(log.log_category)" />
              </td>
              <td class="p-4 font-medium">{{ log.staff_name || 'System Admin' }}</td>
              <td class="p-4 text-gray-900 group-hover:text-blue-700 transition-colors">{{ log.description }}</td>
              <td class="p-4 text-sm text-gray-500 text-right truncate max-w-50">{{ log.details || '-' }}</td>
            </tr>
          </tbody>
        </table>
      </div>

    </div>

    <ViewLogModal :is-open="isLogModalOpen" :log="selectedLog" @close="closeLogDetails" />

  </div>
</template>