<template>
  <div class="specialties-page">
    <div class="page-header">
      <div class="header-content">
        <NuxtLink to="/" class="back-btn">
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M19 12H5M12 19l-7-7 7-7"/>
          </svg>
          Volver al inicio
        </NuxtLink>
        <div class="header-text">
          <h1>Especialidades Médicas</h1>
          <p>Seleccione una especialidad para ver los médicos disponibles</p>
        </div>
      </div>
    </div>

    <div v-if="loading" class="loading">
      <p>Cargando especialidades...</p>
    </div>

    <div v-else-if="error" class="error-message">
      <p>Error al cargar las especialidades</p>
      <button @click="loadSpecialties" class="retry-btn">Reintentar</button>
    </div>

    <div v-else class="specialties-grid">
      <NuxtLink
        v-for="specialty in specialties"
        :key="specialty.id"
        :to="`/specialties/${specialty.id}`"
        class="specialty-card"
      >
        <div class="specialty-icon">
          <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2v20M2 12h20"></path>
          </svg>
        </div>
        <h3>{{ specialty.name }}</h3>
        <span class="view-doctors">Ver médicos →</span>
      </NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
const { fetchSpecialties } = useApi()

const specialties = ref<any[]>([])
const loading = ref(true)
const error = ref(false)

const loadSpecialties = async () => {
  try {
    loading.value = true
    error.value = false
    const response = await fetchSpecialties(0, 100)
    specialties.value = response.items
  } catch (e) {
    console.error('Error loading specialties:', e)
    error.value = true
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadSpecialties()
})
</script>

<style scoped>
.specialties-page {
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

.loading, .error-message {
  text-align: center;
  padding: 60px 20px;
  background: white;
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
}

.loading p, .error-message p {
  font-size: 16px;
  color: #666;
  margin: 0 0 20px;
}

.retry-btn {
  padding: 12px 24px;
  background: linear-gradient(135deg, #0066b2 0%, #004d8c 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.retry-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0, 102, 178, 0.3);
}

.specialties-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.specialty-card {
  background: white;
  padding: 30px 24px;
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
  text-decoration: none;
  color: inherit;
  transition: all 0.3s;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  cursor: pointer;
}

.specialty-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
}

.specialty-icon {
  width: 80px;
  height: 80px;
  background: linear-gradient(135deg, #0066b2 0%, #004d8c 100%);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 20px;
  color: white;
}

.specialty-card h3 {
  font-size: 20px;
  font-weight: 600;
  color: #1a1a1a;
  margin: 0 0 12px;
}

.view-doctors {
  font-size: 14px;
  color: #0066b2;
  font-weight: 500;
  transition: all 0.2s;
}

.specialty-card:hover .view-doctors {
  transform: translateX(4px);
}

@media (max-width: 768px) {
  .specialties-grid {
    grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
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
