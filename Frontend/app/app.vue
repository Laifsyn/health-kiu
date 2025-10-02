<template>
  <NuxtLayout>
    <NuxtLoadingIndicator />
    <NuxtRouteAnnouncer />
    <nav class="bg-gray-100 p-4">
      <div class="flex gap-4">
        <NuxtLink to="/" class="hover:underline">Home</NuxtLink>
        <NuxtLink to="/hola/1" class="hover:underline">Hola 1</NuxtLink>
        <NuxtLink to="/hola/2" class="hover:underline">Hola 2</NuxtLink>
        <NuxtLink to="/hola/3" class="hover:underline">Hola 3</NuxtLink>
      </div>
    </nav>
    <NuxtPage />

    <footer class="bg-gray-100 p-4 mt-8">
      <p>&copy; 2024 My App</p>
    </footer>
  </NuxtLayout>
</template>
