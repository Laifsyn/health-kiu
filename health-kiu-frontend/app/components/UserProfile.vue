<template>
  <div class="user-profile">
    <div class="user-info" v-if="user">
      <div class="user-avatar">
        {{ user.name ? user.name[0].toUpperCase() : user.cedula[0] }}
      </div>
      <div class="user-details">
        <p class="user-name">{{ user.name || 'Usuario' }}</p>
        <p class="user-role">{{ user.role === 'doctor' ? 'Doctor' : 'Paciente' }}</p>
      </div>
    </div>
    <button @click="handleLogout" class="logout-button">
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
        <polyline points="16 17 21 12 16 7"></polyline>
        <line x1="21" y1="12" x2="9" y2="12"></line>
      </svg>
      Salir
    </button>
  </div>
</template>

<script setup lang="ts">
const { user, logout } = useAuth()

const handleLogout = () => {
  if (confirm('¿Estás seguro que deseas cerrar sesión?')) {
    logout()
  }
}
</script>

<style scoped>
.user-profile {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 15px 20px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  margin-bottom: 20px;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.user-avatar {
  width: 45px;
  height: 45px;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  font-size: 18px;
}

.user-details {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.user-name {
  margin: 0;
  font-weight: 600;
  font-size: 16px;
  color: #333;
}

.user-role {
  margin: 0;
  font-size: 13px;
  color: #888;
}

.logout-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: #f44336;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.logout-button:hover {
  background: #d32f2f;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(244, 67, 54, 0.3);
}

.logout-button svg {
  width: 18px;
  height: 18px;
}
</style>
