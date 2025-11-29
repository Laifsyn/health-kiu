<template>
  <div class="register-page">
    <!-- CSS Panama Header -->
    <div class="css-header">
      <div class="css-logo">
        <div class="logo-circle">
          <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 12h-4l-3 9L9 3l-3 9H2"></path>
          </svg>
        </div>
      </div>
      <h1 class="css-title">Caja de Seguro Social</h1>
      <p class="css-subtitle">República de Panamá</p>
      <p class="system-name">Registro de Nuevo Usuario</p>
    </div>

    <!-- Register Form -->
    <div class="register-content">
      <div class="welcome-section">
        <h2 class="welcome-title">Crear Cuenta</h2>
        <p class="welcome-subtitle">Complete el formulario para registrarse en el sistema</p>
      </div>

      <div v-if="error" class="error-alert">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"></circle>
          <line x1="12" y1="8" x2="12" y2="12"></line>
          <line x1="12" y1="16" x2="12.01" y2="16"></line>
        </svg>
        {{ error }}
      </div>

      <div v-if="success" class="success-alert">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
          <polyline points="22 4 12 14.01 9 11.01"></polyline>
        </svg>
        {{ success }}
      </div>

      <form @submit.prevent="handleRegister" class="register-form">
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
              <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
              <circle cx="12" cy="7" r="4"></circle>
            </svg>
            Nombre Completo
          </label>
          <input
            v-model="formData.name"
            type="text"
            class="form-input"
            placeholder="Ingrese su nombre completo"
            required
          />
        </div>

        <div class="form-row">
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
            />
          </div>

          <div class="form-group">
            <label class="form-label">
              <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="2" y="7" width="20" height="14" rx="2" ry="2"></rect>
                <path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"></path>
              </svg>
              Pasaporte
            </label>
            <input
              v-model="formData.passport"
              type="text"
              class="form-input"
              placeholder="Opcional"
            />
          </div>
        </div>

        <p class="field-note">
          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="12" y1="16" x2="12" y2="12"></line>
            <line x1="12" y1="8" x2="12.01" y2="8"></line>
          </svg>
          Debe proporcionar al menos cédula o pasaporte
        </p>

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
            placeholder="Mínimo 6 caracteres"
            required
            minlength="6"
          />
        </div>

        <div class="form-group">
          <label class="form-label">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
              <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
            </svg>
            Confirmar Contraseña
          </label>
          <input
            v-model="formData.confirmPassword"
            type="password"
            class="form-input"
            placeholder="Repita su contraseña"
            required
          />
        </div>

        <button
          type="submit"
          class="submit-btn"
          :disabled="loading"
        >
          <svg v-if="!loading" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
            <circle cx="8.5" cy="7" r="4"></circle>
            <line x1="20" y1="8" x2="20" y2="14"></line>
            <line x1="23" y1="11" x2="17" y2="11"></line>
          </svg>
          <span v-if="loading">Registrando...</span>
          <span v-else>Crear Cuenta</span>
        </button>
      </form>

      <div class="login-section">
        <p class="login-text">¿Ya tienes una cuenta?</p>
        <NuxtLink to="/login" class="login-link">
          Iniciar sesión
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="9 18 15 12 9 6"></polyline>
          </svg>
        </NuxtLink>
      </div>
    </div>

    <!-- Footer -->
    <div class="register-footer">
      <p>© 2025 Caja de Seguro Social - Panamá</p>
      <p class="footer-info">Sistema de Agendamiento de Citas Médicas</p>
    </div>
  </div>
</template>

<script setup lang="ts">
const router = useRouter()
const { register, isAuthenticated } = useAuth()

const formData = reactive({
  name: '',
  cedula: '',
  passport: '',
  password: '',
  confirmPassword: '',
  role: 'patient' as 'patient' | 'doctor'
})

const loading = ref(false)
const error = ref('')
const success = ref('')

// Redirect if already logged in
onMounted(() => {
  if (isAuthenticated.value) {
    router.push('/')
  }
})

const validateForm = () => {
  if (!formData.cedula && !formData.passport) {
    error.value = 'Debe proporcionar al menos cédula o pasaporte'
    return false
  }

  if (formData.password.length < 6) {
    error.value = 'La contraseña debe tener al menos 6 caracteres'
    return false
  }

  if (formData.password !== formData.confirmPassword) {
    error.value = 'Las contraseñas no coinciden'
    return false
  }

  return true
}

