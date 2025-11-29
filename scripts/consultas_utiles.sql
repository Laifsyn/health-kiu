-- ========================================
-- Consultas Útiles - Sistema CSS Panamá
-- ========================================

\echo '======================================'
\echo 'CONSULTAS ÚTILES - HEALTH-KIU CSS'
\echo '======================================'
\echo ''

-- 1. Listar todos los doctores con sus especialidades
\echo '=== 1. DOCTORES CON SUS ESPECIALIDADES ==='
SELECT
    d.name as doctor,
    u.cedula,
    STRING_AGG(e.nombre, ', ' ORDER BY e.nombre) as especialidades,
    COUNT(de.especialidad_id) as num_especialidades
FROM doctor d
JOIN "user" u ON d.id = u.id
LEFT JOIN doctor_especialidad de ON d.id = de.doctor_id AND de.activo = true
LEFT JOIN especialidad e ON de.especialidad_id = e.id
GROUP BY d.id, d.name, u.cedula
ORDER BY d.name;

\echo ''
\echo '=== 2. CITAS PRÓXIMAS (Siguientes 7 días) ==='
SELECT
    c.fecha,
    d.name as doctor,
    e.nombre as especialidad,
    p.name as paciente,
    pu.cedula as cedula_paciente,
    c.estado,
    CASE WHEN a.id IS NOT NULL THEN 'SÍ' ELSE 'NO' END as tiene_seguro
FROM cita c
JOIN doctor d ON c.doctor_id = d.id
JOIN patient p ON c.paciente_id = p.id
JOIN "user" pu ON p.id = pu.id
LEFT JOIN asegurado a ON c.asegurado_id = a.id
LEFT JOIN doctor_especialidad de ON d.id = de.doctor_id AND de.activo = true
LEFT JOIN especialidad e ON de.especialidad_id = e.id
WHERE c.fecha BETWEEN NOW() AND NOW() + INTERVAL '7 days'
    AND c.estado != 'cancelada'
ORDER BY c.fecha;

\echo ''
\echo '=== 3. PACIENTES ASEGURADOS CON SUS DATOS CSS ==='
SELECT
    p.name as paciente,
    u.cedula,
    a.natural_id as numero_css,
    a.description as tipo_cobertura,
    COUNT(c.id) as total_citas
FROM patient p
JOIN "user" u ON p.id = u.id
JOIN asegurado a ON p.id = a.user_id
LEFT JOIN cita c ON p.id = c.paciente_id
GROUP BY p.id, p.name, u.cedula, a.natural_id, a.description
ORDER BY p.name;

\echo ''
\echo '=== 4. DOCTORES MÁS SOLICITADOS (Por número de citas) ==='
SELECT
    d.name as doctor,
    u.cedula,
    STRING_AGG(DISTINCT e.nombre, ', ') as especialidades,
    COUNT(c.id) as total_citas,
    SUM(CASE WHEN c.estado = 'completada' THEN 1 ELSE 0 END) as citas_completadas,
    SUM(CASE WHEN c.estado = 'confirmada' THEN 1 ELSE 0 END) as citas_confirmadas,
    SUM(CASE WHEN c.estado = 'pendiente' THEN 1 ELSE 0 END) as citas_pendientes
FROM doctor d
JOIN "user" u ON d.id = u.id
LEFT JOIN cita c ON d.id = c.doctor_id
LEFT JOIN doctor_especialidad de ON d.id = de.doctor_id AND de.activo = true
LEFT JOIN especialidad e ON de.especialidad_id = e.id
GROUP BY d.id, d.name, u.cedula
HAVING COUNT(c.id) > 0
ORDER BY total_citas DESC;

\echo ''
\echo '=== 5. ESPECIALIDADES DISPONIBLES EN EL SISTEMA ==='
SELECT
    e.nombre as especialidad,
    COUNT(DISTINCT de.doctor_id) as doctores_disponibles,
    COUNT(c.id) as total_citas_programadas
FROM especialidad e
LEFT JOIN doctor_especialidad de ON e.id = de.especialidad_id AND de.activo = true
LEFT JOIN cita c ON de.doctor_id = c.doctor_id
GROUP BY e.id, e.nombre
HAVING COUNT(DISTINCT de.doctor_id) > 0
ORDER BY doctores_disponibles DESC, e.nombre;

