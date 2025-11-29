// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },

  modules: ['@nuxtjs/tailwindcss'],

  // Configure API base URL for backend
  runtimeConfig: {
    public: {
      apiBase: process.env.API_BASE_URL || '/api-proxy'
    }
  },

  // Nitro server configuration for proxy
  nitro: {
    devProxy: {
      '/api-proxy': {
        target: 'https://127.0.0.1:8081',
        changeOrigin: true,
        prependPath: true,
        rewrite: (path) => path.replace(/^\/api-proxy/, ''),
        secure: false // Disable SSL verification for self-signed certs
      }
    }
  },

  // App configuration
  app: {
    head: {
      title: 'Health-KIU',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no' },
        { name: 'description', content: 'Sistema de Agendamiento de Citas Hospitalarias' }
      ]
    }
  }
})
