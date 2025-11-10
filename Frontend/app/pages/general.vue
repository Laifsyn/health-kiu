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
          <h1 class="text-xl sm:text-2xl font-bold" style="color: var(--blue-900);">Información General</h1>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="p-4 sm:p-6">
      <!-- Hospital Info Card -->
      <div class="p-6 rounded-xl shadow-lg border mb-6" style="background-color: var(--white); border-color: var(--blue-200);">
        <div class="flex items-center mb-4">
          <div class="w-12 h-12 rounded-xl flex items-center justify-center mr-4" style="background-color: var(--blue-100);">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
            </svg>
          </div>
          <div>
            <h2 class="font-semibold text-xl mb-1" style="color: var(--blue-900);">Hospital KIU</h2>
            <p class="text-sm" style="color: var(--blue-600);">Tu salud, nuestra prioridad</p>
          </div>
        </div>
        <p class="text-sm leading-relaxed" style="color: var(--gray-600);">
          Hospital KIU es una institución médica de vanguardia comprometida con brindar atención médica de la más alta calidad. 
          Contamos con tecnología de punta y un equipo humano altamente capacitado para cuidar de tu salud y la de tu familia.
        </p>
      </div>

      <!-- Contact Information -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-6 mb-6">
        <div class="p-6 rounded-xl shadow-lg border" style="background-color: var(--white); border-color: var(--blue-200);">
          <div class="flex items-center mb-4">
            <div class="w-12 h-12 rounded-xl flex items-center justify-center mr-4" style="background-color: var(--blue-100);">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"></path>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-lg mb-1" style="color: var(--blue-900);">Contacto</h3>
              <p class="text-sm" style="color: var(--blue-600);">Información de contacto</p>
            </div>
          </div>
          <div class="space-y-3">
            <div>
              <p class="text-sm font-medium" style="color: var(--gray-600);">Teléfono Principal</p>
              <p class="text-sm" style="color: var(--blue-600);">+1 (555) 123-4567</p>
            </div>
            <div>
              <p class="text-sm font-medium" style="color: var(--gray-600);">Emergencias</p>
              <p class="text-sm" style="color: var(--blue-600);">+1 (555) 911-0000</p>
            </div>
            <div>
              <p class="text-sm font-medium" style="color: var(--gray-600);">Email</p>
              <p class="text-sm" style="color: var(--blue-600);">info@hospitalkiu.com</p>
            </div>
          </div>
        </div>

        <div class="p-6 rounded-xl shadow-lg border" style="background-color: var(--white); border-color: var(--blue-200);">
          <div class="flex items-center mb-4">
            <div class="w-12 h-12 rounded-xl flex items-center justify-center mr-4" style="background-color: var(--blue-100);">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-lg mb-1" style="color: var(--blue-900);">Ubicación</h3>
              <p class="text-sm" style="color: var(--blue-600);">Dirección del hospital</p>
            </div>
          </div>
          <div class="space-y-3">
            <div>
              <p class="text-sm font-medium" style="color: var(--gray-600);">Dirección</p>
              <p class="text-sm" style="color: var(--blue-600);">Av. Salud 123, Centro Médico<br>Ciudad KIU, País</p>
            </div>
            <div>
              <p class="text-sm font-medium" style="color: var(--gray-600);">Horarios</p>
              <p class="text-sm" style="color: var(--blue-600);">24 horas, 7 días de la semana</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Quick Stats -->
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
        <div class="p-4 rounded-lg text-center" style="background-color: var(--white); border: 1px solid var(--blue-200);">
          <div class="text-2xl font-bold mb-1" style="color: var(--blue-600);">50+</div>
          <div class="text-xs" style="color: var(--gray-600);">Médicos</div>
        </div>
        <div class="p-4 rounded-lg text-center" style="background-color: var(--white); border: 1px solid var(--blue-200);">
          <div class="text-2xl font-bold mb-1" style="color: var(--blue-600);">100+</div>
          <div class="text-xs" style="color: var(--gray-600);">Enfermeras</div>
        </div>
        <div class="p-4 rounded-lg text-center" style="background-color: var(--white); border: 1px solid var(--blue-200);">
          <div class="text-2xl font-bold mb-1" style="color: var(--blue-600);">200</div>
          <div class="text-xs" style="color: var(--gray-600);">Camas</div>
        </div>
        <div class="p-4 rounded-lg text-center" style="background-color: var(--white); border: 1px solid var(--blue-200);">
          <div class="text-2xl font-bold mb-1" style="color: var(--blue-600);">25+</div>
          <div class="text-xs" style="color: var(--gray-600);">Especialidades</div>
        </div>
      </div>

      <!-- Information Sections -->
      <div class="space-y-6">
        <!-- Mission -->
        <div class="p-6 rounded-xl shadow-lg border" style="background-color: var(--white); border-color: var(--blue-200);">
          <h3 class="font-semibold text-lg mb-3" style="color: var(--blue-900);">Nuestra Misión</h3>
          <p class="text-sm leading-relaxed" style="color: var(--gray-600);">
            Proporcionar atención médica integral de excelencia, combinando tecnología avanzada con un enfoque humano 
            y compasivo, para mejorar la calidad de vida de nuestros pacientes y sus familias.
          </p>
        </div>

        <!-- Services Overview -->
        <div class="p-6 rounded-xl shadow-lg border" style="background-color: var(--white); border-color: var(--blue-200);">
          <h3 class="font-semibold text-lg mb-3" style="color: var(--blue-900);">Nuestros Servicios</h3>
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div class="space-y-2">
              <div class="flex items-center">
                <div class="w-2 h-2 rounded-full mr-3" style="background-color: var(--blue-600);"></div>
                <span class="text-sm" style="color: var(--gray-600);">Consulta Externa</span>
              </div>
              <div class="flex items-center">
                <div class="w-2 h-2 rounded-full mr-3" style="background-color: var(--blue-600);"></div>
                <span class="text-sm" style="color: var(--gray-600);">Hospitalización</span>
              </div>
              <div class="flex items-center">
                <div class="w-2 h-2 rounded-full mr-3" style="background-color: var(--blue-600);"></div>
                <span class="text-sm" style="color: var(--gray-600);">Emergencias</span>
              </div>
              <div class="flex items-center">
                <div class="w-2 h-2 rounded-full mr-3" style="background-color: var(--blue-600);"></div>
                <span class="text-sm" style="color: var(--gray-600);">Cirugía</span>
              </div>
            </div>
            <div class="space-y-2">
              <div class="flex items-center">
                <div class="w-2 h-2 rounded-full mr-3" style="background-color: var(--blue-600);"></div>
                <span class="text-sm" style="color: var(--gray-600);">Laboratorio Clínico</span>
              </div>
              <div class="flex items-center">
                <div class="w-2 h-2 rounded-full mr-3" style="background-color: var(--blue-600);"></div>
                <span class="text-sm" style="color: var(--gray-600);">Imagenología</span>
              </div>
              <div class="flex items-center">
                <div class="w-2 h-2 rounded-full mr-3" style="background-color: var(--blue-600);"></div>
                <span class="text-sm" style="color: var(--gray-600);">Farmacia</span>
              </div>
              <div class="flex items-center">
                <div class="w-2 h-2 rounded-full mr-3" style="background-color: var(--blue-600);"></div>
                <span class="text-sm" style="color: var(--gray-600);">Telemedicina</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Insurance -->
        <div class="p-6 rounded-xl shadow-lg border" style="background-color: var(--white); border-color: var(--blue-200);">
          <h3 class="font-semibold text-lg mb-3" style="color: var(--blue-900);">Seguros Aceptados</h3>
          <p class="text-sm mb-4" style="color: var(--gray-600);">
            Trabajamos con las principales aseguradoras del país para facilitar el acceso a nuestros servicios.
          </p>
          <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
            <div class="p-3 rounded-lg text-center text-sm" style="background-color: var(--blue-50); color: var(--blue-600);">
              Seguros Unidos
            </div>
            <div class="p-3 rounded-lg text-center text-sm" style="background-color: var(--blue-50); color: var(--blue-600);">
              Salud Plus
            </div>
            <div class="p-3 rounded-lg text-center text-sm" style="background-color: var(--blue-50); color: var(--blue-600);">
              MediCare Pro
            </div>
            <div class="p-3 rounded-lg text-center text-sm" style="background-color: var(--blue-50); color: var(--blue-600);">
              Vida Segura
            </div>
            <div class="p-3 rounded-lg text-center text-sm" style="background-color: var(--blue-50); color: var(--blue-600);">
              Protección Total
            </div>
            <div class="p-3 rounded-lg text-center text-sm" style="background-color: var(--blue-50); color: var(--blue-600);">
              Otros
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
          <svg class="w-5 h-5 sm:w-6 sm:h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--gray-400);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2-2v2m8 0H8m8 0v6M8 6v6m0 0H4m4 0h8"></path>
          </svg>
          <span class="text-xs" style="color: var(--gray-400);">Servicios</span>
        </NuxtLink>
        
        <NuxtLink to="/general" class="flex flex-col items-center text-center py-2">
          <svg class="w-5 h-5 sm:w-6 sm:h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--blue-600);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
          </svg>
          <span class="text-xs font-medium" style="color: var(--blue-600);">General</span>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts"></script>

<style></style>