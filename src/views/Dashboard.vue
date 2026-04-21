<script setup lang="ts">
import { ref, onMounted, shallowRef, watch, nextTick } from 'vue';
import { dashboardService } from '../services/dashboardService';
import DataLoader from '../components/ui/DataLoader.vue';
import Chart from 'chart.js/auto';

// Live Database State (Top Cards)
const isLoadingData = ref(true);
const todaySales = ref(0);
const activeStaff = ref(0);
const lowStockAlerts = ref<any[]>([]);

// Chart & UI State
const selectedPeriod = ref<'daily' | 'weekly' | 'monthly' | 'yearly'>('weekly');
const salesChartCanvas = ref<HTMLCanvasElement | null>(null);
const meatChartCanvas = ref<HTMLCanvasElement | null>(null);

// We use shallowRef for chart instances to avoid Vue performance issues with deep reactivity
const salesChart = shallowRef<Chart | null>(null);
const meatChart = shallowRef<Chart | null>(null);

// Static Prototype Data for the Charts
const dashboardData = {
  daily: {
    sales: "$2,150.00",
    trendHTML: "<span class='text-green-500'>↑ 5%</span> vs yesterday",
    skewers: "480",
    prep: "52.5 kg",
    chartLabel: "8 AM - 10 PM",
    chartLabels: ['8AM', '10AM', '12PM', '2PM', '4PM', '6PM', '8PM', '10PM'],
    salesData: [50, 120, 350, 280, 200, 650, 400, 100],
    meatData: [25, 18, 9.5],
    meatText: { chicken: "25 kg (48%)", pork: "18 kg (34%)", seafood: "9.5 kg (18%)" }
  },
  weekly: {
    sales: "$14,250.00",
    trendHTML: "<span class='text-green-500'>↑ 18%</span> vs last week",
    skewers: "3,120",
    prep: "345.5 kg",
    chartLabel: "Mon - Sun",
    chartLabels: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
    salesData: [1200, 1100, 1400, 1600, 2200, 2800, 2150],
    meatData: [165, 110, 70.5],
    meatText: { chicken: "165 kg (48%)", pork: "110 kg (32%)", seafood: "70.5 kg (20%)" }
  },
  monthly: {
    sales: "$62,400.00",
    trendHTML: "<span class='text-red-500'>↓ 2%</span> vs last month",
    skewers: "13,850",
    prep: "1,420.0 kg",
    chartLabel: "Week 1 - Week 4",
    chartLabels: ['Week 1', 'Week 2', 'Week 3', 'Week 4'],
    salesData: [15200, 16800, 14400, 16000],
    meatData: [680, 450, 290],
    meatText: { chicken: "680 kg (48%)", pork: "450 kg (32%)", seafood: "290 kg (20%)" }
  },
  yearly: {
    sales: "$748,800.00",
    trendHTML: "<span class='text-green-500'>↑ 12%</span> vs last year",
    skewers: "165,200",
    prep: "17,500.0 kg",
    chartLabel: "Jan - Dec",
    chartLabels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'],
    salesData: [55000, 58000, 62000, 60000, 65000, 70000, 68000, 64000, 59000, 61000, 66000, 75000],
    meatData: [8400, 5600, 3500],
    meatText: { chicken: "8,400 kg (48%)", pork: "5,600 kg (32%)", seafood: "3,500 kg (20%)" }
  }
};

async function loadDashboard() {
  isLoadingData.value = true;
  try {
    const [sales, staff, alerts] = await Promise.all([
      dashboardService.getTodaySales(),
      dashboardService.getActiveStaffCount(),
      dashboardService.getLowStockAlerts()
    ]);
    todaySales.value = sales;
    activeStaff.value = staff;
    lowStockAlerts.value = alerts;
  } catch (error) {
    console.error("Error loading dashboard metrics:", error);
  } finally {
    // We use async here so we can await nextTick
    setTimeout(async () => {
      isLoadingData.value = false;

      // Wait for Vue to render the <canvas> elements into the DOM
      await nextTick();

      initCharts();
    }, 600);
  }
}

