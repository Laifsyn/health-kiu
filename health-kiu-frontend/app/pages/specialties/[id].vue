<template>
  <div class="p-4">
    <!-- Back button -->
    <button
      @click="navigateTo('/appointments')"
      class="flex items-center space-x-2 text-gray-600 hover:text-gray-900 mb-4"
    >
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
      <span class="text-sm font-medium">Volver</span>
    </button>

    <!-- Specialty info -->
    <div v-if="data" class="mb-6">
      <h2 class="text-2xl font-bold text-gray-800">{{ data.specialty.name }}</h2>
      <p v-if="data.specialty.description" class="text-sm text-gray-600 mt-1">
        {{ data.specialty.description }}
      </p>
    </div>

    <!-- Loading state -->
    <div v-if="pending" class="space-y-3">
      <div v-for="i in 5" :key="i" class="card animate-pulse">
        <div class="h-20 bg-gray-200 rounded"></div>
      </div>
    </div>

    <!-- Error state -->
    <div v-else-if="error" class="card card-pink p-6 text-center">
      <svg class="w-12 h-12 mx-auto text-red-600 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
      </svg>
      <p class="text-gray-700 font-medium">Error al cargar los doctores</p>
      <p class="text-sm text-gray-600 mt-2">{{ error.message }}</p>
      <button
        @click="refresh()"
        class="mt-4 px-6 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
      >
        Reintentar
      </button>
    </div>

    <!-- Doctors list -->
    <div v-else class="space-y-4">
      <h3 class="text-lg font-semibold text-gray-800">Doctores Disponibles</h3>

      <div
        v-for="doctor in data?.items"
        :key="doctor.id"
        class="card hover:shadow-md transition-all"
      >
        <div class="flex items-center justify-between p-4">
          <div class="flex items-center space-x-4">
            <div class="w-14 h-14 bg-gradient-to-br from-blue-500 to-blue-600 rounded-full flex items-center justify-center">
              <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
              </svg>
            </div>
            <div>
              <h4 class="font-semibold text-gray-800">{{ doctor.name.first }} {{ doctor.name.last }}</h4>
              <p class="text-sm text-gray-600">{{ data.specialty.name }}</p>
              <p class="text-xs text-gray-500">Cédula: {{ doctor.cedula }}</p>
              <div class="flex items-center space-x-2 mt-1">
                <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-green-100 text-green-800">
                  Disponible
                </span>
              </div>
            </div>
          </div>
          <button
            @click="openCalendar(doctor.id)"
            class="px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-lg hover:bg-blue-700 transition-colors"
          >
            Agendar Cita
          </button>
        </div>
      </div>

      <!-- Empty state -->
      <div v-if="data?.items.length === 0" class="card p-12 text-center">
        <svg class="w-16 h-16 mx-auto text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
        </svg>
        <p class="text-gray-600 font-medium">No hay doctores disponibles en esta especialidad</p>
        <p class="text-sm text-gray-500 mt-2">Por favor, intente con otra especialidad</p>
      </div>

      <!-- Doctors count -->
      <div v-if="data && data.page_size > 0" class="text-center text-sm text-gray-600 pt-4">
        {{ data.items.length }} doctor(es) encontrado(s)
        <span v-if="data.has_more" class="text-blue-600"> - Hay más disponibles</span>
      </div>
    </div>

    <!-- Calendar Modal -->
    <div v-if="showCalendar" class="modal-overlay" @click.self="closeCalendar">
      <div class="modal-content">
        <div class="modal-header">
          <h2 class="text-xl font-bold text-gray-800">Seleccionar Fecha de Cita</h2>
          <button @click="closeCalendar" class="close-button">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        <AppointmentCalendar
          v-if="selectedDoctorId"
          :doctor-id="selectedDoctorId"
          @date-selected="handleDateSelected"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const route = useRoute()
const api = useApi()

const specialtyId = computed(() => parseInt(route.params.id as string))

const { data, pending, error, refresh } = await useAsyncData(
  `doctors-${specialtyId.value}`,
  () => api.fetchDoctorsBySpecialty(specialtyId.value, 0, 20)
)

// Calendar modal state
const showCalendar = ref(false)
const selectedDoctorId = ref<string | null>(null)

function openCalendar(doctorId: string) {
  selectedDoctorId.value = doctorId
  showCalendar.value = true
}

function closeCalendar() {
  showCalendar.value = false
  selectedDoctorId.value = null
}

function handleDateSelected(date: string) {
  // TODO: Create appointment with the selected date
  console.log('Date selected:', date)
  alert(`Cita agendada para: ${date}`)
  closeCalendar()
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 20px;
}

.modal-content {
  background: white;
  border-radius: 20px;
  max-width: 600px;
  width: 100%;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px;
  border-bottom: 1px solid #E5E7EB;
}

.close-button {
  padding: 8px;
  border-radius: 8px;
  background: #F3F4F6;
  border: none;
  cursor: pointer;
  transition: all 0.2s;
  color: #6B7280;
}

.close-button:hover {
  background: #E5E7EB;
  color: #1F2937;
}
</style>
