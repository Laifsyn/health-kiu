<template>
  <div class="hospital-header">
    <div class="header-content">
      <div class="hospital-info">
        <div class="hospital-logo">
          <svg width="40" height="40" viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="20" cy="20" r="18" fill="white" opacity="0.9"/>
            <path d="M20 8 L20 32 M8 20 L32 20" stroke="#8B4A5A" stroke-width="4" stroke-linecap="round"/>
          </svg>
        </div>
        <div class="hospital-details">
          <div class="hospital-name">
            <span class="spanish-title">Caja de Seguro Social</span>
            <button class="switch-btn">[Cambiar]</button>
          </div>
          <div class="hospital-address">
            <span class="spanish">Policlínica Dr. Carlos N. Brin, Vía España, Panamá</span>
          </div>
        </div>
      </div>
      <button class="qr-btn">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect x="3" y="3" width="8" height="8" stroke="white" stroke-width="2"/>
          <rect x="13" y="3" width="8" height="8" stroke="white" stroke-width="2"/>
          <rect x="3" y="13" width="8" height="8" stroke="white" stroke-width="2"/>
          <rect x="13" y="13" width="8" height="8" stroke="white" stroke-width="2"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.hospital-header {
  background: linear-gradient(135deg, #8B4A5A 0%, #A05868 100%);
  color: white;
  padding: 12px 16px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  max-width: 100%;
}

.hospital-info {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}

.hospital-logo {
  flex-shrink: 0;
}

.hospital-details {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.hospital-name {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
}

.spanish-title {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-size: 16px;
  font-weight: 600;
}

.switch-btn {
  background: transparent;
  border: none;
  color: white;
  font-size: 12px;
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 4px;
  transition: background 0.2s;
  flex-shrink: 0;
}

.switch-btn:hover {
  background: rgba(255, 255, 255, 0.1);
}

.hospital-address {
  font-size: 11px;
  opacity: 0.9;
  display: flex;
  flex-direction: column;
}

.spanish {
  font-size: 10px;
  opacity: 0.8;
  color: #B8E5F0;
}

.qr-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  transition: background 0.2s;
  flex-shrink: 0;
}

.qr-btn:hover {
  background: rgba(255, 255, 255, 0.1);
}
</style>
