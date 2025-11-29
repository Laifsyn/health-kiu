<template>
  <div class="p-4">
    <h2 class="text-2xl font-bold text-gray-800 mb-4">Noticias del Hospital</h2>

    <div class="space-y-4">
      <div class="card hover:shadow-md transition-all">
        <div class="p-4">
          <div class="flex items-start space-x-3">
            <div class="w-16 h-16 bg-blue-100 rounded-lg flex-shrink-0"></div>
            <div class="flex-1">
              <h3 class="font-semibold text-gray-800">Nuevos Servicios de Telemedicina</h3>
              <p class="text-sm text-gray-600 mt-1">Ahora puedes consultar con nuestros especialistas desde la comodidad de tu hogar.</p>
              <span class="text-xs text-gray-500 mt-2 block">Hace 2 días</span>
            </div>
          </div>
        </div>
      </div>

      <div class="card hover:shadow-md transition-all">
        <div class="p-4">
          <div class="flex items-start space-x-3">
            <div class="w-16 h-16 bg-green-100 rounded-lg flex-shrink-0"></div>
            <div class="flex-1">
              <h3 class="font-semibold text-gray-800">Jornada de Vacunación</h3>
              <p class="text-sm text-gray-600 mt-1">Este sábado realizaremos una jornada especial de vacunación. ¡No faltes!</p>
              <span class="text-xs text-gray-500 mt-2 block">Hace 1 semana</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
