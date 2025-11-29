export default defineNuxtRouteMiddleware((to, from) => {
  // Public routes that don't require authentication
  const publicRoutes = ['/login', '/register']

  // Check authentication status from localStorage (client-side only)
  let isAuthenticated = false

  if (process.client) {
    const savedUser = localStorage.getItem('user')
    isAuthenticated = !!savedUser
  }

  // If trying to access a protected route without being authenticated
  if (!publicRoutes.includes(to.path) && !isAuthenticated) {
    return navigateTo('/login')
  }

  // If trying to access login/register while already authenticated
  if (publicRoutes.includes(to.path) && isAuthenticated) {
    return navigateTo('/')
  }
})
