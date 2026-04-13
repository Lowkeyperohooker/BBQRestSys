<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { inventoryService } from '../services/inventoryService';
import { staffService } from '../services/staffService';

// Form State
const selectedCategory = ref('Chicken');
const selectedPart = ref('Intestine');
const rawKilos = ref<number | null>(null);
const skewersProduced = ref<number | null>(null);
const selectedStaff = ref('');

// Data from Database
const staffMembers = ref<any[]>([]);
const recentLogs = ref<any[]>([]); // We store a local list of what we just prepped

// Dynamically change dropdown options based on the chosen category
const partOptions = computed(() => {
  if (selectedCategory.value === 'Chicken') return ['Intestine', 'Neck', 'Leg Quarter', 'Liver'];
  if (selectedCategory.value === 'Pork') return ['Intestine', 'Liver'];
  if (selectedCategory.value === 'Seafood') return ['Milk Fish Fillet', 'Tuna Fillet'];
  return [];
});

async function loadStaff() {
  const result = await staffService.getAllStaff();
  // Filter to only show Active staff members in the dropdown
  staffMembers.value = (result as any[]).filter(staff => staff.status === 'Active');
  if (staffMembers.value.length > 0) {
    selectedStaff.value = staffMembers.value[0].full_name;
  }
}

async function handleSavePrep() {
  if (!rawKilos.value || !skewersProduced.value || !selectedStaff.value) {
    alert("Please fill out all fields.");
    return;
  }

  try {
    // 1. Send the math to the SQLite Database
    await inventoryService.logPrepTransaction(
      selectedCategory.value, 
      selectedPart.value, 
      rawKilos.value, 
      skewersProduced.value
    );

    // 2. Add to our visual "Recent Activity" table
    const now = new Date();
    const timeString = now.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});
    
    recentLogs.value.unshift({
      time: `Today, ${timeString}`,
      staff: selectedStaff.value,
      part: `${selectedCategory.value} - ${selectedPart.value}`,
      kilos: `-${rawKilos.value.toFixed(1)} kg`,
      sticks: `+${skewersProduced.value} sticks`
    });

    // 3. Clear the numbers for the next task
    rawKilos.value = null;
    skewersProduced.value = null;
    alert("Prep log successfully saved! Inventory has been updated.");

  } catch (error) {
    console.error("Failed to log prep:", error);
    alert("Database error: Ensure your category and part names exactly match your SQLite table data.");
  }
}

onMounted(() => {
  loadStaff();
});
</script>

<template>
  <div class="h-full flex flex-col">
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 flex-1 overflow-y-auto pb-8">
        
      <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 lg:col-span-1 h-fit">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">Log Skewering Task</h3>
        
        <form @submit.prevent="handleSavePrep" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Meat Category</label>
            <select v-model="selectedCategory" @change="selectedPart = partOptions[0]" class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white">
              <option value="Chicken">Chicken</option>
              <option value="Pork">Pork</option>
              <option value="Seafood">Seafood</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Specific Part / Cut</label>
            <select v-model="selectedPart" class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white">
              <option v-for="option in partOptions" :key="option" :value="option">{{ option }}</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Raw Kilos Used</label>
            <input v-model.number="rawKilos" type="number" step="0.1" required placeholder="e.g., 5.0" class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500">
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Skewers Produced</label>
            <input v-model.number="skewersProduced" type="number" required placeholder="e.g., 120" class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500">
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Staff Member</label>
            <select v-model="selectedStaff" required class="w-full border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white">
              <option v-for="staff in staffMembers" :key="staff.staff_id" :value="staff.full_name">
                {{ staff.full_name }}
              </option>
            </select>
          </div>
          <button type="submit" class="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 mt-4 rounded-lg transition-colors">
            Save Prep Log
          </button>
        </form>
      </div>

      <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 lg:col-span-2">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">Recent Skewering Activity</h3>
        <div class="overflow-x-auto">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr class="border-b-2 border-gray-200 text-gray-500 text-sm">
                <th class="pb-3 font-semibold">Time</th>
                <th class="pb-3 font-semibold">Staff Member</th>
                <th class="pb-3 font-semibold">Part Used</th>
                <th class="pb-3 font-semibold">Kilos Deducted</th>
                <th class="pb-3 font-semibold">Skewers Added</th>
              </tr>
            </thead>
            <tbody class="text-gray-700">
              <tr v-if="recentLogs.length === 0">
                <td colspan="5" class="py-8 text-center text-gray-500">No prep activity logged yet today.</td>
              </tr>
              <tr v-for="(log, index) in recentLogs" :key="index" class="border-b border-gray-100 hover:bg-gray-50">
                <td class="py-4 text-sm">{{ log.time }}</td>
                <td class="py-4 font-medium">{{ log.staff }}</td>
                <td class="py-4">{{ log.part }}</td>
                <td class="py-4 text-red-600 font-semibold">{{ log.kilos }}</td>
                <td class="py-4 text-green-600 font-semibold">{{ log.sticks }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

    </div>
  </div>
</template>