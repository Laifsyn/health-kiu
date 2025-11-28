<template>
  <div class="calendar-container">
    <!-- Header -->
    <div class="calendar-header">
      <button @click="previousMonth" class="nav-button">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
        </svg>
      </button>
      <h3 class="calendar-title">{{ currentMonthYear }}</h3>
      <button @click="nextMonth" class="nav-button">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
        </svg>
      </button>
    </div>

    <!-- Loading state -->
    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p class="text-sm text-gray-600 mt-3">Cargando disponibilidad...</p>
    </div>

    <!-- Calendar grid -->
    <div v-else class="calendar-grid">
      <!-- Weekday headers -->
      <div v-for="day in weekDays" :key="day" class="weekday-header">
        {{ day }}
      </div>

      <!-- Calendar days -->
      <div
        v-for="(day, index) in calendarDays"
        :key="index"
        :class="getDayClasses(day)"
        @click="selectDate(day)"
      >
        <span class="day-number">{{ day.dayNumber }}</span>
        <span v-if="day.status === 'available'" class="availability-badge">
          {{ day.appointmentsCount }}/5
        </span>
      </div>
    </div>

    <!-- Legend -->
    <div class="legend">
      <div class="legend-item">
        <div class="legend-dot available"></div>
        <span>Disponible</span>
      </div>
      <div class="legend-item">
        <div class="legend-dot full"></div>
        <span>Lleno (5/5)</span>
      </div>
      <div class="legend-item">
        <div class="legend-dot weekend"></div>
        <span>Fin de semana</span>
      </div>
    </div>

    <!-- Selected date info -->
    <div v-if="selectedDate" class="selected-info">
      <p class="text-sm font-medium text-gray-700">Fecha seleccionada:</p>
      <p class="text-lg font-bold text-blue-600">{{ formatSelectedDate }}</p>
      <button @click="confirmDate" class="confirm-button">
        Confirmar Cita
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  doctorId: string
}>()

const emit = defineEmits<{
  dateSelected: [date: string]
}>()

const api = useApi()

const currentDate = ref(new Date())
const selectedDate = ref<string | null>(null)
const availabilityData = ref<any[]>([])
const loading = ref(false)

const weekDays = ['Dom', 'Lun', 'Mar', 'Mié', 'Jue', 'Vie', 'Sáb']

const currentMonthYear = computed(() => {
  const months = [
    'Enero', 'Febrero', 'Marzo', 'Abril', 'Mayo', 'Junio',
    'Julio', 'Agosto', 'Septiembre', 'Octubre', 'Noviembre', 'Diciembre'
  ]
  return `${months[currentDate.value.getMonth()]} ${currentDate.value.getFullYear()}`
})

const formatSelectedDate = computed(() => {
  if (!selectedDate.value) return ''
  const date = new Date(selectedDate.value + 'T00:00:00')
  return date.toLocaleDateString('es-ES', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  })
})

interface CalendarDay {
  date: string
  dayNumber: number
  status: 'available' | 'full' | 'weekend' | 'empty'
  appointmentsCount: number
  isCurrentMonth: boolean
}

const calendarDays = computed((): CalendarDay[] => {
  const year = currentDate.value.getFullYear()
  const month = currentDate.value.getMonth()

  // First day of the month
  const firstDay = new Date(year, month, 1)
  const firstDayOfWeek = firstDay.getDay()

  // Last day of the month
  const lastDay = new Date(year, month + 1, 0)
  const totalDays = lastDay.getDate()

  const days: CalendarDay[] = []

  // Add empty cells for days before the first day of month
  for (let i = 0; i < firstDayOfWeek; i++) {
    days.push({
      date: '',
      dayNumber: 0,
      status: 'empty',
      appointmentsCount: 0,
      isCurrentMonth: false
    })
  }

  // Add days of the month
  for (let day = 1; day <= totalDays; day++) {
    const dateStr = `${year}-${String(month + 1).padStart(2, '0')}-${String(day).padStart(2, '0')}`
    const availability = availabilityData.value.find(d => d.date === dateStr)

    days.push({
      date: dateStr,
      dayNumber: day,
      status: availability?.status || 'weekend',
      appointmentsCount: availability?.appointments_count || 0,
      isCurrentMonth: true
    })
  }

  return days
})

