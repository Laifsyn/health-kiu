# Sistema de Autenticación Health-KIU

## Características

- ✅ Login y Registro de usuarios (Pacientes y Doctores)
- ✅ Autenticación con Basic Auth
- ✅ Protección de rutas con middleware
- ✅ Persistencia de sesión en localStorage
- ✅ Interfaz moderna y responsive

## Páginas Creadas

### `/login`
Página de inicio de sesión para usuarios existentes.
- Selector de tipo de usuario (Paciente/Doctor)
- Campos: Cédula y Contraseña
- Redirección automática si ya está autenticado
- Enlace a página de registro

### `/register`
Página de registro para nuevos usuarios.
- Selector de tipo de usuario (Paciente/Doctor)
- Campos requeridos: Nombre, Contraseña
- Campos opcionales: Cédula, Pasaporte (al menos uno requerido)
- Validación de contraseñas coincidentes
- Auto-login después del registro exitoso

### `/` (Home)
Página principal protegida por autenticación.
- Muestra perfil del usuario logueado
- Botón de cerrar sesión
- Contenido de la aplicación

## Endpoints del Backend

### Registro
- `POST /register/patient` - Registrar paciente
- `POST /register/doctor` - Registrar doctor

**Query Parameters:**
- `name` (requerido): Nombre completo
- `password` (requerido): Contraseña
- `cedula` (opcional): Cédula
- `passport` (opcional): Pasaporte
- *Al menos uno de cedula o passport es requerido*

### Login
- `POST /login/patient` - Login de paciente
- `POST /login/doctor` - Login de doctor

**Headers:**
- `Authorization: Basic <base64(cedula:password)>`

## Flujo de Autenticación

1. Usuario accede a la aplicación
2. Middleware `auth` verifica si hay sesión activa
3. Si no está autenticado → redirige a `/login`
4. Usuario inicia sesión o se registra
5. Credenciales se guardan en localStorage
6. Usuario puede acceder a las páginas protegidas
7. Botón de logout limpia la sesión

## Composable useAuth

Proporciona las siguientes funciones:

```typescript
const {
  user,              // Usuario actual
  isAuthenticated,   // Boolean de autenticación
  login,             // Función de login
  register,          // Función de registro
  logout,            // Función de logout
  loadUserFromStorage // Cargar usuario de localStorage
} = useAuth()
```

## Configuración

El frontend se conecta al backend mediante un proxy configurado en `nuxt.config.ts`:

```typescript
runtimeConfig: {
  public: {
    apiBase: '/api-proxy'
  }
},
nitro: {
  devProxy: {
    '/api-proxy': {
      target: 'https://127.0.0.1:8081',
      changeOrigin: true,
      secure: false
    }
  }
}
```

## Uso

1. Iniciar el backend en `https://127.0.0.1:8081`
2. Iniciar el frontend:
   ```bash
   npm run dev
   ```
3. Acceder a `http://localhost:3000`
4. Crear una cuenta nueva en `/register`
5. Iniciar sesión en `/login`

## Seguridad

- Las contraseñas se envían mediante Basic Auth (base64)
- El backend usa Argon2 para hashear contraseñas
- Las credenciales se almacenan en localStorage (cliente)
- El middleware protege rutas no autorizadas

## Próximas Mejoras

- [ ] Implementar JWT tokens desde el backend
- [ ] Agregar refresh tokens
- [ ] Mejorar manejo de errores
- [ ] Agregar recuperación de contraseña
- [ ] Agregar perfil de usuario completo
