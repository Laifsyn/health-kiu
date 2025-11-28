<template>
  <div
    class="card cursor-pointer hover:shadow-md"
    :class="colorClass"
    @click="handleClick"
  >
    <div class="flex flex-col items-center text-center space-y-2">
      <div class="w-12 h-12 flex items-center justify-center" :class="iconColorClass">
        <component :is="icon" class="w-8 h-8" />
      </div>
      <div>
        <p v-if="chineseTitle" class="text-xs text-gray-700 font-medium">{{ chineseTitle }}</p>
        <p class="text-xs text-gray-600">{{ title }}</p>
        <p v-if="subtitle" class="text-xs text-gray-500 mt-0.5">{{ subtitle }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  title: string
  chineseTitle?: string
  subtitle?: string
  icon?: any
  color?: 'pink' | 'blue' | 'orange' | 'green' | 'white'
  to?: string
}

const props = withDefaults(defineProps<Props>(), {
  color: 'white'
})

const emit = defineEmits<{
  click: []
}>()

const colorClass = computed(() => {
  switch (props.color) {
    case 'pink': return 'card-pink'
    case 'blue': return 'card-blue'
    case 'orange': return 'card-orange'
    case 'green': return 'card-green'
    default: return ''
  }
})

const iconColorClass = computed(() => {
  switch (props.color) {
    case 'pink': return 'text-pink-600'
    case 'blue': return 'text-blue-600'
    case 'orange': return 'text-orange-600'
    case 'green': return 'text-green-600'
    default: return 'text-gray-600'
  }
})

const handleClick = () => {
  emit('click')
  if (props.to) {
    navigateTo(props.to)
  }
}
</script>
