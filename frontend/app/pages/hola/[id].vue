<template>
    <div>
        <h1>Hola Mundo {{ route.params.id }}</h1>
        <NuxtLink to="/about" class="hover:underline">Go to about page</NuxtLink>
    </div>
</template>
<script setup lang="ts">
import { useRoute } from "vue-router";
const route = useRoute()

// When accessing /posts/1, route.params.id will be 1
console.log("Current ID:", route.params.id)
</script>