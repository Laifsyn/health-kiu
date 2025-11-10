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
          <h1 class="text-xl sm:text-2xl font-bold" style="color: var(--blue-900);">Servicios</h1>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="p-4 sm:p-6">
      <!-- Services Grid -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-6 mb-6">
        <!-- Laboratory Services -->
        <div class="p-6 rounded-xl shadow-lg border hover:shadow-xl transition-all duration-200 cursor-pointer" style="background-color: var(--white); border-color: var(--blue-200);">
          <div class="flex items-center mb-4">
            <div class="w-12 h-12 rounded-xl flex items-center justify-center mr-4" style="background-color: var(--blue-100);">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-lg mb-1" style="color: var(--blue-900);">Laboratorio</h3>
              <p class="text-sm" style="color: var(--blue-600);">Análisis clínicos y estudios</p>
            </div>
          </div>
          <div class="space-y-2">
            <div class="flex justify-between items-center py-2 border-b" style="border-color: var(--blue-100);">
              <span class="text-sm" style="color: var(--gray-600);">Hemograma completo</span>
              <span class="text-sm font-medium" style="color: var(--blue-600);">$25</span>
            </div>
            <div class="flex justify-between items-center py-2 border-b" style="border-color: var(--blue-100);">
              <span class="text-sm" style="color: var(--gray-600);">Química sanguínea</span>
              <span class="text-sm font-medium" style="color: var(--blue-600);">$35</span>
            </div>
            <div class="flex justify-between items-center py-2">
              <span class="text-sm" style="color: var(--gray-600);">Perfil lipídico</span>
              <span class="text-sm font-medium" style="color: var(--blue-600);">$30</span>
            </div>
          </div>
        </div>

        <!-- Imaging Services -->
        <div class="p-6 rounded-xl shadow-lg border hover:shadow-xl transition-all duration-200 cursor-pointer" style="background-color: var(--white); border-color: var(--blue-200);">
          <div class="flex items-center mb-4">
            <div class="w-12 h-12 rounded-xl flex items-center justify-center mr-4" style="background-color: var(--blue-100);">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-lg mb-1" style="color: var(--blue-900);">Imagenología</h3>
              <p class="text-sm" style="color: var(--blue-600);">Estudios de imagen</p>
            </div>
          </div>
          <div class="space-y-2">
            <div class="flex justify-between items-center py-2 border-b" style="border-color: var(--blue-100);">
              <span class="text-sm" style="color: var(--gray-600);">Rayos X</span>
              <span class="text-sm font-medium" style="color: var(--blue-600);">$40</span>
            </div>
            <div class="flex justify-between items-center py-2 border-b" style="border-color: var(--blue-100);">
              <span class="text-sm" style="color: var(--gray-600);">Ecografía</span>
              <span class="text-sm font-medium" style="color: var(--blue-600);">$60</span>
            </div>
            <div class="flex justify-between items-center py-2">
              <span class="text-sm" style="color: var(--gray-600);">Tomografía</span>
              <span class="text-sm font-medium" style="color: var(--blue-600);">$150</span>
            </div>
          </div>
        </div>

        <!-- Pharmacy -->
        <div class="p-6 rounded-xl shadow-lg border hover:shadow-xl transition-all duration-200 cursor-pointer" style="background-color: var(--white); border-color: var(--blue-200);">
          <div class="flex items-center mb-4">
            <div class="w-12 h-12 rounded-xl flex items-center justify-center mr-4" style="background-color: var(--blue-100);">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-lg mb-1" style="color: var(--blue-900);">Farmacia</h3>
              <p class="text-sm" style="color: var(--blue-600);">Medicamentos y suministros</p>
            </div>
          </div>
          <div class="space-y-3">
            <div class="text-center">
              <p class="text-sm" style="color: var(--gray-600);">Horario de Atención</p>
              <p class="font-medium" style="color: var(--blue-600);">24/7</p>
            </div>
            <div class="text-center">
              <p class="text-sm" style="color: var(--gray-600);">Medicamentos en stock</p>
              <p class="font-medium" style="color: var(--blue-600);">+500 productos</p>
            </div>
          </div>
        </div>

        <!-- Emergency Services -->
        <div class="p-6 rounded-xl shadow-lg border hover:shadow-xl transition-all duration-200 cursor-pointer" style="background-color: var(--white); border-color: var(--blue-200);">
          <div class="flex items-center mb-4">
            <div class="w-12 h-12 rounded-xl flex items-center justify-center mr-4" style="background-color: var(--blue-100);">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-lg mb-1" style="color: var(--blue-900);">Emergencias</h3>
              <p class="text-sm" style="color: var(--blue-600);">Atención médica urgente</p>
            </div>
          </div>
          <div class="space-y-3">
            <div class="text-center">
              <p class="text-sm" style="color: var(--gray-600);">Tiempo promedio de espera</p>
              <p class="font-medium" style="color: var(--blue-600);">15 minutos</p>
            </div>
            <div class="text-center">
              <p class="text-sm" style="color: var(--gray-600);">Disponible</p>
              <p class="font-medium" style="color: var(--blue-600);">24/7</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Additional Services -->
      <div class="mb-6">
        <h2 class="text-lg font-semibold mb-4" style="color: var(--blue-900);">Servicios Adicionales</h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          <div class="p-4 rounded-lg border cursor-pointer hover:shadow-lg transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg flex items-center justify-center mr-3" style="background-color: var(--blue-100);">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 4v12l-4-2-4 2V4M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                </svg>
              </div>
              <div>
                <h3 class="font-medium" style="color: var(--blue-900);">Certificados Médicos</h3>
                <p class="text-sm" style="color: var(--blue-600);">$20</p>
              </div>
            </div>
          </div>

          <div class="p-4 rounded-lg border cursor-pointer hover:shadow-lg transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg flex items-center justify-center mr-3" style="background-color: var(--blue-100);">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                </svg>
              </div>
              <div>
                <h3 class="font-medium" style="color: var(--blue-900);">Historiales Médicos</h3>
                <p class="text-sm" style="color: var(--blue-600);">$15</p>
              </div>
            </div>
          </div>

          <div class="p-4 rounded-lg border cursor-pointer hover:shadow-lg transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg flex items-center justify-center mr-3" style="background-color: var(--blue-100);">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                </svg>
              </div>
              <div>
                <h3 class="font-medium" style="color: var(--blue-900);">Citas Online</h3>
                <p class="text-sm" style="color: var(--blue-600);">Gratis</p>
              </div>
            </div>
          </div>

          <div class="p-4 rounded-lg border cursor-pointer hover:shadow-lg transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg flex items-center justify-center mr-3" style="background-color: var(--blue-100);">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"></path>
                </svg>
              </div>
              <div>
                <h3 class="font-medium" style="color: var(--blue-900);">Telemedicina</h3>
                <p class="text-sm" style="color: var(--blue-600);">$45</p>
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
                <h3 class="font-medium" style="color: var(--blue-900);">Nutrición</h3>
                <p class="text-sm" style="color: var(--blue-600);">$50</p>
              </div>
            </div>
          </div>

          <div class="p-4 rounded-lg border cursor-pointer hover:shadow-lg transition-all duration-200" style="background-color: var(--white); border-color: var(--blue-200);">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg flex items-center justify-center mr-3" style="background-color: var(--blue-100);">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
                </svg>
              </div>
              <div>
                <h3 class="font-medium" style="color: var(--blue-900);">Rehabilitación</h3>
                <p class="text-sm" style="color: var(--blue-600);">$40</p>
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
          <svg class="w-5 h-5 sm:w-6 sm:h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--gray-400);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
          </svg>
          <span class="text-xs" style="color: var(--gray-400);">Hospital</span>
        </NuxtLink>
        
        <NuxtLink to="/servicios" class="flex flex-col items-center text-center py-2">
          <svg class="w-5 h-5 sm:w-6 sm:h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2-2v2m8 0H8m8 0v6M8 6v6m0 0H4m4 0h8"></path>
          </svg>
          <span class="text-xs font-medium" style="color: var(--blue-600);">Servicios</span>
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