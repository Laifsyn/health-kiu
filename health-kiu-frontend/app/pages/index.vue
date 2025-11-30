<template>
  <div class="hospital-app">
    <!-- CSS Panama Header -->
    <header class="css-header">
      <div class="header-content">
        <div class="logo-container">
          <img src="/logoCajaVirtual.webp" alt="Logo CSS" class="header-logo" />
        </div>
        <div class="hospital-info">
          <h1>Caja de Seguro Social</h1>
          <p class="hospital-location">República de Panamá</p>
        </div>
        <button class="header-action" @click="logout">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
            <polyline points="16 17 21 12 16 7"></polyline>
            <line x1="21" y1="12" x2="9" y2="12"></line>
          </svg>
        </button>
      </div>
      <div class="user-info-bar">
        <span class="user-name">{{ user?.name || 'Usuario' }}</span>
        <span class="user-badge">{{ user?.role === 'doctor' ? 'Médico' : 'Paciente' }}</span>
      </div>
    </header>

    <!-- Main Services Grid -->
    <div class="main-content">
      <div class="services-grid">
        <NuxtLink to="/appointments" class="service-card primary">
          <div class="service-icon blue">
            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
              <line x1="16" y1="2" x2="16" y2="6"></line>
              <line x1="8" y1="2" x2="8" y2="6"></line>
              <line x1="3" y1="10" x2="21" y2="10"></line>
            </svg>
          </div>
          <p class="service-title">Cita<br>Ambulatoria</p>
        </NuxtLink>

        <NuxtLink to="/specialties" class="service-card primary">
          <div class="service-icon pink">
            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
              <circle cx="9" cy="7" r="4"></circle>
              <path d="M23 21v-2a4 4 0 0 0-3-3.87"></path>
              <path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
            </svg>
          </div>
          <p class="service-title">Médicos<br>Especialistas</p>
        </NuxtLink>

        <NuxtLink to="/my-appointments" class="service-card primary">
          <div class="service-icon orange">
            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
              <polyline points="9 11 12 14 22 4"></polyline>
            </svg>
          </div>
          <p class="service-title">Mis Citas<br>Agendadas</p>
        </NuxtLink>

        <div class="service-card primary">
          <div class="service-icon green">
            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 12h-4l-3 9L9 3l-3 9H2"></path>
            </svg>
          </div>
          <p class="service-title">Diagnóstico<br>Integral</p>
        </div>
      </div>

      <!-- Featured Services -->
      <div class="featured-services">
        <div class="service-banner">
          <div class="banner-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
              <polyline points="9 22 9 12 15 12 15 22"></polyline>
            </svg>
          </div>
          <div class="banner-text">
            <p class="banner-title">Médico de Familia</p>
            <p class="banner-subtitle">Atención Primaria</p>
          </div>
        </div>

        <div class="service-banner green">
          <div class="banner-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"></path>
            </svg>
          </div>
          <div class="banner-text">
            <p class="banner-title">Centro de Gestión de Salud</p>
            <p class="banner-subtitle">Prevención y Bienestar</p>
          </div>
        </div>
      </div>

      <!-- Quick Actions Grid -->
      <div class="quick-actions">
        <div class="action-card">
          <div class="action-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
              <polyline points="14 2 14 8 20 8"></polyline>
            </svg>
          </div>
          <p class="action-title">Registro<br>Ambulatorio</p>
        </div>

        <div class="action-card">
          <div class="action-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="1" y="4" width="22" height="16" rx="2" ry="2"></rect>
              <line x1="1" y1="10" x2="23" y2="10"></line>
            </svg>
          </div>
          <p class="action-title">Pago<br>Rápido</p>
        </div>

        <div class="action-card">
          <div class="action-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
              <polyline points="14 2 14 8 20 8"></polyline>
              <line x1="16" y1="13" x2="8" y2="13"></line>
              <line x1="16" y1="17" x2="8" y2="17"></line>
            </svg>
          </div>
          <p class="action-title">Consulta<br>Informes</p>
        </div>

        <div class="action-card">
          <div class="action-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"></circle>
              <polyline points="12 6 12 12 16 14"></polyline>
            </svg>
          </div>
          <p class="action-title">Historial<br>Médico</p>
        </div>
      </div>

      <!-- Bottom illustration -->
      <div class="bottom-illustration">
        <svg viewBox="0 0 400 100" xmlns="http://www.w3.org/2000/svg">
          <rect x="20" y="40" width="30" height="60" fill="#e8e8e8" opacity="0.6"/>
          <rect x="60" y="20" width="35" height="80" fill="#e8e8e8" opacity="0.4"/>
          <rect x="105" y="35" width="40" height="65" fill="#e8e8e8" opacity="0.5"/>
          <rect x="155" y="25" width="32" height="75" fill="#e8e8e8" opacity="0.3"/>
          <rect x="195" y="45" width="38" height="55" fill="#e8e8e8" opacity="0.6"/>
          <rect x="245" y="30" width="35" height="70" fill="#e8e8e8" opacity="0.4"/>
          <rect x="290" y="40" width="30" height="60" fill="#e8e8e8" opacity="0.5"/>
          <rect x="330" y="20" width="40" height="80" fill="#e8e8e8" opacity="0.3"/>
        </svg>
      </div>
    </div>

    <!-- Bottom Navigation -->
    <nav class="bottom-nav">
      <NuxtLink to="/" class="nav-item active">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
        </svg>
        <span>Inicio</span>
      </NuxtLink>

      <NuxtLink to="/services" class="nav-item">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="7" height="7"></rect>
          <rect x="14" y="3" width="7" height="7"></rect>
          <rect x="14" y="14" width="7" height="7"></rect>
          <rect x="3" y="14" width="7" height="7"></rect>
        </svg>
        <span>Servicios</span>
      </NuxtLink>

      <NuxtLink to="/appointments" class="nav-item">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
          <line x1="16" y1="2" x2="16" y2="6"></line>
          <line x1="8" y1="2" x2="8" y2="6"></line>
          <line x1="3" y1="10" x2="21" y2="10"></line>
        </svg>
        <span>Citas</span>
      </NuxtLink>

      <NuxtLink to="/profile" class="nav-item">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
          <circle cx="12" cy="7" r="4"></circle>
        </svg>
        <span>Perfil</span>
      </NuxtLink>
    </nav>
  </div>
