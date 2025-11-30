<template>
  <div class="login-page">
    <!-- CSS Panama Header -->
    <div class="css-header">
      <div class="css-logo">
        <img src="/logoCajaVirtual.webp" alt="Logo Caja de Seguro Social" class="logo-image" />
      </div>
      <h1 class="css-title">Caja de Seguro Social</h1>
      <p class="css-subtitle">República de Panamá</p>
      <p class="system-name">Sistema de Agendamiento de Citas</p>
    </div>

    <!-- Login Form -->
    <div class="login-content">
      <div class="welcome-section">
        <h2 class="welcome-title">Bienvenido</h2>
        <p class="welcome-subtitle">Ingrese sus credenciales para acceder al sistema</p>
      </div>

      <div v-if="error" class="error-alert">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"></circle>
          <line x1="12" y1="8" x2="12" y2="12"></line>
          <line x1="12" y1="16" x2="12.01" y2="16"></line>
        </svg>
        {{ error }}
      </div>

      <form @submit.prevent="handleLogin" class="login-form">
        <div class="form-group">
          <label class="form-label">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
              <circle cx="12" cy="7" r="4"></circle>
            </svg>
            Tipo de Usuario
          </label>
          <select
            v-model="formData.role"
            class="form-input"
            required
          >
            <option value="patient">👤 Paciente</option>
            <option value="doctor">👨‍⚕️ Médico</option>
          </select>
        </div>

        <div class="form-group">
          <label class="form-label">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
              <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
            </svg>
            Cédula
          </label>
          <input
            v-model="formData.cedula"
            type="text"
            class="form-input"
            placeholder="8-123-4567"
            required
          />
        </div>

        <div class="form-group">
          <label class="form-label">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
              <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
            </svg>
            Contraseña
          </label>
          <input
            v-model="formData.password"
            type="password"
            class="form-input"
            placeholder="Ingrese su contraseña"
            required
          />
        </div>

        <button
          type="submit"
          class="submit-btn"
          :disabled="loading"
        >
          <svg v-if="!loading" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"></path>
            <polyline points="10 17 15 12 10 7"></polyline>
            <line x1="15" y1="12" x2="3" y2="12"></line>
          </svg>
          <span v-if="loading">Iniciando sesión...</span>
          <span v-else>Iniciar Sesión</span>
        </button>
      </form>

      <div class="register-section">
        <p class="register-text">¿Primera vez en el sistema?</p>
        <NuxtLink to="/register" class="register-link">
          Crear una cuenta nueva
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="9 18 15 12 9 6"></polyline>
          </svg>
        </NuxtLink>
      </div>
    </div>

    <!-- Footer -->
    <div class="login-footer">
      <p>© 2025 Caja de Seguro Social - Panamá</p>
      <p class="footer-info">Sistema de Agendamiento de Citas Médicas</p>
    </div>
  </div>
</template>

<script setup lang="ts">
const router = useRouter()
const { login, isAuthenticated } = useAuth()

const formData = reactive({
  cedula: '',
  password: '',
  role: 'patient' as 'patient' | 'doctor'
})

const loading = ref(false)
const error = ref('')

// Redirect if already logged in
onMounted(() => {
  if (isAuthenticated.value) {
    router.push('/')
  }
})

const handleLogin = async () => {
  try {
    loading.value = true
    error.value = ''

    await login({
      cedula: formData.cedula,
      password: formData.password,
      role: formData.role
    })

    // Redirect to home
    router.push('/')
  } catch (e: any) {
    error.value = e.message || 'Error al iniciar sesión'
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-page {
  min-height: 100vh;
  background: #f5f7fa;
  display: flex;
  flex-direction: column;
}

/* CSS Panama Header */
.css-header {
  background: linear-gradient(135deg, #0066b2 0%, #004d8c 100%);
  color: white;
  padding: 40px 20px 30px;
  text-align: center;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.css-logo {
  margin-bottom: 20px;
}

.logo-image {
  width: 100px;
  height: 100px;
  object-fit: contain;
  margin: 0 auto;
  display: block;
  filter: drop-shadow(0 2px 8px rgba(0, 0, 0, 0.2));
}

.css-title {
  font-size: 24px;
  font-weight: 700;
  margin: 0 0 8px;
}

.css-subtitle {
  font-size: 14px;
  margin: 0 0 6px;
  opacity: 0.95;
}

.system-name {
  font-size: 13px;
  margin: 0;
  opacity: 0.85;
  font-weight: 300;
}

/* Login Content */
.login-content {
  flex: 1;
  padding: 30px 20px;
  max-width: 450px;
  width: 100%;
  margin: 0 auto;
}

.welcome-section {
  text-align: center;
  margin-bottom: 30px;
}

.welcome-title {
  font-size: 26px;
  font-weight: 600;
  color: #1a1a1a;
  margin: 0 0 8px;
}

.welcome-subtitle {
  font-size: 14px;
  color: #666;
  margin: 0;
}

/* Error Alert */
.error-alert {
  background: #fff3f3;
  border: 1px solid #ffcdd2;
  color: #c62828;
  padding: 14px 16px;
  border-radius: 12px;
  margin-bottom: 24px;
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 14px;
}

.error-alert svg {
  flex-shrink: 0;
}

/* Form */
.login-form {
  background: white;
  padding: 30px 24px;
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
}

.form-group {
  margin-bottom: 24px;
}

.form-label {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
  font-weight: 600;
  color: #333;
  font-size: 14px;
}

.form-label svg {
  color: #0066b2;
}

.form-input {
  width: 100%;
  padding: 14px 16px;
  border: 2px solid #e1e8ed;
  border-radius: 10px;
  font-size: 15px;
  transition: all 0.3s;
  box-sizing: border-box;
  background: #fafbfc;
}

.form-input:focus {
  outline: none;
  border-color: #0066b2;
  background: white;
  box-shadow: 0 0 0 3px rgba(0, 102, 178, 0.1);
}

.submit-btn {
  width: 100%;
  padding: 16px;
  background: linear-gradient(135deg, #0066b2 0%, #004d8c 100%);
  color: white;
  border: none;
  border-radius: 12px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  margin-top: 8px;
}

.submit-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0, 102, 178, 0.3);
}

.submit-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
}

/* Register Section */
.register-section {
  text-align: center;
  margin-top: 24px;
  padding: 20px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.register-text {
  margin: 0 0 12px;
  color: #666;
  font-size: 14px;
}

.register-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  color: #0066b2;
  text-decoration: none;
  font-weight: 600;
  font-size: 15px;
  transition: all 0.2s;
  padding: 8px 16px;
  border-radius: 8px;
}

.register-link:hover {
  background: rgba(0, 102, 178, 0.08);
  gap: 8px;
}

/* Footer */
.login-footer {
  background: white;
  padding: 20px;
  text-align: center;
  border-top: 1px solid #e1e8ed;
  margin-top: auto;
}

.login-footer p {
  margin: 0;
  color: #666;
  font-size: 13px;
}

.footer-info {
  margin-top: 6px;
  color: #999;
  font-size: 12px;
}

/* Responsive */
@media (max-width: 480px) {
  .css-header {
    padding: 30px 15px 25px;
  }

  .css-title {
    font-size: 20px;
  }

  .login-content {
    padding: 20px 15px;
  }

  .login-form {
    padding: 24px 20px;
  }
}
</style>
