-- ========================================
-- Script de Datos de Ejemplo para Health-Kiu
-- Simulando la Caja del Seguro Social de Panamá
-- ========================================

-- 1. VERIFICAR DATOS EXISTENTES
-- ========================================
\echo '=== VERIFICANDO DATOS EXISTENTES ==='
SELECT 'Usuarios' as tabla, COUNT(*) as total FROM "user"
UNION ALL
SELECT 'Doctores', COUNT(*) FROM doctor
UNION ALL
SELECT 'Pacientes', COUNT(*) FROM patient
UNION ALL
SELECT 'Especialidades', COUNT(*) FROM especialidad
UNION ALL
SELECT 'Doctor-Especialidad', COUNT(*) FROM doctor_especialidad
UNION ALL
SELECT 'Habitaciones', COUNT(*) FROM habitacion
UNION ALL
SELECT 'Asegurados', COUNT(*) FROM asegurado
UNION ALL
SELECT 'Citas', COUNT(*) FROM cita
ORDER BY tabla;

\echo ''
\echo '=== INSERTANDO DATOS DE EJEMPLO ==='

-- 2. INSERTAR MÁS DOCTORES (Panamá y Chiriquí)
-- ========================================

-- Doctores adicionales con cédulas panameñas
-- Formato de cédula: provincia-libro-tomo
-- Provincia 8 = Panamá, 4 = Chiriquí, 3 = Colón, etc.

INSERT INTO "user" (id, cedula, passport) VALUES
-- Doctores de Panamá
('11111111-1111-1111-1111-111111111111', '8-789-1234', NULL),
('22222222-2222-2222-2222-222222222222', '8-890-2345', NULL),
('33333333-3333-3333-3333-333333333333', '8-901-3456', NULL),
('44444444-4444-4444-4444-444444444444', '8-123-7890', NULL),
('55555555-5555-5555-5555-555555555555', '8-234-8901', NULL),
-- Doctores de Chiriquí
('66666666-6666-6666-6666-666666666666', '4-567-1234', NULL),
('77777777-7777-7777-7777-777777777777', '4-678-2345', NULL),
('88888888-8888-8888-8888-888888888888', '4-789-3456', NULL),
('99999999-9999-9999-9999-999999999999', '4-890-4567', NULL),
('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', '4-901-5678', NULL)
ON CONFLICT (cedula) DO NOTHING;