</template>

<script setup lang="ts">
// Apply auth middleware
definePageMeta({
  middleware: 'auth'
})

const { user, logout: authLogout, loadUserFromStorage } = useAuth()

// Load user from storage on mount
onMounted(() => {
  loadUserFromStorage()
})

const logout = () => {
  authLogout()
}
</script>

<style scoped>
.hospital-app {
  min-height: 100vh;
  background: #f5f5f5;
  padding-bottom: 70px;
}

/* CSS Panama Header */
.css-header {
  background: linear-gradient(135deg, #0066b2 0%, #004d8c 100%);
  color: white;
  padding: 15px 20px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  gap: 12px;
}

.logo-container {
  flex-shrink: 0;
}

.header-logo {
  width: 45px;
  height: 45px;
  object-fit: contain;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.15));
}

.hospital-info {
  flex: 1;
}

.hospital-info h1 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.hospital-location {
  margin: 4px 0 0;
  font-size: 13px;
  opacity: 0.9;
}

.header-action {
  background: rgba(255, 255, 255, 0.2);
  border: none;
  color: white;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background 0.2s;
}

.header-action:hover {
  background: rgba(255, 255, 255, 0.3);
}

.user-info-bar {
  display: flex;
  align-items: center;
  gap: 10px;
  padding-top: 10px;
  border-top: 1px solid rgba(255, 255, 255, 0.2);
}

