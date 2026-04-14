<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { logService, type SystemLog } from '../services/logService';
import DataLoader from '../components/ui/DataLoader.vue';

const logs = ref<SystemLog[]>([]);
const isLoadingData = ref(true);
const filterCategory = ref('All');

async function loadLogs() {
  isLoadingData.value = true;
  try {
    logs.value = await logService.getRecentLogs(200); // Fetch up to 200 recent events
  } catch (error) {
    console.error("Failed to load system logs:", error);
  } finally {
    setTimeout(() => {
      isLoadingData.value = false;
    }, 600);
  }
}

// Compute available categories for the filter dropdown based on actual data
const availableCategories = computed(() => {
  const categories = new Set(logs.value.map(log => log.log_category));
  return ['All', ...Array.from(categories)];
});

// Filter the logs based on the selected category
const filteredLogs = computed(() => {
  if (filterCategory.value === 'All') return logs.value;
  return logs.value.filter(log => log.log_category === filterCategory.value);
});

// Format SQLite timestamp (UTC) to a readable local format
function formatTime(timestampStr: string) {
  const date = new Date(timestampStr + 'Z'); // Add Z to specify it is UTC from SQLite
  return date.toLocaleString(undefined, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  });
}

onMounted(() => {
  loadLogs();
});
</script>

<template>
  <div class="h-full flex flex-col">
    <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex-1 flex flex-col">
      
      <div class="flex justify-between items-center mb-6">
        <div>
          <h3 class="text-xl font-semibold text-gray-800">System Logs</h3>
          <p class="text-sm text-gray-500">Immutable audit trail of all system transactions and administrative actions.</p>
        </div>
        
        <div class="flex items-center gap-3">
          <label class="text-sm font-medium text-gray-600">Filter by Event:</label>
          <select 
            v-model="filterCategory"
            class="border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white shadow-sm"
          >
            <option v-for="category in availableCategories" :key="category" :value="category">
              {{ category }}
            </option>
          </select>
          <button @click="loadLogs" class="p-2 border border-gray-200 text-gray-600 rounded bg-gray-50 hover:bg-gray-100 transition-colors" title="Refresh Logs">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path></svg>
          </button>
        </div>
      </div>

      <div v-if="isLoadingData" class="flex-1 flex items-center justify-center">
        <DataLoader message="Retrieving secure audit trails..." />
      </div>

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
            <tr v-for="log in filteredLogs" :key="log.log_id" class="border-b border-gray-100 hover:bg-gray-50 transition-colors">
              <td class="p-4 text-sm whitespace-nowrap">{{ formatTime(log.timestamp) }}</td>
              <td class="p-4">
                <span class="bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-xs font-bold tracking-wide">
                  {{ log.log_category }}
                </span>
              </td>
              <td class="p-4 font-medium">{{ log.staff_name || 'System Admin' }}</td>
              <td class="p-4 text-gray-900">{{ log.description }}</td>
              <td class="p-4 text-sm text-gray-500 text-right">{{ log.details || '-' }}</td>
            </tr>
          </tbody>
        </table>
      </div>

    </div>
  </div>
</template>