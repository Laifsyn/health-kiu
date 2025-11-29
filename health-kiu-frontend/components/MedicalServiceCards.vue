<template>
  <div class="medical-services-section">
    <div class="services-grid">
      <div v-for="service in services" :key="service.id" class="medical-card">
        <div class="card-badge" v-if="service.badge">
          <span class="badge-text">{{ service.badge }}</span>
        </div>
        <div class="card-icon" v-if="service.iconType === 'medal'">
          <svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="16" cy="18" r="8" stroke="#B8860B" stroke-width="2" fill="#FFD700"/>
            <path d="M16 10L12 6L10 8L14 12L16 10Z M16 10L20 6L22 8L18 12L16 10Z" fill="#B8860B"/>
            <circle cx="16" cy="18" r="4" fill="#B8860B"/>
          </svg>
        </div>
        <div class="card-icon" v-else-if="service.iconType === 'medical'">
          <svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect x="6" y="10" width="20" height="18" rx="2" fill="#DC2626"/>
            <path d="M16 14V22M12 18H20" stroke="white" stroke-width="2.5" stroke-linecap="round"/>
          </svg>
        </div>
        <div class="card-icon" v-else-if="service.iconType === 'heart'">
          <svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M16 28C16 28 4 20 4 11C4 7 7 4 11 4C13 4 15 5 16 7C17 5 19 4 21 4C25 4 28 7 28 11C28 20 16 28 16 28Z" fill="#DC2626"/>
            <path d="M10 10L13 13L16 10" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <div class="card-content">
          <div class="service-title">{{ service.title }}</div>
          <div class="service-subtitle" v-if="service.subtitle">{{ service.subtitle }}</div>
        </div>
      </div>

      <!-- Quick Action Cards -->
      <div v-for="action in quickActions" :key="action.id" class="quick-action-card">
        <div class="action-icon" v-html="action.icon"></div>
        <div class="action-content">
          <div class="service-title">{{ action.title }}</div>
          <div class="service-subtitle" v-if="action.subtitle">{{ action.subtitle }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
const services = [
  {
    id: 1,
    title: 'Citas por Telemedicina',
    subtitle: 'Consultas virtuales',
    badge: 'Online',
    iconType: null
  },
  {
    id: 2,
    title: 'Farmacias CSS',
    subtitle: 'Retiro de medicamentos',
    iconType: 'medal'
  },
  {
    id: 3,
    title: 'Médico de Familia',
    subtitle: 'Atención primaria',
    iconType: 'medical'
  },
  {
    id: 4,
    title: 'Salud Preventiva',
    subtitle: 'Chequeos y vacunación',
    iconType: 'heart'
  }
];

const quickActions = [
  {
    id: 5,
    title: 'Registro de Llegada',
    subtitle: 'Check-in de cita',
    icon: `<svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="8" y="6" width="16" height="20" rx="1" stroke="#8B4A5A" stroke-width="2" fill="none"/>
      <path d="M12 11H20M12 15H20M12 19H17" stroke="#8B4A5A" stroke-width="2" stroke-linecap="round"/>
      <circle cx="22" cy="22" r="4" fill="#8B4A5A"/>
      <path d="M20 22L21 23L24 20" stroke="white" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>`
  },
  {
    id: 6,
    title: 'Pagos en Línea',
    subtitle: 'Pago de servicios',
    icon: `<svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="8" y="6" width="16" height="20" rx="1" stroke="#F59E0B" stroke-width="2" fill="none"/>
      <path d="M12 11H20M12 15H20" stroke="#F59E0B" stroke-width="2" stroke-linecap="round"/>
      <circle cx="22" cy="22" r="4" fill="#F59E0B"/>
      <text x="22" y="25" text-anchor="middle" fill="white" font-size="10" font-weight="bold">$</text>
    </svg>`
  },
  {
    id: 7,
    title: 'Solicitar Referencia',
    subtitle: 'Referencias médicas',
    icon: `<svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="8" y="6" width="16" height="20" rx="1" stroke="#8B4A5A" stroke-width="2" fill="none"/>
      <path d="M12 11H20M12 15H18" stroke="#8B4A5A" stroke-width="2" stroke-linecap="round"/>
      <path d="M16 19V23M13 21H19" stroke="#8B4A5A" stroke-width="2" stroke-linecap="round"/>
    </svg>`
  },
  {
    id: 8,
    title: 'Ver Resultados',
    subtitle: 'Laboratorio e imágenes',
    icon: `<svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="8" y="6" width="16" height="20" rx="1" stroke="#8B4A5A" stroke-width="2" fill="none"/>
      <path d="M12 11H20M12 15H20M12 19H17" stroke="#8B4A5A" stroke-width="2" stroke-linecap="round"/>
      <circle cx="21" cy="21" r="5" fill="white" stroke="#8B4A5A" stroke-width="2"/>
      <path d="M21 18V21L23 23" stroke="#8B4A5A" stroke-width="1.5" stroke-linecap="round"/>
    </svg>`
  }
];
</script>

<style scoped>
.medical-services-section {
  padding: 0 16px 16px;
  background: #f8f9fa;
}

.services-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.medical-card {
  background: linear-gradient(135deg, #FCE7F3 0%, #FBCFE8 100%);
  border-radius: 12px;
  padding: 16px;
  display: flex;
  align-items: center;
  gap: 12px;
  position: relative;
  cursor: pointer;
  transition: all 0.3s ease;
  border: 2px solid #F9A8D4;
  min-height: 70px;
}

.medical-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(236, 72, 153, 0.2);
}

.card-badge {
  position: absolute;
  top: -4px;
  left: 12px;
  background: #10B981;
  color: white;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
}

.badge-text {
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.card-icon {
  flex-shrink: 0;
}

.card-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.quick-action-card {
  background: white;
  border-radius: 12px;
  padding: 16px;
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
  transition: all 0.3s ease;
  border: 2px solid #E5E7EB;
  min-height: 70px;
}

.quick-action-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  border-color: #8B4A5A;
}

.action-icon {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.service-title {
  font-size: 14px;
  font-weight: 600;
  color: #1F2937;
  line-height: 1.3;
}

.service-subtitle {
  font-size: 11px;
  color: #6B7280;
  line-height: 1.3;
  margin-top: 2px;
}
</style>