.user-name {
  font-size: 14px;
  font-weight: 500;
}

.user-badge {
  background: rgba(255, 255, 255, 0.2);
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
}

/* Main Content */
.main-content {
  padding: 20px 15px;
  max-width: 600px;
  margin: 0 auto;
}

/* Services Grid */
.services-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 15px;
  margin-bottom: 20px;
}

.service-card {
  background: white;
  border-radius: 12px;
  padding: 15px 10px;
  text-align: center;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transition: transform 0.2s, box-shadow 0.2s;
  cursor: pointer;
  text-decoration: none;
  color: inherit;
}

.service-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
}

.service-icon {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  margin: 0 auto 10px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.service-icon.blue {
  background: linear-gradient(135deg, #e3f2fd 0%, #bbdefb 100%);
  color: #1976d2;
}

.service-icon.pink {
  background: linear-gradient(135deg, #fce4ec 0%, #f8bbd0 100%);
  color: #c2185b;
}

.service-icon.orange {
  background: linear-gradient(135deg, #fff3e0 0%, #ffe0b2 100%);
  color: #f57c00;
}

.service-icon.green {
  background: linear-gradient(135deg, #e8f5e9 0%, #c8e6c9 100%);
  color: #388e3c;
}

.service-title {
  font-size: 12px;
  line-height: 1.4;
  margin: 0;
  color: #333;
  font-weight: 500;
}

/* Featured Services */
.featured-services {
  display: grid;
  gap: 12px;
  margin-bottom: 20px;
}

.service-banner {
  background: linear-gradient(135deg, #e3f2fd 0%, #f0f8ff 100%);
  border-radius: 12px;
  padding: 15px;
  display: flex;
  align-items: center;
  gap: 15px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  cursor: pointer;
  transition: transform 0.2s;
}

.service-banner:hover {
  transform: translateY(-2px);
}

.service-banner.green {
  background: linear-gradient(135deg, #e8f5e9 0%, #f0fff0 100%);
}

.banner-icon {
  width: 50px;
  height: 50px;
  background: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #0066b2;
  flex-shrink: 0;
}

.service-banner.green .banner-icon {
  color: #388e3c;
}

.banner-text {
  flex: 1;
}

.banner-title {
  margin: 0 0 4px;
  font-size: 15px;
  font-weight: 600;
  color: #333;
}

.banner-subtitle {
  margin: 0;
  font-size: 13px;
  color: #666;
}

/* Quick Actions */
.quick-actions {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  margin-bottom: 30px;
}

.action-card {
  background: white;
  border-radius: 12px;
  padding: 20px 15px;
  text-align: center;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  cursor: pointer;
  transition: transform 0.2s;
}

.action-card:hover {
  transform: translateY(-2px);
}

.action-icon {
  color: #0066b2;
  margin-bottom: 10px;
}

.action-title {
  font-size: 13px;
  line-height: 1.4;
  margin: 0;
  color: #333;
  font-weight: 500;
}

/* Bottom Illustration */
.bottom-illustration {
  margin-top: 40px;
  opacity: 0.6;
}

.bottom-illustration svg {
  width: 100%;
  height: auto;
}

/* Bottom Navigation */
.bottom-nav {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  background: white;
  border-top: 1px solid #e0e0e0;
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  padding: 8px 0;
  box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.05);
  z-index: 100;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  padding: 6px;
  text-decoration: none;
  color: #999;
  font-size: 11px;
  transition: color 0.2s;
}

.nav-item:hover,
.nav-item.active {
  color: #0066b2;
}

.nav-item svg {
  width: 22px;
  height: 22px;
}

/* Responsive */
@media (max-width: 480px) {
  .services-grid {
    grid-template-columns: repeat(4, 1fr);
    gap: 10px;
  }

  .service-icon {
    width: 50px;
    height: 50px;
  }

  .service-title {
    font-size: 11px;
  }
}
</style>
