<template>
  <div class="app-container">
    <!-- Hospital Header -->
    <HospitalHeader />

    <!-- Main Content -->
    <div class="main-content">
      <!-- Top 4 Service Cards -->
      <MainServiceCards />

      <!-- Medical Service Cards + Quick Actions -->
      <MedicalServiceCards />

      <!-- Bottom Navigation -->
      <BottomNav />
    </div>
  </div>
</template>

<script setup lang="ts">
import HospitalHeader from '../../components/HospitalHeader.vue'
import MainServiceCards from '../../components/MainServiceCards.vue'
import MedicalServiceCards from '../../components/MedicalServiceCards.vue'
import BottomNav from '../../components/BottomNav.vue'
</script>

<style scoped>
.app-container {
  min-height: 100vh;
  background: #f8f9fa;
  padding-bottom: 80px; /* Space for bottom nav */
}

.main-content {
  position: relative;
}
</style>
