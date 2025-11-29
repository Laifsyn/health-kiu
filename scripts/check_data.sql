-- ========================================
-- Script para verificar datos en la base de datos
-- ========================================

\echo '======================================'
\echo 'VERIFICACIÓN DE DATOS - HEALTH-KIU CSS'
\echo '======================================'
\echo ''

-- Conteo general de registros
\echo '=== CONTEO GENERAL DE REGISTROS ==='
SELECT
    'Usuarios' as tabla,
    COUNT(*) as total,
    CASE WHEN COUNT(*) > 0 THEN '✓' ELSE '✗' END as tiene_datos
FROM "user"
UNION ALL
SELECT 'Doctores', COUNT(*), CASE WHEN COUNT(*) > 0 THEN '✓' ELSE '✗' END FROM doctor
UNION ALL
SELECT 'Pacientes', COUNT(*), CASE WHEN COUNT(*) > 0 THEN '✓' ELSE '✗' END FROM patient
UNION ALL
SELECT 'Especialidades', COUNT(*), CASE WHEN COUNT(*) > 0 THEN '✓' ELSE '✗' END FROM especialidad
UNION ALL
SELECT 'Doctor-Especialidad', COUNT(*), CASE WHEN COUNT(*) > 0 THEN '✓' ELSE '✗' END FROM doctor_especialidad
UNION ALL
SELECT 'Habitaciones', COUNT(*), CASE WHEN COUNT(*) > 0 THEN '✓' ELSE '✗' END FROM habitacion
UNION ALL
SELECT 'Asegurados', COUNT(*), CASE WHEN COUNT(*) > 0 THEN '✓' ELSE '✗' END FROM asegurado
UNION ALL
SELECT 'Citas', COUNT(*), CASE WHEN COUNT(*) > 0 THEN '✓' ELSE '✗' END FROM cita
ORDER BY tabla;

\echo ''
\echo '=== DOCTORES POR PROVINCIA (basado en cédula) ==='
SELECT
    CASE
        WHEN u.cedula LIKE '8-%' THEN 'Panamá'
        WHEN u.cedula LIKE '4-%' THEN 'Chiriquí'
        WHEN u.cedula LIKE '3-%' THEN 'Colón'
        ELSE 'Otra provincia'
    END as provincia,
    COUNT(*) as total_doctores
FROM doctor d
JOIN "user" u ON d.id = u.id
GROUP BY provincia
ORDER BY total_doctores DESC;

\echo ''
\echo '=== PACIENTES POR PROVINCIA (basado en cédula) ==='
SELECT
    CASE
        WHEN u.cedula LIKE '8-%' THEN 'Panamá'
        WHEN u.cedula LIKE '4-%' THEN 'Chiriquí'
        WHEN u.cedula LIKE '3-%' THEN 'Colón'
        WHEN u.cedula LIKE '7-%' THEN 'Veraguas'
        WHEN u.cedula LIKE '6-%' THEN 'Herrera'
        ELSE 'Otra provincia'
    END as provincia,
    COUNT(*) as total_pacientes
FROM patient p
JOIN "user" u ON p.id = u.id
GROUP BY provincia
ORDER BY total_pacientes DESC;

\echo ''
\echo '=== ESPECIALIDADES MÁS COMUNES ENTRE DOCTORES ==='
SELECT
    e.nombre as especialidad,
    COUNT(de.doctor_id) as num_doctores
FROM especialidad e
LEFT JOIN doctor_especialidad de ON e.id = de.especialidad_id
WHERE de.activo = true
GROUP BY e.id, e.nombre
HAVING COUNT(de.doctor_id) > 0
ORDER BY num_doctores DESC, e.nombre
LIMIT 10;

\echo ''
\echo '=== ESTADO DE CITAS ==='
SELECT
    estado,
    COUNT(*) as cantidad,
    ROUND(COUNT(*) * 100.0 / SUM(COUNT(*)) OVER (), 2) as porcentaje
FROM cita
GROUP BY estado
ORDER BY cantidad DESC;

\echo ''
\echo '=== ASEGURADOS CSS POR TIPO ==='
SELECT
    CASE
        WHEN description LIKE '%directa%' OR description LIKE '%directo%' THEN 'Asegurado Directo'
        WHEN description LIKE '%Beneficiari%' THEN 'Beneficiario'
        WHEN description LIKE '%Pensionado%' THEN 'Pensionado'
        ELSE 'Otro'
    END as tipo_asegurado,
    COUNT(*) as cantidad
FROM asegurado
GROUP BY tipo_asegurado
ORDER BY cantidad DESC;

\echo ''
\echo '=== HABITACIONES POR PISO ==='
SELECT
    piso,
    COUNT(*) as total_habitaciones,
    STRING_AGG(DISTINCT
        CASE
            WHEN descripcion LIKE '%UCI%' THEN 'UCI'
            WHEN descripcion LIKE '%Emergencia%' THEN 'Emergencias'
            WHEN descripcion LIKE '%Maternidad%' THEN 'Maternidad'
            WHEN descripcion LIKE '%Cirugía%' THEN 'Cirugía'
            WHEN descripcion LIKE '%Cardiología%' THEN 'Cardiología'
            WHEN descripcion LIKE '%Pediatría%' THEN 'Pediatría'
            ELSE 'General'
        END,
        ', '
    ) as servicios
FROM habitacion
GROUP BY piso
ORDER BY piso;

\echo ''
\echo '======================================'
\echo 'VERIFICACIÓN COMPLETADA'
\echo '======================================'
