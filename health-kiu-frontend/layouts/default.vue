<template>
  <div class="min-h-screen bg-gray-50 pb-16">
    <!-- Header -->
    <header class="bg-gradient-to-r from-red-600 to-red-700 text-white p-4 shadow-lg">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-3">
          <div class="w-10 h-10 bg-white rounded-full flex items-center justify-center">
            <svg class="w-6 h-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
            </svg>
          </div>
          <div>
            <h1 class="text-lg font-bold">{{ hospitalName }}</h1>
            <p class="text-xs opacity-90">{{ hospitalAddress }}</p>
          </div>
        </div>
        <div class="flex items-center space-x-3">
          <button class="text-sm">{{ changeText }}</button>
          <button class="p-2">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
          </button>
        </div>
      </div>
    </header>

    <!-- Main content -->
    <main class="pb-4">
      <slot />
    </main>

    <!-- Bottom Navigation -->
    <BottomNav />
  </div>
</template>

<script setup lang="ts">
const hospitalName = ref('Health-KIU Hospital')
const hospitalAddress = ref('Vía España, Panamá Ciudad')
const changeText = ref('[Cambiar]')
</script>
