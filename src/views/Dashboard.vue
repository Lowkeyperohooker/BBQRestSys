<script setup lang="ts">
import { ref, onMounted, shallowRef, watch, nextTick } from 'vue';
import { dashboardService, type PeriodMetrics } from '../services/dashboardService';
import DataLoader from '../components/ui/DataLoader.vue';
import Chart from 'chart.js/auto';
import { useResponsive } from '../composables/useResponsive';

const { fontSm, fontBase, fontLg, fontXl, font2xl } = useResponsive();

// Live Database State (Top Cards)
const isLoadingData = ref(true);
const isUpdatingChart = ref(false);
const todaySales = ref(0);
const activeStaff = ref(0);
const lowStockAlerts = ref<any[]>([]);
const topItems = ref<any[]>([]);

// Chart & UI State
const selectedPeriod = ref<'daily' | 'weekly' | 'monthly' | 'yearly'>('weekly');
const salesChartCanvas = ref<HTMLCanvasElement | null>(null);
const meatChartCanvas = ref<HTMLCanvasElement | null>(null);

const salesChart = shallowRef<Chart | null>(null);
const meatChart = shallowRef<Chart | null>(null);

const currentMetrics = ref<PeriodMetrics>({
  current_sales: 0,
  previous_sales: 0,
  skewers_sold: 0,
  orders: [],
  meat_distribution: []
});

const trendHtml = ref("");
const chartLabelText = ref("");

async function loadDashboard() {
  isLoadingData.value = true;
  try {
    const [sales, staff, alerts, tops] = await Promise.all([
      dashboardService.getTodaySales(),
      dashboardService.getActiveStaffCount(),
      dashboardService.getLowStockAlerts(),
      dashboardService.getTopSellingItems()
    ]);
    todaySales.value = sales;
    activeStaff.value = staff;
    lowStockAlerts.value = alerts;
    topItems.value = tops;
    
    await fetchPeriodMetrics();
  } catch (error) {
    console.error("Error loading dashboard metrics:", error);
  } finally {
    setTimeout(async () => {
      isLoadingData.value = false;
      await nextTick();
      initCharts();
    }, 600);
  }
}

async function fetchPeriodMetrics() {
  isUpdatingChart.value = true;
  try {
    currentMetrics.value = await dashboardService.getPeriodMetrics(selectedPeriod.value);
    updateTrendCalculation();
    if (salesChart.value && meatChart.value) {
      updateCharts();
    }
  } catch (error) {
    console.error("Error fetching period metrics:", error);
  } finally {
    isUpdatingChart.value = false;
  }
}

function updateTrendCalculation() {
  const current = currentMetrics.value.current_sales;
  const previous = currentMetrics.value.previous_sales;
  
  if (previous === 0) {
    trendHtml.value = "<span class='text-green-500 font-bold'>100% Increase</span> vs previous period";
  } else {
    const diff = current - previous;
    const percent = (Math.abs(diff) / previous) * 100;
    
    if (diff >= 0) {
      trendHtml.value = `<span class='text-green-500 font-bold'>Up ${percent.toFixed(1)}%</span> vs previous period`;
    } else {
      trendHtml.value = `<span class='text-red-500 font-bold'>Down ${percent.toFixed(1)}%</span> vs previous period`;
    }
  }

  // Updated to reflect calendar-aligned dates
  chartLabelText.value = selectedPeriod.value === 'daily' ? 'Today' 
                       : selectedPeriod.value === 'weekly' ? 'This Week' 
                       : selectedPeriod.value === 'monthly' ? 'This Month' : 'This Year';
}

function processChartData() {
  const labels: string[] = [];
  const data: number[] = [];
  const grouped = new Map<string, number>();

  currentMetrics.value.orders.forEach(order => {
    const date = new Date(order.timestamp);
    let key = "";
    
    if (selectedPeriod.value === 'daily') {
      key = date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    } else if (selectedPeriod.value === 'weekly' || selectedPeriod.value === 'monthly') {
      key = date.toLocaleDateString([], { month: 'short', day: 'numeric' });
    } else {
      key = date.toLocaleDateString([], { month: 'short', year: 'numeric' });
    }
    
    grouped.set(key, (grouped.get(key) || 0) + order.amount);
  });

  grouped.forEach((value, key) => {
    labels.push(key);
    data.push(value);
  });

  return { labels, data };
}

