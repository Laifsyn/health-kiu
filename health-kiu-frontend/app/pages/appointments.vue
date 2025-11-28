<template>
  <div class="p-4">
    <h2 class="text-xl font-bold mb-4 text-gray-800">Especialidades Médicas</h2>
    <p class="text-sm text-gray-600 mb-6">Seleccione una especialidad para ver los doctores disponibles</p>

    <!-- Loading state -->
    <div v-if="pending" class="space-y-3">
      <div v-for="i in 5" :key="i" class="card animate-pulse">
        <div class="h-16 bg-gray-200 rounded"></div>
      </div>
    </div>

    <!-- Error state -->
    <div v-else-if="error" class="card card-pink p-6 text-center">
      <svg class="w-12 h-12 mx-auto text-red-600 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
      </svg>
      <p class="text-gray-700 font-medium">Error al cargar las especialidades</p>
      <p class="text-sm text-gray-600 mt-2">{{ error.message }}</p>
      <button
        @click="refresh()"
        class="mt-4 px-6 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
      >
        Reintentar
      </button>
    </div>

    <!-- Specialties list -->
    <div v-else class="space-y-3">
      <div
        v-for="specialty in data?.items"
        :key="specialty.id"
        class="card hover:shadow-md cursor-pointer transition-all"
        @click="navigateTo(`/specialties/${specialty.id}`)"
      >
        <div class="flex items-center justify-between p-4">
          <div class="flex items-center space-x-4">
            <div class="w-12 h-12 bg-blue-100 rounded-full flex items-center justify-center">
              <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-gray-800">{{ specialty.name }}</h3>
              <p v-if="specialty.img_path" class="text-xs text-gray-500">{{ specialty.img_path }}</p>
            </div>
          </div>
          <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </div>
      </div>

      <!-- Empty state -->
      <div v-if="data?.items.length === 0" class="card p-12 text-center">
        <svg class="w-16 h-16 mx-auto text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
        </svg>
        <p class="text-gray-600 font-medium">No hay especialidades disponibles</p>
      </div>
    </div>

    <!-- Pagination info -->
    <div v-if="data && data.page_size > 0" class="mt-6 text-center text-sm text-gray-600">
      Mostrando {{ data.items.length }} especialidades
      <span v-if="data.has_more" class="text-blue-600"> - Hay más disponibles</span>
    </div>
  </div>
</template>

<script setup lang="ts">
const api = useApi()

const { data, pending, error, refresh } = await useAsyncData(
  'specialties',
  () => api.fetchSpecialties(0, 20)
)
</script>
