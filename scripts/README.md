# Scripts SQL - Health-Kiu CSS

Scripts para gestionar datos de ejemplo simulando la Caja del Seguro Social de Panamá.

## Archivos Disponibles

### 1. `seed_css_panama.sql`
Script principal para poblar la base de datos con datos de ejemplo realistas de la CSS.

**Incluye:**
- 10 doctores adicionales (5 en Panamá, 5 en Chiriquí)
- 15 pacientes de diferentes provincias panameñas
- 12 asegurados CSS con números de afiliación
- 26 habitaciones en diferentes hospitales CSS
- 16 citas médicas (confirmadas, pendientes, canceladas, completadas)

**Uso:**
```bash
# Opción 1: Con psql
psql -h localhost -U orlandomontenegro -d hk-db -f scripts/seed_css_panama.sql

# Opción 2: Desde pgAdmin
# 1. Conectarte a la base de datos hk-db
# 2. Tools → Query Tool
# 3. File → Open → Seleccionar seed_css_panama.sql
# 4. Execute (F5)
```

### 2. `check_data.sql`
Script para verificar rápidamente qué datos existen en la base de datos.

**Muestra:**
- Conteo de registros por tabla
- Distribución de doctores por provincia
- Distribución de pacientes por provincia
- Especialidades más comunes
- Estado de citas
- Tipos de asegurados
- Habitaciones por piso

**Uso:**
```bash
psql -h localhost -U orlandomontenegro -d hk-db -f scripts/check_data.sql
```

### 3. `consultas_utiles.sql`
Colección de 10 consultas útiles para explorar y analizar los datos.

**Incluye:**
1. Doctores con sus especialidades
2. Citas próximas (siguientes 7 días)
3. Pacientes asegurados con datos CSS
4. Doctores más solicitados
5. Especialidades disponibles
6. Disponibilidad de habitaciones
7. Reporte de citas por fecha y estado
8. Pacientes sin seguro CSS
9. Doctores de Chiriquí
10. Resumen estadístico general

**Uso:**
```bash
psql -h localhost -U orlandomontenegro -d hk-db -f scripts/consultas_utiles.sql
```

## Datos de Ejemplo

### Provincias Representadas
- **Panamá (8-XXX-XXXX)**: Ciudad de Panamá - Complejo Hospitalario Metropolitano
- **Chiriquí (4-XXX-XXXX)**: David - Hospital José Domingo de Obaldía
- **Colón (3-XXX-XXXX)**: Zona Libre de Colón
- **Veraguas (7-XXX-XXXX)**: Santiago
- **Herrera (6-XXX-XXXX)**: Chitré

### Hospitales Simulados

#### Complejo Hospitalario Metropolitano (Panamá)
- Habitaciones 101-404
- Servicios: Medicina General, UCI, Cardiología, Cirugía, Maternidad

#### Hospital José Domingo de Obaldía (David, Chiriquí)
- Habitaciones 501-702
- Servicios: Medicina Interna, UCI, Pediatría, Cirugía

#### Hospital Regional de Chiriquí
- Habitaciones 801-902
- Servicios: Emergencias, Traumatología

### Especialidades Médicas
39 especialidades médicas disponibles, incluyendo:
- Medicina General
- Medicina Familiar
- Cardiología
- Pediatría
- Ginecología y Obstetricia
- Traumatología y Ortopedia
- Y muchas más...

### Tipos de Asegurados CSS
- **Asegurados Directos**: Trabajadores activos (sector público, privado, independiente)
- **Beneficiarios**: Familiares de asegurados (cónyuge, hijos)
- **Pensionados**: Jubilados de la CSS

### Credenciales de Acceso
**Contraseña para todos los usuarios:** `password123`

Todos los usuarios (doctores y pacientes) usan la misma contraseña hasheada con bcrypt.

## Formato de Números CSS

Los números de afiliación CSS siguen el formato:
```
CSS-[CEDULA]-001
Ejemplo: CSS-8-111-2222-001
```

## Ejecutar Todo en Secuencia

Para poblar y verificar la base de datos completa:

```bash
# 1. Poblar con datos
psql -h localhost -U orlandomontenegro -d hk-db -f scripts/seed_css_panama.sql

# 2. Verificar datos
psql -h localhost -U orlandomontenegro -d hk-db -f scripts/check_data.sql

# 3. Ejecutar consultas de ejemplo
psql -h localhost -U orlandomontenegro -d hk-db -f scripts/consultas_utiles.sql
```

## Notas Importantes

- Todos los scripts usan `ON CONFLICT DO NOTHING` para evitar duplicados
- Los scripts pueden ejecutarse múltiples veces sin problemas
- Las cédulas usan el formato real panameño: `PROVINCIA-LIBRO-TOMO`
- Los UUIDs están fijos para facilitar las pruebas y debugging
- Las fechas de citas incluyen pasadas, presentes y futuras

## Resetear la Base de Datos

Si necesitas empezar de cero:

```bash
# Opción 1: Usar sea-orm-cli
sea-orm-cli migrate fresh -u postgresql://orlandomontenegro@localhost:5432/hk-db --verbose

# Opción 2: Desde el código Rust
cd hk-api && cargo run
# Las migraciones se ejecutan automáticamente al iniciar
```

## Consultas Rápidas desde la Terminal

```bash
# Contar doctores
psql -h localhost -U orlandomontenegro -d hk-db -c "SELECT COUNT(*) FROM doctor;"

# Ver todas las especialidades
psql -h localhost -U orlandomontenegro -d hk-db -c "SELECT * FROM especialidad;"

# Ver citas de hoy
psql -h localhost -U orlandomontenegro -d hk-db -c "SELECT * FROM cita WHERE DATE(fecha) = CURRENT_DATE;"
```

## Troubleshooting

### Error: "relation does not exist"
Las tablas no se han creado. Ejecuta las migraciones primero:
```bash
cd hk-api && cargo run
```

### Error: "duplicate key value"
Algunos datos ya existen. Los scripts usan `ON CONFLICT DO NOTHING` así que esto es normal y se puede ignorar.

### Error: "connection refused"
PostgreSQL no está corriendo o la configuración en `.env.development` es incorrecta.
