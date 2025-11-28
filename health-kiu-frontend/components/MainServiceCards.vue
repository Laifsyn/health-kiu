<template>
  <div class="main-services-grid">
    <div v-for="service in services" :key="service.id" :class="['service-card', service.colorClass]">
      <div class="service-icon" v-html="service.icon"></div>
      <div class="service-text">
        <div class="service-title">{{ service.title }}</div>
        <div class="service-subtitle" v-if="service.subtitle">{{ service.subtitle }}</div>
      </div>
    </div>
  </div>
</template>

<script setup>
const services = [
  {
    id: 1,
    title: 'Cita Ambulatoria',
    subtitle: 'Reserva de consultas',
    colorClass: 'bg-blue-50',
    icon: `<svg width="48" height="48" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M24 8C18 8 13 13 13 19C13 25 18 30 24 30C30 30 35 25 35 19C35 13 30 8 24 8Z" stroke="#2563EB" stroke-width="2.5" fill="none"/>
      <path d="M30 20C30 20 28 24 24 24C20 24 18 20 18 20" stroke="#2563EB" stroke-width="2.5" stroke-linecap="round"/>
      <path d="M16 28C16 28 18 32 24 32M32 28C32 28 30 32 24 32" stroke="#2563EB" stroke-width="2.5" stroke-linecap="round"/>
      <ellipse cx="24" cy="35" rx="7" ry="3" stroke="#2563EB" stroke-width="2" fill="none"/>
    </svg>`
  },
  {
    id: 2,
    title: 'Especialidades',
    subtitle: 'Consultas especializadas',
    colorClass: 'bg-pink-50',
    icon: `<svg width="48" height="48" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="24" cy="16" r="6" stroke="#EC4899" stroke-width="2.5" fill="none"/>
      <path d="M18 38C18 31 20 26 24 26C28 26 30 31 30 38" stroke="#EC4899" stroke-width="2.5" stroke-linecap="round"/>
      <path d="M15 30H12V33M33 30H36V33" stroke="#EC4899" stroke-width="2" stroke-linecap="round"/>
      <rect x="20" y="20" width="3" height="4" fill="#EC4899"/>
      <rect x="25" y="20" width="3" height="4" fill="#EC4899"/>
    </svg>`
  },
  {
    id: 3,
    title: 'Registro Hoy',
    subtitle: 'Atención del mismo día',
    colorClass: 'bg-orange-50',
    icon: `<svg width="48" height="48" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="12" y="10" width="24" height="28" rx="2" stroke="#F97316" stroke-width="2.5" fill="none"/>
      <path d="M17 6V14M31 6V14" stroke="#F97316" stroke-width="2.5" stroke-linecap="round"/>
      <path d="M12 18H36" stroke="#F97316" stroke-width="2.5"/>
      <path d="M20 24L23 27L28 22" stroke="#F97316" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>`
  },
  {
    id: 4,
    title: 'Multidisciplinario',
    subtitle: 'Diagnóstico integral',
    colorClass: 'bg-green-50',
    icon: `<svg width="48" height="48" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="24" cy="18" r="6" stroke="#10B981" stroke-width="2.5" fill="none"/>
      <path d="M16 38C16 31 19 26 24 26C29 26 32 31 32 38" stroke="#10B981" stroke-width="2.5" stroke-linecap="round"/>
      <path d="M24 28L24 20M20 24L28 24" stroke="#10B981" stroke-width="2.5" stroke-linecap="round"/>
      <circle cx="24" cy="24" r="3" fill="#10B981"/>
    </svg>`
  }
];
</script>

<style scoped>
.main-services-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  padding: 16px;
  background: #f8f9fa;
}

.service-card {
  border-radius: 16px;
  padding: 20px 16px;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 12px;
  cursor: pointer;
  transition: all 0.3s ease;
  border: 2px solid transparent;
  min-height: 140px;
}

.service-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
}

.bg-blue-50 {
  background: linear-gradient(135deg, #EFF6FF 0%, #DBEAFE 100%);
  border-color: #BFDBFE;
}

.bg-pink-50 {
  background: linear-gradient(135deg, #FDF2F8 0%, #FCE7F3 100%);
  border-color: #FBCFE8;
}

.bg-orange-50 {
  background: linear-gradient(135deg, #FFF7ED 0%, #FFEDD5 100%);
  border-color: #FED7AA;
}

.bg-green-50 {
  background: linear-gradient(135deg, #F0FDF4 0%, #DCFCE7 100%);
  border-color: #BBF7D0;
}

.service-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.service-text {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.service-title {
  font-size: 15px;
  font-weight: 600;
  color: #1F2937;
  line-height: 1.3;
}

.service-subtitle {
  font-size: 12px;
  font-weight: 400;
  color: #6B7280;
  line-height: 1.3;
  margin-top: 2px;
}
</style>
