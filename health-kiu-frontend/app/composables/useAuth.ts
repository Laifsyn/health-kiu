export interface User {
  id: string
  role: 'patient' | 'doctor'
  name: string
  cedula: string
}

export interface LoginCredentials {
  cedula: string
  password: string
  role: 'patient' | 'doctor'
}

export interface RegisterData {
  cedula?: string
  passport?: string
  name: string
  password: string
  role: 'patient' | 'doctor'
}

export const useAuth = () => {
  const user = useState<User | null>('user', () => null)
  const isAuthenticated = computed(() => user.value !== null)
  const router = useRouter()

  // Get API base URL from runtime config or environment
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBase || 'http://localhost:3000/api'

  const login = async (credentials: LoginCredentials) => {
    try {
      // Create Basic Auth header
      const basicAuth = btoa(`${credentials.cedula}:${credentials.password}`)

      const response = await $fetch<{ type: string; id: string }>(`${apiBase}/login/${credentials.role}`, {
        method: 'POST',
        headers: {
          'Authorization': `Basic ${basicAuth}`,
          'Content-Type': 'application/json'
        }
      })

      // Save user data - extract the id field from the response object
      user.value = {
        id: response.id,
        role: credentials.role,
        name: '', // Will be filled from profile endpoint later
        cedula: credentials.cedula
      }

      // Save to localStorage
      if (process.client) {
        localStorage.setItem('user', JSON.stringify(user.value))
        localStorage.setItem('credentials', basicAuth)
      }

      return true
    } catch (error: any) {
      console.error('Login error:', error)
      throw new Error(error?.data?.message || 'Error al iniciar sesión')
    }
  }

  const register = async (data: RegisterData) => {
    try {
      const params = new URLSearchParams()
      if (data.cedula) params.append('cedula', data.cedula)
      if (data.passport) params.append('passport', data.passport)
      params.append('name', data.name)
      params.append('password', data.password)

      const response = await $fetch(`${apiBase}/register/${data.role}?${params.toString()}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        }
      })

      // Auto-login after registration
      if (data.cedula) {
        await login({
          cedula: data.cedula,
          password: data.password,
          role: data.role
        })
      }

      return response
    } catch (error: any) {
      console.error('Register error:', error)
      throw new Error(error?.data?.message || 'Error al registrarse')
    }
  }

  const logout = () => {
    user.value = null
    if (process.client) {
      localStorage.removeItem('user')
      localStorage.removeItem('credentials')
    }
    router.push('/login')
  }

  const loadUserFromStorage = () => {
    if (process.client) {
      const savedUser = localStorage.getItem('user')
      if (savedUser) {
        try {
          const parsed = JSON.parse(savedUser)
          // Handle old format where id was an object { type, id }
          if (parsed.id && typeof parsed.id === 'object' && parsed.id.id) {
            parsed.id = parsed.id.id
          }
          user.value = parsed
        } catch (e) {
          console.error('Error parsing saved user:', e)
        }
      }
    }
  }

  return {
    user,
    isAuthenticated,
    login,
    register,
    logout,
    loadUserFromStorage
  }
}