function initCharts() {
  if (!salesChartCanvas.value || !meatChartCanvas.value) return;

  const currentData = dashboardData[selectedPeriod.value];

  // Sales Line Chart
  salesChart.value = new Chart(salesChartCanvas.value, {
    type: 'line',
    data: {
      labels: currentData.chartLabels,
      datasets: [{
        label: 'Gross Sales (PHP)',
        data: currentData.salesData,
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

  // Meat Usage Doughnut Chart
  meatChart.value = new Chart(meatChartCanvas.value, {
    type: 'doughnut',
    data: {
      labels: ['Chicken', 'Pork', 'Seafood'],
      datasets: [{
        data: currentData.meatData,
        backgroundColor: ['#f97316', '#ef4444', '#3b82f6'],
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

// Watch for Period Change and Update Charts smoothly without destroying them
watch(selectedPeriod, (newPeriod) => {
  const data = dashboardData[newPeriod];

  if (salesChart.value) {
    salesChart.value.data.labels = data.chartLabels;
    salesChart.value.data.datasets[0].data = data.salesData;
    salesChart.value.update();
  }

  if (meatChart.value) {
    meatChart.value.data.datasets[0].data = data.meatData;
    meatChart.value.update();
  }
});

onMounted(() => {
  loadDashboard();
});
</script>

<template>
  <div class="h-full flex flex-col">

    <div v-if="isLoadingData"
      class="flex-1 bg-white rounded-xl shadow-sm border border-gray-100 flex items-center justify-center">
      <DataLoader message="Compiling live metrics..." />
    </div>

    <div v-else class="flex-1 overflow-y-auto space-y-6 pb-8">

      <div
        class="sticky top-0 z-40 bg-gray-50/95 backdrop-blur -mt-3 md:-mt-4 -mx-3 md:-mx-4 px-3 md:px-4 pt-3 md:pt-4 pb-4 mb-6 border-b border-gray-200 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <h3 class="text-2xl font-bold text-gray-800">Business Overview</h3>
        <div class="flex bg-white rounded-lg shadow-sm border border-gray-200 p-1 w-full md:w-auto mt-4 md:mt-0">
          <button @click="selectedPeriod = 'daily'"
            :class="selectedPeriod === 'daily' ? 'bg-blue-50 text-blue-700' : 'text-gray-500 hover:bg-gray-50'"
            class="flex-1 md:flex-none px-4 py-1.5 text-sm font-medium rounded-md transition-colors">Daily</button>
          <button @click="selectedPeriod = 'weekly'"
            :class="selectedPeriod === 'weekly' ? 'bg-blue-50 text-blue-700' : 'text-gray-500 hover:bg-gray-50'"
            class="flex-1 md:flex-none px-4 py-1.5 text-sm font-medium rounded-md transition-colors">Weekly</button>
          <button @click="selectedPeriod = 'monthly'"
            :class="selectedPeriod === 'monthly' ? 'bg-blue-50 text-blue-700' : 'text-gray-500 hover:bg-gray-50'"
            class="flex-1 md:flex-none px-4 py-1.5 text-sm font-medium rounded-md transition-colors">Monthly</button>
          <button @click="selectedPeriod = 'yearly'"
            :class="selectedPeriod === 'yearly' ? 'bg-blue-50 text-blue-700' : 'text-gray-500 hover:bg-gray-50'"
            class="flex-1 md:flex-none px-4 py-1.5 text-sm font-medium rounded-md transition-colors">Yearly</button>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex items-center gap-4">
          <div class="w-12 h-12 bg-green-100 text-green-600 rounded-full flex items-center justify-center">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z">
              </path>
            </svg>
          </div>
          <div>
            <p class="text-sm font-medium text-gray-500">Live Sales (Today)</p>
            <h3 class="text-2xl font-bold text-gray-900">₱{{ todaySales.toFixed(2) }}</h3>
          </div>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex items-center gap-4">
          <div class="w-12 h-12 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z">
              </path>
            </svg>
          </div>
          <div>
            <p class="text-sm font-medium text-gray-500">Live Active Staff</p>
            <h3 class="text-2xl font-bold text-gray-900">{{ activeStaff }} Members</h3>
          </div>
        </div>

        <div :class="lowStockAlerts.length > 0 ? 'bg-red-50 border-red-100' : 'bg-white border-gray-100'"
          class="p-6 rounded-xl shadow-sm border flex items-center gap-4 transition-colors">
          <div :class="lowStockAlerts.length > 0 ? 'bg-red-100 text-red-600' : 'bg-gray-100 text-gray-400'"
            class="w-12 h-12 rounded-full flex items-center justify-center">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z">
              </path>
            </svg>
          </div>
          <div>
            <p class="text-sm font-medium" :class="lowStockAlerts.length > 0 ? 'text-red-800' : 'text-gray-500'">
              Inventory Warnings</p>
            <h3 class="text-2xl font-bold" :class="lowStockAlerts.length > 0 ? 'text-red-700' : 'text-gray-900'">
              {{ lowStockAlerts.length }} Critical
            </h3>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">

        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 lg:col-span-2">
          <div class="flex justify-between items-center mb-6">
            <h3 class="text-lg font-semibold text-gray-800">Sales Trend</h3>
            <span class="text-xs text-gray-400 font-medium uppercase tracking-wider">{{
              dashboardData[selectedPeriod].chartLabel }}</span>
          </div>
          <div class="relative h-64 w-full">
            <canvas ref="salesChartCanvas"></canvas>
          </div>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex flex-col">
          <h3 class="text-lg font-semibold text-gray-800 mb-6">Meat Usage Distribution</h3>
          <div class="relative flex-1 min-h-40">
            <canvas ref="meatChartCanvas"></canvas>
          </div>
          <div class="mt-6 space-y-3">
            <div class="flex justify-between text-sm items-center">
              <div class="flex items-center gap-2"><span class="w-3 h-3 rounded-full bg-orange-500"></span> <span
                  class="text-gray-600">Chicken Parts</span></div>
              <span class="font-bold text-gray-800">{{ dashboardData[selectedPeriod].meatText.chicken }}</span>
            </div>
            <div class="flex justify-between text-sm items-center">
              <div class="flex items-center gap-2"><span class="w-3 h-3 rounded-full bg-red-500"></span> <span
                  class="text-gray-600">Pork Parts</span></div>
              <span class="font-bold text-gray-800">{{ dashboardData[selectedPeriod].meatText.pork }}</span>
            </div>
            <div class="flex justify-between text-sm items-center">
              <div class="flex items-center gap-2"><span class="w-3 h-3 rounded-full bg-blue-500"></span> <span
                  class="text-gray-600">Seafood Fillets</span></div>
              <span class="font-bold text-gray-800">{{ dashboardData[selectedPeriod].meatText.seafood }}</span>
            </div>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>