const handleRegister = async () => {
  try {
    loading.value = true
    error.value = ''
    success.value = ''

    if (!validateForm()) {
      loading.value = false
      return
    }

    await register({
      name: formData.name,
      cedula: formData.cedula || undefined,
      passport: formData.passport || undefined,
      password: formData.password,
      role: formData.role
    })

    success.value = 'Registro exitoso! Redirigiendo...'

    // Redirect after 1 second
    setTimeout(() => {
      router.push('/')
    }, 1000)
  } catch (e: any) {
    error.value = e.message || 'Error al registrarse'
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.register-page {
  min-height: 100vh;
  background: #f5f7fa;
  display: flex;
  flex-direction: column;
}

/* CSS Panama Header */
.css-header {
  background: linear-gradient(135deg, #0066b2 0%, #004d8c 100%);
  color: white;
  padding: 35px 20px 28px;
  text-align: center;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.css-logo {
  margin-bottom: 16px;
}

.logo-circle {
  width: 70px;
  height: 70px;
  background: rgba(255, 255, 255, 0.15);
  border-radius: 50%;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 3px solid rgba(255, 255, 255, 0.3);
}

.css-title {
  font-size: 22px;
  font-weight: 700;
  margin: 0 0 6px;
}

.css-subtitle {
  font-size: 13px;
  margin: 0 0 5px;
  opacity: 0.95;
}

.system-name {
  font-size: 13px;
  margin: 0;
  opacity: 0.85;
  font-weight: 300;
}

/* Register Content */
.register-content {
  flex: 1;
  padding: 25px 20px;
  max-width: 500px;
  width: 100%;
  margin: 0 auto;
}

.welcome-section {
  text-align: center;
  margin-bottom: 24px;
}

.welcome-title {
  font-size: 24px;
  font-weight: 600;
  color: #1a1a1a;
  margin: 0 0 6px;
}

.welcome-subtitle {
  font-size: 14px;
  color: #666;
  margin: 0;
}

/* Alerts */
.error-alert {
  background: #fff3f3;
  border: 1px solid #ffcdd2;
  color: #c62828;
  padding: 14px 16px;
  border-radius: 12px;
  margin-bottom: 20px;
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 14px;
}

.error-alert svg {
  flex-shrink: 0;
}

.success-alert {
  background: #f0f9f4;
  border: 1px solid #b7dfcc;
  color: #1b5e34;
  padding: 14px 16px;
  border-radius: 12px;
  margin-bottom: 20px;
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 14px;
}

.success-alert svg {
  flex-shrink: 0;
}

/* Form */
.register-form {
  background: white;
  padding: 28px 22px;
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
}

.form-group {
  margin-bottom: 20px;
  flex: 1;
}

.form-row {
  display: flex;
  gap: 12px;
  margin-bottom: 20px;
}

.form-label {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
  font-weight: 600;
  color: #333;
  font-size: 14px;
}

.form-label svg {
  color: #0066b2;
  flex-shrink: 0;
}

.form-input {
  width: 100%;
  padding: 13px 15px;
  border: 2px solid #e1e8ed;
  border-radius: 10px;
  font-size: 14px;
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

.field-note {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: #666;
  margin: -8px 0 16px 0;
  padding: 10px;
  background: #f8f9fa;
  border-radius: 8px;
  border-left: 3px solid #0066b2;
}

.field-note svg {
  flex-shrink: 0;
  color: #0066b2;
}

.submit-btn {
  width: 100%;
  padding: 15px;
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
  margin-top: 4px;
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

/* Login Section */
.login-section {
  text-align: center;
  margin-top: 20px;
  padding: 18px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.login-text {
  margin: 0 0 10px;
  color: #666;
  font-size: 14px;
}

.login-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  color: #0066b2;
  text-decoration: none;
  font-weight: 600;
  font-size: 15px;
  transition: all 0.2s;
  padding: 8px 14px;
  border-radius: 8px;
}

.login-link:hover {
  background: rgba(0, 102, 178, 0.08);
  gap: 8px;
}

/* Footer */
.register-footer {
  background: white;
  padding: 18px;
  text-align: center;
  border-top: 1px solid #e1e8ed;
  margin-top: auto;
}

.register-footer p {
  margin: 0;
  color: #666;
  font-size: 13px;
}

.footer-info {
  margin-top: 5px;
  color: #999;
  font-size: 12px;
}

/* Responsive */
@media (max-width: 480px) {
  .css-header {
    padding: 28px 15px 22px;
  }

  .css-title {
    font-size: 19px;
  }

  .register-content {
    padding: 20px 15px;
  }

  .register-form {
    padding: 22px 18px;
  }

  .form-row {
    flex-direction: column;
    gap: 0;
  }
}
</style>
