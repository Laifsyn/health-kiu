- [Sistema de Agendamiento de Citas Hospitalarias - Casos de Uso](#sistema-de-agendamiento-de-citas-hospitalarias---casos-de-uso)
  - [Actores del Sistema](#actores-del-sistema)
    - [1. PACIENTE](#1-paciente)
      - [Gestión de Acceso](#gestión-de-acceso)
      - [Consulta de Información](#consulta-de-información)
      - [Gestión de Citas](#gestión-de-citas)
      - [Gestión Personal](#gestión-personal)
    - [2. MÉDICO](#2-médico)
      - [Gestión de Acceso](#gestión-de-acceso-1)
      - [Gestión de Agenda](#gestión-de-agenda)
      - [Gestión de Pacientes](#gestión-de-pacientes)
      - [Gestión de Citas](#gestión-de-citas-1)
    - [3. ADMINISTRADOR](#3-administrador)
      - [Gestión de Usuarios](#gestión-de-usuarios)
      - [Gestión de Citas](#gestión-de-citas-2)
      - [Gestión de Recursos](#gestión-de-recursos)
      - [Administración del Sistema](#administración-del-sistema)
    - [4. SISTEMA (Actor Automatizado)](#4-sistema-actor-automatizado)
      - [Validaciones Automáticas](#validaciones-automáticas)
      - [Notificaciones Automáticas](#notificaciones-automáticas)
      - [Gestión Inteligente de Citas](#gestión-inteligente-de-citas)
      - [Mantenimiento del Sistema](#mantenimiento-del-sistema)
  - [Casos de Uso Transversales](#casos-de-uso-transversales)
    - [Gestión de Notificaciones ***NP***](#gestión-de-notificaciones-np)


# Sistema de Agendamiento de Citas Hospitalarias - Casos de Uso

## Actores del Sistema

### 1. PACIENTE

#### Gestión de Acceso
- [ ] Registrarse en la plataforma
- [ ] Iniciar sesión
- [ ] Recuperar contraseña ***NP***
- [ ] Actualizar perfil personal ***NP***
- [ ] Cerrar sesión ***NP***

#### Consulta de Información
- [ ] Consultar especialidades médicas disponibles
- [ ] Consultar médicos disponibles por especialidad
- [ ] Ver horarios disponibles de médicos
- [ ] Consultar ubicación de centros de atención ***NP***
- [ ] Ver información de contacto del hospital ***NP***

#### Gestión de Citas
- [ ] Agendar nueva cita médica
- [ ] Ver historial de citas programadas
- [ ] Ver citas próximas
- [ ] Cancelar citas programadas
- [ ] Solicitar reprogramación de citas ***NP***
- [ ] Confirmar asistencia a citas ***NP***
- [ ] Recibir recordatorios de citas ***NP***

#### Gestión Personal
- [ ] Ver historial médico básico ***NP***
- [ ] Actualizar información de contacto ***NP***
- [ ] Gestionar preferencias de notificaciones ***NP***

### 2. MÉDICO

#### Gestión de Acceso
- [ ] Iniciar sesión
- [ ] Actualizar perfil profesional ***NP***
- [ ] Cerrar sesión ***NP***

#### Gestión de Agenda
- [ ] Ver agenda diaria/semanal/mensual
- [ ] Consultar detalles de citas programadas
- [ ] Marcar disponibilidad/no disponibilidad
- [ ] Solicitar cambios en horarios ***NP***
- [ ] Bloquear horarios para actividades específicas ***NP***

#### Gestión de Pacientes
- [ ] Ver lista de pacientes asignados
- [ ] Consultar historial médico de pacientes ***NP***
- [ ] Ver información de contacto de pacientes ***NP***
- [ ] Agregar notas post-consulta ***NP***

#### Gestión de Citas
- [ ] Confirmar citas programadas
- [ ] Solicitar reprogramación de citas ***NP***
- [ ] Marcar citas como completadas ***NP***
- [ ] Registrar inasistencias de pacientes ***NP***

### 3. ADMINISTRADOR

#### Gestión de Usuarios
- [ ] Ver lista de pacientes registrados
- [ ] Crear/modificar/eliminar pacientes
- [ ] Ver lista de médicos registrados
- [ ] Crear/modificar/eliminar médicos
- [ ] Gestionar roles y permisos ***NP***

#### Gestión de Citas
- [ ] Ver todas las citas del sistema
- [ ] Crear/modificar/cancelar cualquier cita
- [ ] Reasignar citas entre médicos ***NP***
- [ ] Generar reportes de citas ***NP***

#### Gestión de Recursos
- [ ] Administrar especialidades médicas
- [ ] Gestionar centros de atención ***NP***
- [ ] Configurar horarios de atención ***NP***
- [ ] Administrar salas y consultorios ***NP***

#### Administración del Sistema
- [ ] Configurar parámetros generales ***NP***
- [ ] Generar reportes estadísticos ***NP***
- [ ] Gestionar notificaciones del sistema ***NP***
- [ ] Realizar respaldos de información ***NP***
- [ ] Monitorear actividad del sistema ***NP***

### 4. SISTEMA (Actor Automatizado)

#### Validaciones Automáticas
- [ ] Validar disponibilidad de médicos al agendar
- [ ] Verificar conflictos de horarios
- [ ] Validar capacidad de centros de atención ***NP***
- [ ] Confirmar datos de contacto ***NP***

#### Notificaciones Automáticas
- [ ] Enviar recordatorios de citas por email/SMS
- [ ] Notificar cambios en citas ***NP***
- [ ] Alertar sobre citas canceladas
- [ ] Enviar confirmaciones de agendamiento ***NP***

#### Gestión Inteligente de Citas
- [ ] Aprobar/rechazar solicitudes de reprogramación automáticamente
- [ ] Optimizar asignación de citas según disponibilidad
- [ ] Gestionar lista de espera automáticamente
- [ ] Reagendar citas canceladas cuando sea posible ***NP***

#### Mantenimiento del Sistema
- [ ] Generar reportes automáticos ***NP***
- [ ] Limpiar datos obsoletos ***NP***
- [ ] Actualizar estados de citas vencidas ***NP***
- [ ] Sincronizar información entre módulos ***NP***

## Casos de Uso Transversales

### Gestión de Notificaciones ***NP***
- [ ] Configurar preferencias de notificación por usuario ***NP***
- [ ] Enviar notificaciones push/email/SMS ***NP***
- [ ] Gestionar plantillas de mensajes ***NP***

---
*NP = No Prioritario*