function processMeatData() {
  const labels = currentMetrics.value.meat_distribution.map(d => d.category);
  const data = currentMetrics.value.meat_distribution.map(d => Number(d.quantity));
  
  const colors = ['#f97316', '#ef4444', '#3b82f6', '#10b981', '#8b5cf6', '#64748b'];
  const bgColors = labels.map((_, i) => colors[i % colors.length]);

  const total = data.reduce((a, b) => a + b, 0);

  return { labels, data, bgColors, total };
}

function initCharts() {
  if (!salesChartCanvas.value || !meatChartCanvas.value) return;

  const salesRes = processChartData();
  salesChart.value = new Chart(salesChartCanvas.value, {
    type: 'line',
    data: {
      labels: salesRes.labels,
      datasets: [{
        label: 'Gross Sales (PHP)',
        data: salesRes.data,
        borderColor: '#3b82f6',
        backgroundColor: 'rgba(59, 130, 246, 0.1)',
        borderWidth: 2,
        fill: true,
        tension: 0.4
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: { y: { beginAtZero: true } }
    }
  });

  const meatRes = processMeatData();
  meatChart.value = new Chart(meatChartCanvas.value, {
    type: 'doughnut',
    data: {
      labels: meatRes.labels.length > 0 ? meatRes.labels : ['No Data'],
      datasets: [{
        data: meatRes.data.length > 0 ? meatRes.data : [1],
        backgroundColor: meatRes.bgColors.length > 0 ? meatRes.bgColors : ['#e2e8f0'],
        borderWidth: 0
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      cutout: '70%',
      plugins: { legend: { display: false } }
    }
  });
}

function updateCharts() {
  if (!salesChart.value || !meatChart.value) return;

  const salesRes = processChartData();
  salesChart.value.data.labels = salesRes.labels;
  salesChart.value.data.datasets[0].data = salesRes.data;
  salesChart.value.update();

  const meatRes = processMeatData();
  meatChart.value.data.labels = meatRes.labels.length > 0 ? meatRes.labels : ['No Data'];
  meatChart.value.data.datasets[0].data = meatRes.data.length > 0 ? meatRes.data : [1];
  meatChart.value.data.datasets[0].backgroundColor = meatRes.bgColors.length > 0 ? meatRes.bgColors : ['#e2e8f0'];
  meatChart.value.update();
}

watch(selectedPeriod, () => {
  fetchPeriodMetrics();
});

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

      <div class="sticky top-0 z-40 bg-gray-50/95 backdrop-blur -mt-3 md:-mt-4 -mx-3 md:-mx-4 px-3 md:px-4 pt-3 md:pt-4 pb-4 mb-6 border-b border-gray-200 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <h3 :class="['font-bold text-gray-800 tracking-tight', font2xl]">Business Overview</h3>
        <div class="flex bg-white rounded-lg shadow-sm border border-gray-200 p-1 w-full md:w-auto mt-4 md:mt-0">
          <button @click="selectedPeriod = 'daily'" :class="[selectedPeriod === 'daily' ? 'bg-blue-50 text-blue-700' : 'text-gray-500 hover:bg-gray-50', 'flex-1 md:flex-none px-4 py-1.5 font-medium rounded-md transition-colors', fontSm]">Daily</button>
          <button @click="selectedPeriod = 'weekly'" :class="[selectedPeriod === 'weekly' ? 'bg-blue-50 text-blue-700' : 'text-gray-500 hover:bg-gray-50', 'flex-1 md:flex-none px-4 py-1.5 font-medium rounded-md transition-colors', fontSm]">Weekly</button>
          <button @click="selectedPeriod = 'monthly'" :class="[selectedPeriod === 'monthly' ? 'bg-blue-50 text-blue-700' : 'text-gray-500 hover:bg-gray-50', 'flex-1 md:flex-none px-4 py-1.5 font-medium rounded-md transition-colors', fontSm]">Monthly</button>
          <button @click="selectedPeriod = 'yearly'" :class="[selectedPeriod === 'yearly' ? 'bg-blue-50 text-blue-700' : 'text-gray-500 hover:bg-gray-50', 'flex-1 md:flex-none px-4 py-1.5 font-medium rounded-md transition-colors', fontSm]">Yearly</button>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex items-center gap-4">
          <div class="w-12 h-12 bg-green-100 text-green-600 rounded-full flex items-center justify-center">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
          </div>
          <div>
            <p :class="['font-medium text-gray-500', fontSm]">Live Sales (Today)</p>
            <h3 :class="['font-bold text-gray-900', font2xl]">₱{{ todaySales.toFixed(2) }}</h3>
          </div>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex items-center gap-4">
          <div class="w-12 h-12 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
          </div>
          <div>
            <p :class="['font-medium text-gray-500', fontSm]">Live Active Staff</p>
            <h3 :class="['font-bold text-gray-900', font2xl]">{{ activeStaff }} Members</h3>
          </div>
        </div>

        <div :class="[lowStockAlerts.length > 0 ? 'bg-red-50 border-red-100' : 'bg-white border-gray-100', 'p-6 rounded-xl shadow-sm border flex items-center gap-4 transition-colors']">
          <div :class="[lowStockAlerts.length > 0 ? 'bg-red-100 text-red-600' : 'bg-gray-100 text-gray-400', 'w-12 h-12 rounded-full flex items-center justify-center']">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
          </div>
          <div>
            <p :class="['font-medium', lowStockAlerts.length > 0 ? 'text-red-800' : 'text-gray-500', fontSm]">Inventory Warnings</p>
            <h3 :class="['font-bold', lowStockAlerts.length > 0 ? 'text-red-700' : 'text-gray-900', font2xl]">{{ lowStockAlerts.length }} Critical</h3>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 relative">
        
        <div v-if="isUpdatingChart" class="absolute inset-0 z-10 bg-white/50 backdrop-blur-sm rounded-xl flex items-center justify-center">
          <p class="font-bold text-blue-600">Updating metrics...</p>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 lg:col-span-2">
          <div class="flex justify-between items-start mb-6">
            <div>
              <h3 :class="['font-semibold text-gray-800', fontLg]">Sales Performance</h3>
              <p class="text-sm mt-1" v-html="trendHtml"></p>
            </div>
            <div class="text-right">
              <span class="text-xs text-gray-400 font-medium uppercase tracking-wider">{{ chartLabelText }}</span>
              <p :class="['font-bold text-gray-900', fontXl]">₱{{ currentMetrics.current_sales.toFixed(2) }}</p>
            </div>
          </div>
          <div class="relative h-64 w-full">
            <canvas ref="salesChartCanvas"></canvas>
          </div>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex flex-col">
          <div class="flex justify-between items-start mb-6">
            <h3 :class="['font-semibold text-gray-800', fontLg]">Volume Sold</h3>
            <div class="text-right">
              <p :class="['font-bold text-gray-900', fontBase]">{{ currentMetrics.skewers_sold }}</p>
              <span class="text-xs text-gray-400 font-medium uppercase tracking-wider">Units</span>
            </div>
          </div>
          <div class="relative flex-1 min-h-40">
            <canvas ref="meatChartCanvas"></canvas>
          </div>
          <div class="mt-6 space-y-3">
            <div v-if="currentMetrics.meat_distribution.length === 0" class="text-center text-sm text-gray-400 font-medium py-2">
              No sales data recorded.
            </div>
            <div v-for="(dist, index) in currentMetrics.meat_distribution" :key="index" class="flex justify-between text-sm items-center">
              <div class="flex items-center gap-2">
                <span class="text-gray-600 font-medium">{{ dist.category }}</span>
              </div>
              <span class="font-bold text-gray-800">{{ dist.quantity }} units</span>
            </div>
          </div>
        </div>
      </div>

      <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
        <h3 :class="['font-semibold text-gray-800 mb-4', fontLg]">Top Selling Items</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
          <div v-if="topItems.length === 0" class="col-span-full text-center text-gray-400 text-sm py-4">No items sold yet.</div>
          <div v-for="(item, index) in topItems" :key="index" class="bg-gray-50 border border-gray-200 rounded-lg p-4 flex items-center justify-between">
            <div>
              <p class="text-xs text-gray-500 font-bold mb-1">#{{ index + 1 }}</p>
              <p class="font-bold text-gray-900 truncate" :title="item.pos_display_name">{{ item.pos_display_name }}</p>
            </div>
            <div class="bg-blue-100 text-blue-700 font-black text-sm px-2.5 py-1 rounded">
              {{ item.total_sold }}
            </div>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>