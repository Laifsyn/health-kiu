<template>
  <div class="appointments-page">
    <div class="page-header">
      <div class="header-content">
        <NuxtLink to="/" class="back-btn">
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M19 12H5M12 19l-7-7 7-7"/>
          </svg>
          Volver al inicio
        </NuxtLink>
        <div class="header-text">
          <h1>Mis Citas Agendadas</h1>
          <p>Consulta todas tus citas médicas programadas</p>
        </div>
      </div>
    </div>

    <div v-if="loading" class="loading">
      <p>Cargando tus citas...</p>
    </div>

    <div v-else-if="error" class="error-message">
      <p>Error al cargar las citas</p>
      <button @click="loadAppointments" class="retry-btn">Reintentar</button>
    </div>

    <div v-else-if="appointments.length === 0" class="empty-state">
      <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
        <line x1="16" y1="2" x2="16" y2="6"></line>
        <line x1="8" y1="2" x2="8" y2="6"></line>
        <line x1="3" y1="10" x2="21" y2="10"></line>
      </svg>
      <p>No tienes citas agendadas</p>
      <NuxtLink to="/specialties" class="book-btn">Agendar una cita</NuxtLink>
    </div>

    <div v-else class="appointments-grid">
      <div
        v-for="appointment in appointments"
        :key="appointment.id"
        class="appointment-card"
      >
        <div class="card-header">
          <div class="specialty-badge">
            {{ appointment.specialty_name }}
          </div>
          <div :class="['status-badge', getStatusClass(appointment.status)]">
            {{ getStatusText(appointment.status) }}
          </div>
        </div>

        <div class="doctor-info">
          <div class="doctor-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
              <circle cx="12" cy="7" r="4"></circle>
            </svg>
          </div>
          <div>
            <h3>{{ appointment.doctor_name }}</h3>
            <p class="specialty-text">{{ appointment.specialty_name }}</p>
          </div>
        </div>

        <div class="appointment-details">
          <div class="detail-row">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
              <line x1="16" y1="2" x2="16" y2="6"></line>
              <line x1="8" y1="2" x2="8" y2="6"></line>
              <line x1="3" y1="10" x2="21" y2="10"></line>
            </svg>
            <span>{{ formatDate(appointment.date) }}</span>
          </div>

          <div class="detail-row">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"></circle>
              <polyline points="12 6 12 12 16 14"></polyline>
            </svg>
            <span>{{ appointment.time }}</span>
          </div>

          <div v-if="appointment.hospital" class="detail-row">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
              <polyline points="9 22 9 12 15 12 15 22"></polyline>
            </svg>
            <span>{{ appointment.hospital }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { fetchPatientAppointments } = useApi()
const { user } = useAuth()

const appointments = ref<any[]>([])
const loading = ref(true)
const error = ref(false)

const loadAppointments = async () => {
  // Load user from storage first
  const { loadUserFromStorage } = useAuth()
  loadUserFromStorage()

  if (!user.value?.id) {
    // If no user, just show empty state, not error
    loading.value = false
    return
  }

  try {
    loading.value = true
    error.value = false
    const data = await fetchPatientAppointments(user.value.id)
    appointments.value = data || []
  } catch (e) {
    console.error('Error loading appointments:', e)
    // Don't show error for empty responses
    appointments.value = []
  } finally {
    loading.value = false
  }
}

const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  return date.toLocaleDateString('es-ES', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  })
}

const getStatusClass = (status: string) => {
  const statusMap: Record<string, string> = {
    'scheduled': 'status-scheduled',
    'confirmed': 'status-confirmed',
    'completed': 'status-completed',
    'cancelled': 'status-cancelled'
  }
  return statusMap[status.toLowerCase()] || 'status-scheduled'
}

const getStatusText = (status: string) => {
  const statusMap: Record<string, string> = {
    'scheduled': 'Programada',
    'confirmed': 'Confirmada',
    'completed': 'Completada',
    'cancelled': 'Cancelada'
  }
  return statusMap[status.toLowerCase()] || status
}

onMounted(() => {
  loadAppointments()
})
</script>

<style scoped>
.appointments-page {
  min-height: 100vh;
  background: #f5f7fa;
  padding: 20px;
}

.page-header {
  margin-bottom: 40px;
  padding: 30px 20px;
  background: white;
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
}

.header-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.back-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  color: #0066b2;
  text-decoration: none;
  font-weight: 600;
  font-size: 15px;
  transition: all 0.2s;
  align-self: flex-start;
}

.back-btn:hover {
  gap: 12px;
}

.back-btn svg {
  transition: transform 0.2s;
}

.back-btn:hover svg {
  transform: translateX(-4px);
}

.header-text {
  text-align: center;
}

.page-header h1 {
  font-size: 28px;
  font-weight: 700;
  color: #1a1a1a;
  margin: 0 0 10px;
}

.page-header p {
  font-size: 16px;
  color: #666;
  margin: 0;
}

.loading, .error-message, .empty-state {
  text-align: center;
  padding: 60px 20px;
  background: white;
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
}

.loading p, .error-message p, .empty-state p {
  font-size: 16px;
  color: #666;
  margin: 20px 0;
}

.empty-state svg {
  color: #999;
  margin-bottom: 20px;
}

.retry-btn, .book-btn {
  padding: 12px 24px;
  background: linear-gradient(135deg, #0066b2 0%, #004d8c 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
  text-decoration: none;
  display: inline-block;
  margin-top: 10px;
}

.retry-btn:hover, .book-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0, 102, 178, 0.3);
}

.appointments-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.appointment-card {
  background: white;
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
  padding: 24px;
  transition: all 0.3s;
}

.appointment-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.specialty-badge {
  background: linear-gradient(135deg, #0066b2 0%, #004d8c 100%);
  color: white;
  padding: 6px 12px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 600;
}

.status-badge {
  padding: 6px 12px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 600;
}

.status-scheduled {
  background: #e3f2fd;
  color: #1976d2;
}

.status-confirmed {
  background: #e8f5e9;
  color: #388e3c;
}

.status-completed {
  background: #f3e5f5;
  color: #7b1fa2;
}

.status-cancelled {
  background: #ffebee;
  color: #d32f2f;
}

.doctor-info {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 20px;
  padding-bottom: 20px;
  border-bottom: 1px solid #eee;
}

.doctor-icon {
  width: 48px;
  height: 48px;
  background: #f5f7fa;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #0066b2;
}

.doctor-info h3 {
  font-size: 18px;
  font-weight: 600;
  color: #1a1a1a;
  margin: 0 0 4px;
}

.specialty-text {
  font-size: 14px;
  color: #666;
  margin: 0;
}

.appointment-details {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.detail-row {
  display: flex;
  align-items: center;
  gap: 12px;
  color: #666;
  font-size: 14px;
}

.detail-row svg {
  color: #0066b2;
  flex-shrink: 0;
}

@media (max-width: 768px) {
  .appointments-grid {
    grid-template-columns: 1fr;
    gap: 16px;
  }

  .page-header {
    padding: 20px 16px;
    margin-bottom: 24px;
  }

  .page-header h1 {
    font-size: 24px;
  }
}
</style>
