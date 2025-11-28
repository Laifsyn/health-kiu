<template>
  <nav class="bottom-nav">
    <div class="nav-container">
      <NuxtLink
        to="/"
        class="nav-item"
        :class="{ active: $route.path === '/' }"
      >
        <svg class="nav-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
        </svg>
        <span class="nav-label">Inicio</span>
      </NuxtLink>

      <NuxtLink
        to="/news"
        class="nav-item"
        :class="{ active: $route.path === '/news' }"
      >
        <svg class="nav-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5.882V19.24a1.76 1.76 0 01-3.417.592l-2.147-6.15M18 13a3 3 0 100-6M5.436 13.683A4.001 4.001 0 017 6h1.832c4.1 0 7.625-1.234 9.168-3v14c-1.543-1.766-5.067-3-9.168-3H7a3.988 3.988 0 01-1.564-.317z" />
        </svg>
        <span class="nav-label">Noticias</span>
      </NuxtLink>

      <NuxtLink
        to="/services"
        class="nav-item"
        :class="{ active: $route.path === '/services' }"
      >
        <svg class="nav-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
        </svg>
        <span class="nav-label">Servicios</span>
      </NuxtLink>

      <NuxtLink
        to="/profile"
        class="nav-item"
        :class="{ active: $route.path === '/profile' }"
      >
        <svg class="nav-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
        </svg>
        <span class="nav-label">Perfil</span>
      </NuxtLink>
    </div>
  </nav>
</template>

<script setup lang="ts">
</script>

<style scoped>
.bottom-nav {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  background: white;
  border-top: 1px solid #E5E7EB;
  box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.08);
  z-index: 50;
}

.nav-container {
  display: flex;
  align-items: center;
  justify-content: space-around;
  height: 64px;
  max-width: 100%;
  margin: 0 auto;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 4px;
  flex: 1;
  color: #6B7280;
  text-decoration: none;
  transition: all 0.2s;
  position: relative;
  padding: 8px;
}

.nav-item.active {
  color: #8B4A5A;
}

.nav-item.active::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 32px;
  height: 3px;
  background: #8B4A5A;
  border-radius: 2px 2px 0 0;
}

.nav-icon {
  width: 24px;
  height: 24px;
}

.nav-label {
  font-size: 11px;
  font-weight: 500;
  text-align: center;
}
</style>
