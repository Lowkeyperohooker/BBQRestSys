<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Database from "@tauri-apps/plugin-sql";

const systemStatus = ref("Waiting for system...");
// Array to hold the data we fetch from SQLite
const staffMembers = ref<any[]>([]);

// 1. Check the Rust backend connection
async function fetchStatus() {
  try {
    systemStatus.value = await invoke("check_system");
  } catch (error) {
    console.error("Failed to fetch status:", error);
    systemStatus.value = "Backend Ping Failed!";
  }
}

// 2. Test the SQLite Database Connection
async function testDatabase() {
  try {
    // Load the database file created by our Rust migrations
    const db = await Database.load("sqlite:bbq_system.db");

    // Insert a dummy staff member using parameterized queries for security
    await db.execute(
      "INSERT INTO Staff (full_name, role, phone_number) VALUES ($1, $2, $3)",
      ["Juan Dela Cruz", "Grill Cook", "09123456789"]
    );

    // Retrieve all staff members from the table
    const result = await db.select("SELECT * FROM Staff");
    
    // Update the Vue ref to render the list in the template
    staffMembers.value = result as any[];
    systemStatus.value = "Database connection and query successful!";
    
  } catch (error) {
    console.error("Database error:", error);
    systemStatus.value = "Database Error! Check console.";
  }
}
</script>

<template>
  <main class="min-h-screen bg-gray-100 flex flex-col items-center justify-center p-8">
    <div class="max-w-2xl w-full bg-white rounded-xl shadow-lg p-8 border border-gray-200">
      <h1 class="text-3xl font-bold text-gray-800 mb-6 text-center">BBQ Dashboard</h1>
      
      <div class="bg-gray-50 rounded-lg p-4 border border-gray-100 mb-6">
        <p class="text-sm text-gray-500 mb-1">Current Status:</p>
        <p class="text-lg font-medium text-amber-600">{{ systemStatus }}</p>
      </div>

      <div class="flex gap-4 mb-8">
        <button 
          @click="fetchStatus"
          class="flex-1 bg-gray-800 hover:bg-gray-900 text-white font-semibold py-3 px-4 rounded-lg transition-colors duration-200"
        >
          Ping Backend
        </button>
        <button 
          @click="testDatabase"
          class="flex-1 bg-amber-600 hover:bg-amber-700 text-white font-semibold py-3 px-4 rounded-lg transition-colors duration-200"
        >
          Insert & Load Staff DB
        </button>
      </div>

      <div v-if="staffMembers.length > 0" class="border-t border-gray-200 pt-6">
        <h2 class="text-xl font-bold text-gray-800 mb-4">Staff Directory (SQLite Data)</h2>
        <ul class="space-y-3">
          <li 
            v-for="staff in staffMembers" 
            :key="staff.staff_id" 
            class="p-4 bg-gray-50 border border-gray-200 rounded-lg flex justify-between items-center"
          >
            <div>
              <span class="font-medium text-gray-800 block">{{ staff.full_name }}</span>
              <span class="text-sm text-gray-500">{{ staff.phone_number }}</span>
            </div>
            <span class="bg-amber-100 text-amber-800 text-xs font-semibold px-2.5 py-1 rounded">
              {{ staff.role }}
            </span>
          </li>
        </ul>
      </div>

    </div>
  </main>
</template>