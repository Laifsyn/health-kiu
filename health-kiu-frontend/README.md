# Health-KIU Frontend

Frontend para el Sistema de Agendamiento de Citas Hospitalarias Health-KIU, desarrollado con Nuxt 3 y TailwindCSS.

## Tecnologías

- **Nuxt 3** - Framework Vue.js
- **TailwindCSS** - Framework CSS utility-first
- **TypeScript** - Tipado estático

## Estructura

- `app/pages/` - Páginas de la aplicación
- `components/` - Componentes reutilizables
- `composables/` - Composables (useApi)
- `layouts/` - Layouts de la aplicación
- `assets/css/` - Estilos personalizados

## Requisitos

- Node.js >= 18.x
- Backend Health-KIU en `https://127.0.0.1:8081`

## Setup

Instalar dependencias:

```bash
npm install
```

## Development Server

Iniciar servidor de desarrollo en `http://localhost:3000`:

```bash
npm run dev
```

## Production

Build para producción:

```bash
npm run build
```

Preview de producción:

```bash
npm run preview
```

## Variables de Entorno

Crear archivo `.env`:

```env
API_BASE_URL=https://127.0.0.1:8081
```

## Páginas Implementadas

- `/` - Dashboard principal con grid de servicios
- `/appointments` - Lista de especialidades médicas
- `/specialties/:id` - Doctores por especialidad
- `/news` - Noticias del hospital
- `/services` - Servicios digitales
- `/profile` - Perfil de usuario

## Componentes

- `BottomNav` - Navegación inferior (HOME, Noticias, Servicios, Perfil)
- `TabNav` - Navegación por tabs (门诊, 住院, 服务, 综合)
- `ServiceCard` - Tarjeta de servicio reutilizable

## Documentación

Ver [Nuxt documentation](https://nuxt.com/docs) y [deployment documentation](https://nuxt.com/docs/getting-started/deployment) para más información.
