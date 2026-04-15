<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { dashboardService } from '../services/dashboardService';
import DataLoader from '../components/ui/DataLoader.vue';

// State
const isLoadingData = ref(true);
const todaySales = ref(0);
const activeStaff = ref(0);
const lowStockAlerts = ref<any[]>([]);
const topItems = ref<any[]>([]);

async function loadDashboard() {
  isLoadingData.value = true;
  try {
    // We use Promise.all to run all these database queries at the exact same time!
    const [sales, staff, alerts, items] = await Promise.all([
      dashboardService.getTodaySales(),
      dashboardService.getActiveStaffCount(),
      dashboardService.getLowStockAlerts(),
      dashboardService.getTopSellingItems()
    ]);

    todaySales.value = sales;
    activeStaff.value = staff;
    lowStockAlerts.value = alerts;
    topItems.value = items;

  } catch (error) {
    console.error("Error loading dashboard metrics:", error);
  } finally {
    setTimeout(() => {
      isLoadingData.value = false;
    }, 600);
  }
}

onMounted(() => {
  loadDashboard();
});
</script>

<template>
  <div class="h-full flex flex-col">
    
    <div v-if="isLoadingData" class="flex-1 bg-white rounded-xl shadow-sm border border-gray-100 flex items-center justify-center">
      <DataLoader message="Compiling live metrics..." />
    </div>

    <div v-else class="flex-1 overflow-y-auto space-y-6 pb-8">
      
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex items-center gap-4">
          <div class="w-12 h-12 bg-green-100 text-green-600 rounded-full flex items-center justify-center">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
          </div>
          <div>
            <p class="text-sm font-medium text-gray-500">Today's Sales</p>
            <h3 class="text-2xl font-bold text-gray-900">₱{{ todaySales.toFixed(2) }}</h3>
          </div>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex items-center gap-4">
          <div class="w-12 h-12 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
          </div>
          <div>
            <p class="text-sm font-medium text-gray-500">Active Staff</p>
            <h3 class="text-2xl font-bold text-gray-900">{{ activeStaff }} Members</h3>
          </div>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex items-center gap-4">
          <div class="w-12 h-12 bg-red-100 text-red-600 rounded-full flex items-center justify-center">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
          </div>
          <div>
            <p class="text-sm font-medium text-gray-500">Inventory Warnings</p>
            <h3 class="text-2xl font-bold" :class="lowStockAlerts.length > 0 ? 'text-red-600' : 'text-gray-900'">
              {{ lowStockAlerts.length }} Critical
            </h3>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        
        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
          <h3 class="text-lg font-bold text-gray-800 mb-4">Urgent Restock Needed</h3>
          <div v-if="lowStockAlerts.length === 0" class="text-center py-8 text-gray-500">
            <p>All inventory levels are healthy! ✅</p>
          </div>
          <div v-else class="space-y-3">
            <div v-for="(alert, index) in lowStockAlerts" :key="index" class="flex justify-between items-center p-3 bg-red-50 border border-red-100 rounded-lg">
              <div>
                <p class="font-bold text-red-800">{{ alert.category }} - {{ alert.specific_part }}</p>
                <p class="text-xs text-red-600">Threshold: {{ alert.alert_threshold_kg }}kg</p>
              </div>
              <div class="text-right">
                <p class="font-black text-red-600">{{ alert.current_stock_kg }} kg</p>
              </div>
            </div>
          </div>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
          <h3 class="text-lg font-bold text-gray-800 mb-4">Top Selling Items (All Time)</h3>
          <div v-if="topItems.length === 0" class="text-center py-8 text-gray-500">
            <p>No sales data recorded yet.</p>
          </div>
          <div v-else class="space-y-4">
            <div v-for="(item, index) in topItems" :key="index" class="flex items-center gap-4">
              <div class="w-8 h-8 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center font-bold text-sm">
                #{{ index + 1 }}
              </div>
              <div class="flex-1">
                <p class="font-bold text-gray-800">{{ item.pos_display_name }}</p>
              </div>
              <div>
                <p class="font-bold text-gray-900">{{ item.total_sold }} sticks</p>
              </div>
            </div>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>