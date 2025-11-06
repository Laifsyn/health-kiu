<template>
  <div class="min-h-screen flex items-center justify-center relative overflow-hidden" style="background: linear-gradient(135deg, var(--primary-400) 0%, var(--primary-600) 50%, var(--primary-800) 100%);">
    <!-- Background pattern -->
    <div class="absolute inset-0 opacity-10">
      <div class="absolute top-10 left-10 w-20 h-20 rounded-full border-2 border-white"></div>
      <div class="absolute top-32 right-20 w-16 h-16 rounded-full border-2 border-white"></div>
      <div class="absolute bottom-20 left-32 w-24 h-24 rounded-full border-2 border-white"></div>
      <div class="absolute bottom-32 right-10 w-12 h-12 rounded-full border-2 border-white"></div>
      <div class="absolute top-1/2 left-1/4 w-8 h-8 rounded-full border-2 border-white"></div>
      <div class="absolute top-1/3 right-1/3 w-14 h-14 rounded-full border-2 border-white"></div>
    </div>

    <!-- Login Form Container -->
    <div class="bg-white/90 backdrop-blur-sm p-8 rounded-2xl shadow-2xl w-full max-w-md mx-4 relative z-10">
      <!-- Logo/Brand Area -->
      <div class="text-center mb-8">
        <div class="w-16 h-16 mx-auto mb-4 rounded-full flex items-center justify-center" style="background-color: var(--primary-600);">
          <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--white);">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
          </svg>
        </div>
        <h1 class="text-3xl font-bold mb-2" style="color: var(--gray-900);">Health Kiu</h1>
        <p class="text-sm opacity-70" style="color: var(--gray-600);">Bienvenido al sistema de gestión médica</p>
      </div>

      <!-- Login Form -->
      <form @submit.prevent="handleLogin" class="space-y-6">
        <!-- Email Input -->
        <div class="space-y-2">
          <label for="email" class="block text-sm font-medium" style="color: var(--gray-700);">Email Address</label>
          <input 
            type="email" 
            id="email"
            v-model="form.email"
            class="w-full px-4 py-3 border-2 border-gray-200 rounded-lg focus:border-primary-500 focus:outline-none transition-colors duration-200"
            style="background-color: var(--white);"
            placeholder="Ingrese su email"
            required
          />
        </div>

        <!-- Password Input -->
        <div class="space-y-2">
          <label for="password" class="block text-sm font-medium" style="color: var(--gray-700);">Password</label>
          <div class="relative">
            <input 
              :type="showPassword ? 'text' : 'password'"
              id="password"
              v-model="form.password"
              class="w-full px-4 py-3 border-2 border-gray-200 rounded-lg focus:border-primary-500 focus:outline-none transition-colors duration-200"
              style="background-color: var(--white);"
              placeholder="••••••••"
              required
            />
            <button 
              type="button"
              @click="showPassword = !showPassword"
              class="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 hover:text-gray-600"
            >
              <svg v-if="showPassword" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
              </svg>
              <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L3 3m6.878 6.878L21 21"></path>
              </svg>
            </button>
          </div>
        </div>

        <!-- Remember Me -->
        <div class="flex items-center justify-between">
          <label class="flex items-center">
            <input type="checkbox" v-model="form.remember" class="rounded border-gray-300 text-primary-600 shadow-sm focus:border-primary-300 focus:ring focus:ring-primary-200 focus:ring-opacity-50">
            <span class="ml-2 text-sm" style="color: var(--gray-600);">Recordarme</span>
          </label>
          <a href="#" class="text-sm hover:underline" style="color: var(--primary-600);">¿Olvidaste tu contraseña?</a>
        </div>

        <!-- Login Button -->
        <button 
          type="submit"
          class="w-full py-3 px-4 rounded-lg font-semibold text-white shadow-lg hover:shadow-xl transform hover:-translate-y-0.5 transition-all duration-200"
          style="background-color: var(--primary-600); hover:background-color: var(--primary-700);"
        >
          Iniciar Sesión
        </button>

        <!-- Footer Links -->
        <div class="text-center space-y-2">
          <p class="text-sm" style="color: var(--gray-600);">
            ¿No tienes cuenta? 
            <a href="#" class="font-medium hover:underline" style="color: var(--primary-600);">Registrarse</a>
          </p>
          <p class="text-xs opacity-60" style="color: var(--gray-500);">
            © 2024 Health Kiu. Todos los derechos reservados.
          </p>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const form = ref({
  email: '',
  password: '',
  remember: false
})

const showPassword = ref(false)

const handleLogin = () => {
  console.log('Login attempt:', form.value)
  // Aquí iría la lógica de autenticación
}
</script>

<style scoped>
/* Custom focus styles */
input:focus {
  border-color: var(--primary-500) !important;
  box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.1);
}

/* Custom checkbox styles */
input[type="checkbox"]:checked {
  background-color: var(--primary-600);
  border-color: var(--primary-600);
}

/* Animation for background circles */
@keyframes float {
  0%, 100% { transform: translateY(0px) rotate(0deg); }
  50% { transform: translateY(-10px) rotate(180deg); }
}

.absolute div {
  animation: float 6s ease-in-out infinite;
}

.absolute div:nth-child(2n) {
  animation-delay: -2s;
}

.absolute div:nth-child(3n) {
  animation-delay: -4s;
}
</style>