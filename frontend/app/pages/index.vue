<template>
  <div class="min-h-screen" style="background-color: var(--gray-50);">
    <!-- Main Navigation Icons -->
    <div class="grid grid-cols-4 gap-3 p-4 sm:gap-4 sm:p-6" style="background-color: var(--white);">
      <div class="flex flex-col items-center text-center">
        <div class="w-10 h-10 sm:w-12 sm:h-12 mb-2 rounded-lg flex items-center justify-center" style="background-color: var(--blue-100);">
          <svg class="w-5 h-5 sm:w-6 sm:h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
          </svg>
        </div>
        <span class="text-xs" style="color: var(--gray-600);">Agendar Consulta</span>
      </div>
      
      <div class="flex flex-col items-center text-center">
        <div class="w-10 h-10 sm:w-12 sm:h-12 mb-2 rounded-lg flex items-center justify-center" style="background-color: var(--red-100);">
          <svg class="w-5 h-5 sm:w-6 sm:h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--red-600);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
          </svg>
        </div>
        <span class="text-xs" style="color: var(--gray-600);">Atención Internacional</span>
      </div>
      
      <div class="flex flex-col items-center text-center">
        <div class="w-10 h-10 sm:w-12 sm:h-12 mb-2 rounded-lg flex items-center justify-center" style="background-color: var(--orange-100);">
          <svg class="w-5 h-5 sm:w-6 sm:h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--orange-600);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path>
          </svg>
        </div>
        <span class="text-xs" style="color: var(--gray-600);">Registro del Paciente</span>
      </div>
      
      <div class="flex flex-col items-center text-center">
        <div class="w-10 h-10 sm:w-12 sm:h-12 mb-2 rounded-lg flex items-center justify-center" style="background-color: var(--primary-100);">
          <svg class="w-5 h-5 sm:w-6 sm:h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--primary-600);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path>
          </svg>
        </div>
        <span class="text-xs" style="color: var(--gray-600);">Consulta General</span>
      </div>
    </div>

    <!-- Administrative Functions -->
    <div class="p-4 sm:p-6">
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-6">
        <div class="p-6 sm:p-8 rounded-xl shadow-lg border-2 hover:shadow-xl transition-all duration-200 transform hover:-translate-y-1" style="background: linear-gradient(135deg, var(--blue-100) 0%, var(--white) 100%); border-color: var(--blue-600);">
          <div class="flex items-center">
            <div class="w-12 h-12 sm:w-16 sm:h-16 rounded-xl flex items-center justify-center mr-4 shadow-md" style="background-color: var(--blue-600);">
              <svg class="w-6 h-6 sm:w-8 sm:h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--white);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-base sm:text-lg mb-1" style="color: var(--gray-900);">Llegada Consulta</h3>
              <p class="text-xs sm:text-sm opacity-75" style="color: var(--gray-600);">Registrar llegada de pacientes</p>
            </div>
          </div>
        </div>
        
        <div class="p-6 sm:p-8 rounded-xl shadow-lg border-2 hover:shadow-xl transition-all duration-200 transform hover:-translate-y-1" style="background: linear-gradient(135deg, var(--primary-100) 0%, var(--white) 100%); border-color: var(--primary-600);">
          <div class="flex items-center">
            <div class="w-12 h-12 sm:w-16 sm:h-16 rounded-xl flex items-center justify-center mr-4 shadow-md" style="background-color: var(--primary-600);">
              <svg class="w-6 h-6 sm:w-8 sm:h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--white);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"></path>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-base sm:text-lg mb-1" style="color: var(--gray-900);">Pago Rápido</h3>
              <p class="text-xs sm:text-sm opacity-75" style="color: var(--gray-600);">Procesar pagos inmediatos</p>
            </div>
          </div>
        </div>
        
        <div class="p-6 sm:p-8 rounded-xl shadow-lg border-2 hover:shadow-xl transition-all duration-200 transform hover:-translate-y-1" style="background: linear-gradient(135deg, var(--red-100) 0%, var(--white) 100%); border-color: var(--red-600);">
          <div class="flex items-center">
            <div class="w-12 h-12 sm:w-16 sm:h-16 rounded-xl flex items-center justify-center mr-4 shadow-md" style="background-color: var(--red-600);">
              <svg class="w-6 h-6 sm:w-8 sm:h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--white);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-base sm:text-lg mb-1" style="color: var(--gray-900);">Factura Automática</h3>
              <p class="text-xs sm:text-sm opacity-75" style="color: var(--gray-600);">Generar facturas automatizadas</p>
            </div>
          </div>
        </div>
        
        <div class="p-6 sm:p-8 rounded-xl shadow-lg border-2 hover:shadow-xl transition-all duration-200 transform hover:-translate-y-1" style="background: linear-gradient(135deg, var(--purple-100) 0%, var(--white) 100%); border-color: var(--purple-600);">
          <div class="flex items-center">
            <div class="w-12 h-12 sm:w-16 sm:h-16 rounded-xl flex items-center justify-center mr-4 shadow-md" style="background-color: var(--purple-600);">
              <svg class="w-6 h-6 sm:w-8 sm:h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--white);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-base sm:text-lg mb-1" style="color: var(--gray-900);">Reportes</h3>
              <p class="text-xs sm:text-sm opacity-75" style="color: var(--gray-600);">Visualizar estadísticas y reportes</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Bottom Navigation -->
    <div class="fixed bottom-0 left-0 right-0 border-t" style="background-color: var(--white); border-color: var(--gray-200);">
      <div class="grid grid-cols-4 py-2">
        <div class="flex flex-col items-center text-center py-2">
          <svg class="w-5 h-5 sm:w-6 sm:h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--primary-600);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
          </svg>
          <span class="text-xs font-medium" style="color: var(--primary-600);">Consulta</span>
        </div>
        
        <div class="flex flex-col items-center text-center py-2">
          <svg class="w-5 h-5 sm:w-6 sm:h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--gray-400);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
          </svg>
          <span class="text-xs" style="color: var(--gray-400);">Hospital</span>
        </div>
        
        <div class="flex flex-col items-center text-center py-2">
          <svg class="w-5 h-5 sm:w-6 sm:h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--gray-400);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2-2v2m8 0H8m8 0v6M8 6v6m0 0H4m4 0h8"></path>
          </svg>
          <span class="text-xs" style="color: var(--gray-400);">Servicios</span>
        </div>
        
        <div class="flex flex-col items-center text-center py-2">
          <svg class="w-5 h-5 sm:w-6 sm:h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--gray-400);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
          </svg>
          <span class="text-xs" style="color: var(--gray-400);">General</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts"></script>

<style></style>