function getDayClasses(day: CalendarDay) {
  const classes = ['calendar-day']

  if (!day.isCurrentMonth) {
    classes.push('empty-day')
    return classes
  }

  if (day.status === 'available') {
    classes.push('available-day')
    if (day.date === selectedDate.value) {
      classes.push('selected-day')
    }
  } else if (day.status === 'full') {
    classes.push('full-day')
  } else if (day.status === 'weekend') {
    classes.push('weekend-day')
  }

  return classes
}

function selectDate(day: CalendarDay) {
  if (day.status === 'available' && day.isCurrentMonth) {
    selectedDate.value = day.date
  }
}

function confirmDate() {
  if (selectedDate.value) {
    emit('dateSelected', selectedDate.value)
  }
}

async function fetchAvailability() {
  loading.value = true
  try {
    const year = currentDate.value.getFullYear()
    const month = currentDate.value.getMonth()

    // Get first and last day of the month
    const startDate = `${year}-${String(month + 1).padStart(2, '0')}-01`
    const lastDay = new Date(year, month + 1, 0).getDate()

    const data = await api.fetchDoctorAvailability(props.doctorId, startDate, lastDay)
    availabilityData.value = data.dates
  } catch (error) {
    console.error('Error fetching availability:', error)
  } finally {
    loading.value = false
  }
}

function previousMonth() {
  currentDate.value = new Date(currentDate.value.getFullYear(), currentDate.value.getMonth() - 1, 1)
  selectedDate.value = null
  fetchAvailability()
}

function nextMonth() {
  currentDate.value = new Date(currentDate.value.getFullYear(), currentDate.value.getMonth() + 1, 1)
  selectedDate.value = null
  fetchAvailability()
}

// Fetch availability on mount
onMounted(() => {
  fetchAvailability()
})
</script>

<style scoped>
.calendar-container {
  background: white;
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  max-width: 500px;
  margin: 0 auto;
}

.calendar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.calendar-title {
  font-size: 18px;
  font-weight: 600;
  color: #1F2937;
}

.nav-button {
  padding: 8px;
  border-radius: 8px;
  background: #F3F4F6;
  border: none;
  cursor: pointer;
  transition: all 0.2s;
  color: #6B7280;
}

.nav-button:hover {
  background: #E5E7EB;
  color: #1F2937;
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #E5E7EB;
  border-top-color: #3B82F6;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.calendar-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 8px;
  margin-bottom: 20px;
}

.weekday-header {
  text-align: center;
  font-size: 12px;
  font-weight: 600;
  color: #6B7280;
  padding: 8px 0;
}

.calendar-day {
  aspect-ratio: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  position: relative;
  transition: all 0.2s;
}

.day-number {
  font-size: 14px;
  margin-bottom: 2px;
}

.availability-badge {
  font-size: 9px;
  color: #059669;
  font-weight: 600;
}

.empty-day {
  background: transparent;
}

.available-day {
  background: #F0FDF4;
  border: 2px solid #BBF7D0;
  color: #065F46;
  cursor: pointer;
}

.available-day:hover {
  background: #DCFCE7;
  transform: scale(1.05);
  box-shadow: 0 2px 8px rgba(16, 185, 129, 0.2);
}

.full-day {
  background: #FEE2E2;
  border: 2px solid #FECACA;
  color: #991B1B;
  cursor: not-allowed;
  opacity: 0.7;
}

.weekend-day {
  background: #F9FAFB;
  color: #9CA3AF;
  cursor: not-allowed;
}

.selected-day {
  background: #3B82F6 !important;
  color: white !important;
  transform: scale(1.1);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.4);
}

.selected-day .availability-badge {
  color: white;
}

.legend {
  display: flex;
  justify-content: center;
  gap: 20px;
  padding: 16px 0;
  border-top: 1px solid #E5E7EB;
  border-bottom: 1px solid #E5E7EB;
  margin-bottom: 16px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: #6B7280;
}

.legend-dot {
  width: 12px;
  height: 12px;
  border-radius: 4px;
}

.legend-dot.available {
  background: #BBF7D0;
  border: 2px solid #10B981;
}

.legend-dot.full {
  background: #FECACA;
  border: 2px solid #EF4444;
}

.legend-dot.weekend {
  background: #F9FAFB;
  border: 2px solid #D1D5DB;
}

.selected-info {
  text-align: center;
  padding: 16px;
  background: #EFF6FF;
  border-radius: 12px;
}

.confirm-button {
  margin-top: 12px;
  width: 100%;
  padding: 12px 24px;
  background: #3B82F6;
  color: white;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.confirm-button:hover {
  background: #2563EB;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}
</style>
