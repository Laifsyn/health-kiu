<template>
  <div class="bg-white border-b border-gray-200">
    <div class="flex items-center justify-around">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        @click="activeTab = tab.id"
        class="flex-1 py-4 text-center relative transition-colors"
        :class="{
          'text-red-600 font-semibold': activeTab === tab.id,
          'text-gray-600': activeTab !== tab.id
        }"
      >
        <span class="text-base">{{ tab.label }}</span>
        <div
          v-if="activeTab === tab.id"
          class="absolute bottom-0 left-0 right-0 h-0.5 bg-red-600"
        ></div>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const tabs = [
  { id: 'outpatient', label: '门诊' },
  { id: 'inpatient', label: '住院' },
  { id: 'services', label: '服务' },
  { id: 'comprehensive', label: '综合' }
]

const activeTab = ref('outpatient')
</script>