INSERT INTO doctor (id, name, password_hash) VALUES
-- Doctores de Panamá (Complejo Hospitalario Metropolitano)
('11111111-1111-1111-1111-111111111111', 'Dr. Roberto Chen', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('22222222-2222-2222-2222-222222222222', 'Dra. Patricia Santos', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('33333333-3333-3333-3333-333333333333', 'Dr. José Vargas', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('44444444-4444-4444-4444-444444444444', 'Dra. Luz Marina Castillo', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('55555555-5555-5555-5555-555555555555', 'Dr. Eduardo Herrera', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
-- Doctores de Chiriquí (Hospital José Domingo de Obaldía)
('66666666-6666-6666-6666-666666666666', 'Dr. Manuel Arosemena', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('77777777-7777-7777-7777-777777777777', 'Dra. Yamileth Cedeño', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('88888888-8888-8888-8888-888888888888', 'Dr. Ricardo Pimentel', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('99999999-9999-9999-9999-999999999999', 'Dra. Xiomara González', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 'Dr. Osvaldo Moreno', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O')
ON CONFLICT (id) DO NOTHING;

-- Asignar especialidades a los nuevos doctores
INSERT INTO doctor_especialidad (doctor_id, especialidad_id, certification_date, activo) VALUES
-- Dr. Roberto Chen - Traumatología
('11111111-1111-1111-1111-111111111111', 16, '2018-03-15', true),
-- Dra. Patricia Santos - Gastroenterología
('22222222-2222-2222-2222-222222222222', 22, '2019-06-20', true),
-- Dr. José Vargas - Neumología
('33333333-3333-3333-3333-333333333333', 23, '2017-09-10', true),
-- Dra. Luz Marina Castillo - Endocrinología
('44444444-4444-4444-4444-444444444444', 24, '2020-01-25', true),
-- Dr. Eduardo Herrera - Urología
('55555555-5555-5555-5555-555555555555', 21, '2016-11-30', true),
-- Dr. Manuel Arosemena - Medicina General y Medicina Familiar
('66666666-6666-6666-6666-666666666666', 1, '2015-05-12', true),
('66666666-6666-6666-6666-666666666666', 2, '2015-05-12', true),
-- Dra. Yamileth Cedeño - Pediatría
('77777777-7777-7777-7777-777777777777', 4, '2019-08-18', true),
-- Dr. Ricardo Pimentel - Oftalmología
('88888888-8888-8888-8888-888888888888', 18, '2018-12-05', true),
-- Dra. Xiomara González - Ginecología y Obstetricia
('99999999-9999-9999-9999-999999999999', 6, '2017-04-22', true),
-- Dr. Osvaldo Moreno - Dermatología
('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 20, '2021-02-14', true)
ON CONFLICT (doctor_id, especialidad_id) DO NOTHING;

-- 3. INSERTAR PACIENTES
-- ========================================

INSERT INTO "user" (id, cedula, passport) VALUES
-- Pacientes de Panamá
('bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', '8-111-2222', NULL),
('cccccccc-cccc-cccc-cccc-cccccccccccc', '8-222-3333', NULL),
('dddddddd-dddd-dddd-dddd-dddddddddddd', '8-333-4444', NULL),
('eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee', '8-444-5555', NULL),
('ffffffff-ffff-ffff-ffff-ffffffffffff', '8-555-6666', NULL),
('10101010-1010-1010-1010-101010101010', '8-666-7777', NULL),
('20202020-2020-2020-2020-202020202020', '8-777-8888', NULL),
('30303030-3030-3030-3030-303030303030', '8-888-9999', NULL),
-- Pacientes de Chiriquí
('40404040-4040-4040-4040-404040404040', '4-111-2222', NULL),
('50505050-5050-5050-5050-505050505050', '4-222-3333', NULL),
('60606060-6060-6060-6060-606060606060', '4-333-4444', NULL),
('70707070-7070-7070-7070-707070707070', '4-444-5555', NULL),
-- Pacientes de otras provincias
('80808080-8080-8080-8080-808080808080', '3-567-8901', NULL), -- Colón
('90909090-9090-9090-9090-909090909090', '7-678-9012', NULL), -- Veraguas
('a0a0a0a0-a0a0-a0a0-a0a0-a0a0a0a0a0a0', '6-789-0123', NULL)  -- Herrera
ON CONFLICT (cedula) DO NOTHING;

INSERT INTO patient (id, name, password_hash) VALUES
-- Pacientes de Panamá
('bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', 'María Pérez', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('cccccccc-cccc-cccc-cccc-cccccccccccc', 'Juan Carlos Gómez', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('dddddddd-dddd-dddd-dddd-dddddddddddd', 'Rosa María Villarreal', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee', 'Carlos Alberto Miranda', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('ffffffff-ffff-ffff-ffff-ffffffffffff', 'Ana Lucía Samaniego', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('10101010-1010-1010-1010-101010101010', 'Roberto Jiménez', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('20202020-2020-2020-2020-202020202020', 'Gloria Esther Batista', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('30303030-3030-3030-3030-303030303030', 'Luis Fernando Cordero', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
-- Pacientes de Chiriquí
('40404040-4040-4040-4040-404040404040', 'Mariela Quintero', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('50505050-5050-5050-5050-505050505050', 'Alberto Palacios', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('60606060-6060-6060-6060-606060606060', 'Yaneth Caballero', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('70707070-7070-7070-7070-707070707070', 'Rafael Domínguez', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
-- Pacientes de otras provincias
('80808080-8080-8080-8080-808080808080', 'Carmen Elisa Torres', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('90909090-9090-9090-9090-909090909090', 'Pedro Antonio Valdés', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O'),
('a0a0a0a0-a0a0-a0a0-a0a0-a0a0a0a0a0a0', 'Iris Yolanda Campos', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewgtLyq0pUPLFq6O')
ON CONFLICT (id) DO NOTHING;

-- 4. INSERTAR ASEGURADOS (Pacientes con seguro CSS)
-- ========================================

INSERT INTO asegurado (id, user_id, natural_id, description) VALUES
('a5600001-0000-0000-0000-000000000001', 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', 'CSS-8-111-2222-001', 'Asegurada directa - Empleada sector público'),
('a5600002-0000-0000-0000-000000000002', 'cccccccc-cccc-cccc-cccc-cccccccccccc', 'CSS-8-222-3333-001', 'Asegurado directo - Trabajador independiente'),
('a5600003-0000-0000-0000-000000000003', 'dddddddd-dddd-dddd-dddd-dddddddddddd', 'CSS-8-333-4444-001', 'Asegurada directa - Sector privado'),
('a5600004-0000-0000-0000-000000000004', 'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee', 'CSS-8-444-5555-001', 'Asegurado directo - Pensionado CSS'),
('a5600005-0000-0000-0000-000000000005', 'ffffffff-ffff-ffff-ffff-ffffffffffff', 'CSS-8-555-6666-001', 'Beneficiaria - Cónyuge asegurado'),
('a5600006-0000-0000-0000-000000000006', '10101010-1010-1010-1010-101010101010', 'CSS-8-666-7777-001', 'Asegurado directo - Sector construcción'),
('a5600007-0000-0000-0000-000000000007', '20202020-2020-2020-2020-202020202020', 'CSS-8-777-8888-001', 'Asegurada directa - Sector salud'),
('a5600008-0000-0000-0000-000000000008', '40404040-4040-4040-4040-404040404040', 'CSS-4-111-2222-001', 'Asegurada directa - Sector agrícola'),
('a5600009-0000-0000-0000-000000000009', '50505050-5050-5050-5050-505050505050', 'CSS-4-222-3333-001', 'Asegurado directo - Empleado bananera'),
('a560000a-0000-0000-0000-00000000000a', '60606060-6060-6060-6060-606060606060', 'CSS-4-333-4444-001', 'Beneficiaria - Hija menor de edad'),
('a560000b-0000-0000-0000-00000000000b', '80808080-8080-8080-8080-808080808080', 'CSS-3-567-8901-001', 'Asegurada directa - Zona Libre Colón'),
('a560000c-0000-0000-0000-00000000000c', '90909090-9090-9090-9090-909090909090', 'CSS-7-678-9012-001', 'Asegurado directo - Ganadero')
ON CONFLICT (id) DO NOTHING;

-- 5. INSERTAR HABITACIONES (Diferentes hospitales CSS)
-- ========================================

INSERT INTO habitacion (numero, descripcion, piso) VALUES
-- Complejo Hospitalario Metropolitano (Panamá)
(101, 'Habitación individual - Medicina General', '1'),
(102, 'Habitación doble - Medicina General', '1'),
(103, 'Habitación individual - Medicina General', '1'),
(201, 'Habitación UCI - Cuidados Intensivos', '2'),
(202, 'Habitación UCI - Cuidados Intensivos', '2'),
(203, 'Semi-privada - Cardiología', '2'),
(204, 'Semi-privada - Cardiología', '2'),
(301, 'Sala común - Cirugía General (4 camas)', '3'),
(302, 'Habitación individual - Post-operatorio', '3'),
(303, 'Habitación doble - Post-operatorio', '3'),
(401, 'Maternidad - Sala de partos 1', '4'),
(402, 'Maternidad - Sala de partos 2', '4'),
(403, 'Habitación individual - Maternidad', '4'),
(404, 'Habitación doble - Maternidad', '4'),

-- Hospital José Domingo de Obaldía (David, Chiriquí)
(501, 'Habitación individual - Medicina Interna', '1'),
(502, 'Habitación doble - Medicina Interna', '1'),
(503, 'Sala común - Medicina General (6 camas)', '1'),
(601, 'Habitación UCI - Cuidados Intensivos', '2'),
(602, 'Semi-privada - Pediatría', '2'),
(603, 'Sala común - Pediatría (4 camas)', '2'),
(701, 'Habitación individual - Cirugía', '3'),
(702, 'Habitación doble - Cirugía', '3'),

-- Hospital Regional de Chiriquí (Chiriquí)
(801, 'Emergencias - Observación 1', 'PB'),
(802, 'Emergencias - Observación 2', 'PB'),
(803, 'Emergencias - Observación 3', 'PB'),
(901, 'Habitación individual - Traumatología', '1'),
(902, 'Sala común - Traumatología (3 camas)', '1')
ON CONFLICT (numero) DO NOTHING;

-- 6. INSERTAR CITAS
-- ========================================

INSERT INTO cita (id, doctor_id, paciente_id, asegurado_id, fecha, estado) VALUES
-- Citas en Panamá
('c17a0001-0001-0001-0001-000000000001', '01234567-89ab-cdef-0123-456789abcdef', 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', 'a5600001-0000-0000-0000-000000000001', '2025-12-01 08:00:00', 'confirmada'),
('c17a0002-0002-0002-0002-000000000002', '01234567-89ab-cdef-0123-456789abcde0', 'cccccccc-cccc-cccc-cccc-cccccccccccc', 'a5600002-0000-0000-0000-000000000002', '2025-12-01 09:30:00', 'confirmada'),
('c17a0003-0003-0003-0003-000000000003', '01234567-89ab-cdef-0123-456789abcde1', 'dddddddd-dddd-dddd-dddd-dddddddddddd', 'a5600003-0000-0000-0000-000000000003', '2025-12-01 10:00:00', 'pendiente'),
('c17a0004-0004-0004-0004-000000000004', '11111111-1111-1111-1111-111111111111', 'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee', 'a5600004-0000-0000-0000-000000000004', '2025-12-02 08:30:00', 'confirmada'),
('c17a0005-0005-0005-0005-000000000005', '22222222-2222-2222-2222-222222222222', 'ffffffff-ffff-ffff-ffff-ffffffffffff', 'a5600005-0000-0000-0000-000000000005', '2025-12-02 11:00:00', 'confirmada'),
('c17a0006-0006-0006-0006-000000000006', '33333333-3333-3333-3333-333333333333', '10101010-1010-1010-1010-101010101010', 'a5600006-0000-0000-0000-000000000006', '2025-12-03 09:00:00', 'pendiente'),
('c17a0007-0007-0007-0007-000000000007', '44444444-4444-4444-4444-444444444444', '20202020-2020-2020-2020-202020202020', 'a5600007-0000-0000-0000-000000000007', '2025-12-03 14:00:00', 'confirmada'),
('c17a0008-0008-0008-0008-000000000008', '55555555-5555-5555-5555-555555555555', '30303030-3030-3030-3030-303030303030', NULL, '2025-12-04 10:30:00', 'cancelada'),

-- Citas en Chiriquí
('c17a0009-0009-0009-0009-000000000009', '66666666-6666-6666-6666-666666666666', '40404040-4040-4040-4040-404040404040', 'a5600008-0000-0000-0000-000000000008', '2025-12-02 08:00:00', 'confirmada'),
('c17a000a-000a-000a-000a-00000000000a', '77777777-7777-7777-7777-777777777777', '50505050-5050-5050-5050-505050505050', 'a5600009-0000-0000-0000-000000000009', '2025-12-02 13:30:00', 'confirmada'),
('c17a000b-000b-000b-000b-00000000000b', '88888888-8888-8888-8888-888888888888', '60606060-6060-6060-6060-606060606060', 'a560000a-0000-0000-0000-00000000000a', '2025-12-03 09:30:00', 'pendiente'),
('c17a000c-000c-000c-000c-00000000000c', '99999999-9999-9999-9999-999999999999', '70707070-7070-7070-7070-707070707070', NULL, '2025-12-03 15:00:00', 'confirmada'),
('c17a000d-000d-000d-000d-00000000000d', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', '80808080-8080-8080-8080-808080808080', 'a560000b-0000-0000-0000-00000000000b', '2025-12-04 11:00:00', 'confirmada'),

-- Citas pasadas (completadas)
('c17a000e-000e-000e-000e-00000000000e', '01234567-89ab-cdef-0123-456789abcdef', 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', 'a5600001-0000-0000-0000-000000000001', '2025-11-25 10:00:00', 'completada'),
('c17a000f-000f-000f-000f-00000000000f', '66666666-6666-6666-6666-666666666666', '40404040-4040-4040-4040-404040404040', 'a5600008-0000-0000-0000-000000000008', '2025-11-26 14:30:00', 'completada'),
('c17a0010-0010-0010-0010-000000000010', '77777777-7777-7777-7777-777777777777', '60606060-6060-6060-6060-606060606060', 'a560000a-0000-0000-0000-00000000000a', '2025-11-27 09:00:00', 'completada')
ON CONFLICT (id) DO NOTHING;

-- 7. RESUMEN DE DATOS INSERTADOS
-- ========================================
\echo ''
\echo '=== RESUMEN DE DATOS DESPUÉS DE LA INSERCIÓN ==='
SELECT 'Usuarios' as tabla, COUNT(*) as total FROM "user"
UNION ALL
SELECT 'Doctores', COUNT(*) FROM doctor
UNION ALL
SELECT 'Pacientes', COUNT(*) FROM patient
UNION ALL
SELECT 'Especialidades', COUNT(*) FROM especialidad
UNION ALL
SELECT 'Doctor-Especialidad', COUNT(*) FROM doctor_especialidad
UNION ALL
SELECT 'Habitaciones', COUNT(*) FROM habitacion
UNION ALL
SELECT 'Asegurados', COUNT(*) FROM asegurado
UNION ALL
SELECT 'Citas', COUNT(*) FROM cita
ORDER BY tabla;

\echo ''
\echo '=== DATOS INSERTADOS EXITOSAMENTE ==='
\echo 'Simulación de la Caja del Seguro Social de Panamá'
\echo '- 10 doctores adicionales (Panamá y Chiriquí)'
\echo '- 15 pacientes de diferentes provincias'
\echo '- 12 asegurados CSS con números de afiliación'
\echo '- 26 habitaciones en hospitales CSS'
\echo '- 16 citas médicas (confirmadas, pendientes, canceladas, completadas)'
\echo ''
\echo 'Contraseña para todos los usuarios: password123'
