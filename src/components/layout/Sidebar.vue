<template>
  <aside class="w-64 bg-gray-900 text-white flex flex-col z-20 shadow-xl h-full">
    <div class="p-6 border-b border-gray-800">
      <h1 class="text-2xl font-bold uppercase tracking-wider text-white">BBQ System</h1>
      <p class="text-sm text-gray-400 mt-1">Admin Portal</p>
    </div>
    
    <nav class="flex-1 px-4 py-6 space-y-1 overflow-y-auto">
      
      <router-link to="/" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z"></path></svg>
        Dashboard
      </router-link>
      
      <router-link to="/pos" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"></path></svg>
        Point of Sale
      </router-link>

      <router-link to="/inventory" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0M6 7l3 9M6 7l6-2m6 2l3-1m-3 1l-3 9a5.002 5.002 0 006.001 0M18 7l3 9m-3-9l-6-2m0-2v2m0 16V5m0 16H9m3 0h3"></path></svg>
        Meat Inventory
      </router-link>

      <router-link to="/prep" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.121 14.121L19 19m-7-7l7-7m-7 7l-2.879 2.879M12 12L9.121 9.121m0 5.758a3 3 0 10-4.243-4.243 3 3 0 004.243 4.243zm8.486 8.486a3 3 0 10-4.243-4.243 3 3 0 004.243 4.243z"></path></svg>
        Prep Station
      </router-link>

      <router-link to="/schedule" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
        Staff Scheduling
      </router-link>
      
      <router-link to="/staff" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>
        Staff Directory
      </router-link>

      <router-link to="/logs" class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-gray-400 hover:bg-gray-800 hover:text-white" active-class="bg-blue-600 text-white shadow-md">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
        System Logs
      </router-link>

    </nav>
    
    <div class="p-4 border-t border-gray-800 text-sm text-gray-500 text-center">
      <p>System Status: <span class="text-green-400 font-semibold">Online</span></p>
    </div>
  </aside>
</template>