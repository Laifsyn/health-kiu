<template>
  <div class="p-4">
    <h2 class="text-2xl font-bold text-gray-800 mb-4">Servicios Digitales</h2>

    <div class="grid grid-cols-2 gap-3">
      <ServiceCard
        title="Resultados de Laboratorio"
        color="blue"
      />
      <ServiceCard
        title="Historial Médico"
        color="green"
      />
      <ServiceCard
        title="Recetas Electrónicas"
        color="orange"
      />
      <ServiceCard
        title="Facturación"
        color="pink"
      />
    </div>

    <div class="mt-6 card p-6 text-center">
      <svg class="w-16 h-16 mx-auto text-gray-400 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
      </svg>
      <p class="text-gray-600 font-medium">Próximamente más servicios</p>
      <p class="text-sm text-gray-500 mt-1">Estamos trabajando para ofrecerte más opciones digitales</p>
    </div>
  </div>
</template>