\echo ''
\echo '=== 6. DISPONIBILIDAD DE HABITACIONES ==='
SELECT
    h.numero as habitacion,
    h.piso,
    h.descripcion,
    CASE
        WHEN h.descripcion LIKE '%individual%' THEN '1'
        WHEN h.descripcion LIKE '%doble%' THEN '2'
        WHEN h.descripcion LIKE '%(3 camas)%' THEN '3'
        WHEN h.descripcion LIKE '%(4 camas)%' THEN '4'
        WHEN h.descripcion LIKE '%(6 camas)%' THEN '6'
        ELSE 'Variable'
    END as capacidad,
    CASE
        WHEN h.numero BETWEEN 100 AND 499 THEN 'Complejo Metropolitano - Panamá'
        WHEN h.numero BETWEEN 500 AND 799 THEN 'Hospital J.D. Obaldía - David'
        WHEN h.numero >= 800 THEN 'Hospital Regional - Chiriquí'
        ELSE 'Otro'
    END as hospital
FROM habitacion h
ORDER BY h.numero;

\echo ''
\echo '=== 7. REPORTE DE CITAS POR FECHA Y ESTADO ==='
SELECT
    DATE(c.fecha) as fecha,
    c.estado,
    COUNT(*) as cantidad,
    STRING_AGG(DISTINCT d.name, ', ') as doctores
FROM cita c
JOIN doctor d ON c.doctor_id = d.id
GROUP BY DATE(c.fecha), c.estado
ORDER BY fecha DESC, c.estado;

\echo ''
\echo '=== 8. PACIENTES SIN SEGURO CSS ==='
SELECT
    p.name as paciente,
    u.cedula,
    COUNT(c.id) as citas_registradas
FROM patient p
JOIN "user" u ON p.id = u.id
LEFT JOIN asegurado a ON p.id = a.user_id
LEFT JOIN cita c ON p.id = c.paciente_id
WHERE a.id IS NULL
GROUP BY p.id, p.name, u.cedula
ORDER BY citas_registradas DESC, p.name;

\echo ''
\echo '=== 9. DOCTORES DE CHIRIQUÍ CON SUS ESPECIALIDADES ==='
SELECT
    d.name as doctor,
    u.cedula,
    STRING_AGG(e.nombre, ', ' ORDER BY e.nombre) as especialidades,
    COUNT(c.id) as citas_programadas
FROM doctor d
JOIN "user" u ON d.id = u.id
LEFT JOIN doctor_especialidad de ON d.id = de.doctor_id AND de.activo = true
LEFT JOIN especialidad e ON de.especialidad_id = e.id
LEFT JOIN cita c ON d.id = c.doctor_id
WHERE u.cedula LIKE '4-%'  -- Chiriquí
GROUP BY d.id, d.name, u.cedula
ORDER BY d.name;

\echo ''
\echo '=== 10. RESUMEN ESTADÍSTICO GENERAL ==='
SELECT
    'Total de Usuarios' as metrica,
    COUNT(*)::text as valor
FROM "user"
UNION ALL
SELECT 'Total de Doctores', COUNT(*)::text FROM doctor
UNION ALL
SELECT 'Total de Pacientes', COUNT(*)::text FROM patient
UNION ALL
SELECT 'Pacientes Asegurados CSS', COUNT(*)::text FROM asegurado
UNION ALL
SELECT 'Total de Especialidades', COUNT(*)::text FROM especialidad
UNION ALL
SELECT 'Total de Habitaciones', COUNT(*)::text FROM habitacion
UNION ALL
SELECT 'Total de Citas', COUNT(*)::text FROM cita
UNION ALL
SELECT 'Citas Completadas', COUNT(*)::text FROM cita WHERE estado = 'completada'
UNION ALL
SELECT 'Citas Confirmadas', COUNT(*)::text FROM cita WHERE estado = 'confirmada'
UNION ALL
SELECT 'Citas Pendientes', COUNT(*)::text FROM cita WHERE estado = 'pendiente'
UNION ALL
SELECT 'Citas Canceladas', COUNT(*)::text FROM cita WHERE estado = 'cancelada';

\echo ''
\echo '======================================'
\echo 'CONSULTAS COMPLETADAS'
\echo '======================================'
