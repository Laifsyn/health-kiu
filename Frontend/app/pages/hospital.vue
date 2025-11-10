<template>
  <div class="min-h-screen" style="background-color: var(--blue-50);">
    <!-- Header -->
    <div class="p-4 sm:p-6" style="background-color: var(--white);">
      <div class="flex items-center justify-between">
        <div class="flex items-center">
          <button @click="$router.back()" class="mr-4 p-2 rounded-lg hover:bg-blue-100 transition-colors">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
            </svg>
          </button>
          <h1 class="text-xl sm:text-2xl font-bold" style="color: var(--blue-900);">Hospital</h1>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="p-4 sm:p-6">
      <!-- Hospital Information Cards -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-6 mb-6">
        <div class="p-6 rounded-xl shadow-lg border hover:shadow-xl transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
          <div class="flex items-center mb-4">
            <div class="w-12 h-12 rounded-xl flex items-center justify-center mr-4" style="background-color: var(--blue-100);">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-lg mb-1" style="color: var(--blue-900);">Ocupación de Camas</h3>
              <p class="text-sm" style="color: var(--blue-600);">Estado actual de hospitalización</p>
            </div>
          </div>
          <div class="space-y-3">
            <div class="flex justify-between">
              <span style="color: var(--gray-600);">Camas Disponibles</span>
              <span class="font-semibold" style="color: var(--blue-600);">24</span>
            </div>
            <div class="flex justify-between">
              <span style="color: var(--gray-600);">Camas Ocupadas</span>
              <span class="font-semibold" style="color: var(--blue-600);">18</span>
            </div>
          </div>
        </div>

        <div class="p-6 rounded-xl shadow-lg border hover:shadow-xl transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
          <div class="flex items-center mb-4">
            <div class="w-12 h-12 rounded-xl flex items-center justify-center mr-4" style="background-color: var(--blue-100);">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-lg mb-1" style="color: var(--blue-900);">Personal Médico</h3>
              <p class="text-sm" style="color: var(--blue-600);">Médicos y enfermeras en turno</p>
            </div>
          </div>
          <div class="space-y-3">
            <div class="flex justify-between">
              <span style="color: var(--gray-600);">Médicos</span>
              <span class="font-semibold" style="color: var(--blue-600);">12</span>
            </div>
            <div class="flex justify-between">
              <span style="color: var(--gray-600);">Enfermeras</span>
              <span class="font-semibold" style="color: var(--blue-600);">28</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Hospital Departments -->
      <div class="mb-6">
        <h2 class="text-lg font-semibold mb-4" style="color: var(--blue-900);">Departamentos</h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          <div class="p-4 rounded-lg border cursor-pointer hover:shadow-lg transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg flex items-center justify-center mr-3" style="background-color: var(--blue-100);">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
                </svg>
              </div>
              <div>
                <h3 class="font-medium" style="color: var(--blue-900);">Cardiología</h3>
                <p class="text-sm" style="color: var(--blue-600);">Piso 3</p>
              </div>
            </div>
          </div>

          <div class="p-4 rounded-lg border cursor-pointer hover:shadow-lg transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg flex items-center justify-center mr-3" style="background-color: var(--blue-100);">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"></path>
                </svg>
              </div>
              <div>
                <h3 class="font-medium" style="color: var(--blue-900);">Neurología</h3>
                <p class="text-sm" style="color: var(--blue-600);">Piso 4</p>
              </div>
            </div>
          </div>

          <div class="p-4 rounded-lg border cursor-pointer hover:shadow-lg transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg flex items-center justify-center mr-3" style="background-color: var(--blue-100);">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"></path>
                </svg>
              </div>
              <div>
                <h3 class="font-medium" style="color: var(--blue-900);">Pediatría</h3>
                <p class="text-sm" style="color: var(--blue-600);">Piso 2</p>
              </div>
            </div>
          </div>

          <div class="p-4 rounded-lg border cursor-pointer hover:shadow-lg transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg flex items-center justify-center mr-3" style="background-color: var(--blue-100);">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4V2a1 1 0 011-1h8a1 1 0 011 1v2m-9 0h10l1 14H6L7 4z"></path>
                </svg>
              </div>
              <div>
                <h3 class="font-medium" style="color: var(--blue-900);">Cirugía</h3>
                <p class="text-sm" style="color: var(--blue-600);">Piso 5</p>
              </div>
            </div>
          </div>

          <div class="p-4 rounded-lg border cursor-pointer hover:shadow-lg transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg flex items-center justify-center mr-3" style="background-color: var(--blue-100);">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
                </svg>
              </div>
              <div>
                <h3 class="font-medium" style="color: var(--blue-900);">UCI</h3>
                <p class="text-sm" style="color: var(--blue-600);">Piso 6</p>
              </div>
            </div>
          </div>

          <div class="p-4 rounded-lg border cursor-pointer hover:shadow-lg transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg flex items-center justify-center mr-3" style="background-color: var(--blue-100);">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                </svg>
              </div>
              <div>
                <h3 class="font-medium" style="color: var(--blue-900);">Emergencias</h3>
                <p class="text-sm" style="color: var(--blue-600);">Planta Baja</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Bottom Navigation -->
    <div class="fixed bottom-0 left-0 right-0 border-t" style="background-color: var(--white); border-color: var(--blue-200);">
      <div class="grid grid-cols-4 py-2">
        <NuxtLink to="/" class="flex flex-col items-center text-center py-2">
          <svg class="w-5 h-5 sm:w-6 sm:h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--gray-400);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
          </svg>
          <span class="text-xs" style="color: var(--gray-400);">Consulta</span>
        </NuxtLink>
        
        <NuxtLink to="/hospital" class="flex flex-col items-center text-center py-2">
          <svg class="w-5 h-5 sm:w-6 sm:h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
          </svg>
          <span class="text-xs font-medium" style="color: var(--blue-600);">Hospital</span>
        </NuxtLink>
        
        <NuxtLink to="/servicios" class="flex flex-col items-center text-center py-2">
          <svg class="w-5 h-5 sm:w-6 sm:h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--gray-400);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2-2v2m8 0H8m8 0v6M8 6v6m0 0H4m4 0h8"></path>
          </svg>
          <span class="text-xs" style="color: var(--gray-400);">Servicios</span>
        </NuxtLink>
        
        <NuxtLink to="/general" class="flex flex-col items-center text-center py-2">
          <svg class="w-5 h-5 sm:w-6 sm:h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--gray-400);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
          </svg>
          <span class="text-xs" style="color: var(--gray-400);">General</span>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts"></script>

<style></style>