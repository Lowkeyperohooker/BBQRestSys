<script setup lang="ts">
import type { SystemLog } from '../../services/logService';
import BaseBadge from './BaseBadge.vue';

const props = defineProps<{
  isOpen: boolean;
  log: SystemLog | null;
}>();

defineEmits<{
  close: [];
}>();

function formatTime(timestampStr: string | undefined) {
  if (!timestampStr) return '';
  const date = new Date(timestampStr + 'Z'); 
  return date.toLocaleString(undefined, {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  });
}

function getCategoryVariant(category: string | undefined): 'info' | 'warning' | 'success' | 'danger' | 'default' {
  if (!category) return 'default';
  if (category === 'POS') return 'info';
  if (category === 'PREP') return 'warning';
  if (category === 'INVENTORY') return 'success';
  if (category === 'ADMIN') return 'danger';
  return 'default';
}
</script>

<template>
  <div v-if="isOpen && log" class="fixed inset-0 bg-gray-900/50 z-50 flex items-center justify-center backdrop-blur-sm" @click.self="$emit('close')">
    <div class="bg-white rounded-xl shadow-xl w-full max-w-lg p-0 m-4 overflow-hidden flex flex-col max-h-[90vh]">
      
      <div class="px-6 py-4 bg-gray-50 border-b border-gray-100 flex justify-between items-center">
        <div>
          <h3 class="text-lg font-bold text-gray-900">Log Details</h3>
          <p class="text-xs text-gray-500 font-mono mt-0.5">ID: {{ log.log_id }}</p>
        </div>
        <button type="button" @click="$emit('close')" class="text-gray-400 hover:text-gray-600 transition-colors p-1">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div>
      
      <div class="p-6 overflow-y-auto flex-1 space-y-6">
        
        <div class="flex items-center justify-between">
          <div>
            <p class="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-1">Category</p>
            <BaseBadge :text="log.log_category" :variant="getCategoryVariant(log.log_category)" />
          </div>
          <div class="text-right">
            <p class="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-1">Timestamp</p>
            <p class="text-sm font-medium text-gray-800">{{ formatTime(log.timestamp) }}</p>
          </div>
        </div>

        <div class="border-t border-gray-100 pt-4">
          <p class="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-2">Initiated By</p>
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-full bg-blue-100 text-blue-700 flex items-center justify-center font-bold text-xs">
              {{ log.staff_name ? log.staff_name.charAt(0) : 'S' }}
            </div>
            <div>
              <p class="font-bold text-gray-900">{{ log.staff_name || 'System Admin' }}</p>
              <p class="text-xs text-gray-500">Staff ID: {{ log.staff_id || 'N/A' }}</p>
            </div>
          </div>
        </div>

        <div class="border-t border-gray-100 pt-4">
          <p class="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-2">Action Description</p>
          <p class="text-base text-gray-800">{{ log.description }}</p>
        </div>

        <div class="border-t border-gray-100 pt-4">
          <p class="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-2">Extended Details / Data</p>
          <div class="bg-gray-50 p-4 rounded-lg border border-gray-200">
             <p class="text-sm text-gray-700 font-mono whitespace-pre-wrap wrap-break-word">
              {{ log.details || 'No extended details recorded for this event.' }}
            </p>
          </div>
        </div>

      </div>

    </div>
  </div>
</template>