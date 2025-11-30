use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;
use time::Date;
use uuid::Uuid;

use crate::log_with_context as lwc;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create User table first
        manager
            .create_table(User::create_table())
            .await
            .map_err(lwc("Failed to create user table"))?;

        manager
            .create_index(User::create_cedula_index())
            .await
            .map_err(lwc("Failed to create user.cedula index"))?;

        // Create Doctor table
        manager
            .create_table(Doctor::create_table())
            .await
            .map_err(lwc("Failed to create doctor table"))?;

        manager
            .create_foreign_key(Doctor::table_user_fk())
            .await
            .map_err(lwc("Failed to relate doctor.user_id -> user.id"))?;

        // Create Patient table
        manager
            .create_table(Patient::create_table())
            .await
            .map_err(lwc("Failed to create patient table"))?;

        manager
            .create_foreign_key(Patient::table_user_fk())
            .await
            .map_err(lwc("Failed to relate patient.user_id -> user.id"))?;

        // Create Especialidad table
        manager
            .create_table(Especialidad::create_table())
            .await
            .map_err(lwc("Failed to create especialidad table"))?;

        Especialidad::populate_data(manager).await?;

        // Create DoctorEspecialidad junction table
        manager
            .create_table(DoctorEspecialidad::create_table())
            .await
            .map_err(lwc("Failed to create doctor_especialidad table"))?;

        manager
            .create_foreign_key(DoctorEspecialidad::table_doctor_fk())
            .await
            .map_err(lwc(
                "Failed to relate doctor_especialidad.doctor_id -> doctor.id"
            ))?;

        manager
            .create_foreign_key(DoctorEspecialidad::table_especialidad_fk())
            .await
            .map_err(lwc("Failed to relate \
                          doctor_especialidad.especialidad_id -> \
                          especialidad.id"))?;

        // Create Habitacion table
        manager
            .create_table(Habitacion::create_table())
            .await
            .map_err(lwc("Failed to create habitacion table"))?;

        // Create Asegurado table
        manager
            .create_table(Asegurado::create_table())
            .await
            .map_err(lwc("Failed to create asegurado table"))?;

        manager
            .create_foreign_key(Asegurado::table_user_fk())
            .await
            .map_err(lwc("Failed to relate asegurado.patient_id -> user.id"))?;

        // Create Cita table
        manager
            .create_table(Cita::create_table())
            .await
            .map_err(lwc("Failed to create cita table"))?;

        manager
            .create_foreign_key(Cita::table_doctor_fk())
            .await
            .map_err(lwc("Failed to relate cita.doctor_id -> doctor.id"))?;

        manager
            .create_foreign_key(Cita::table_patient_fk())
            .await
            .map_err(lwc("Failed to relate cita.paciente_id -> patient.id"))?;

        manager.create_foreign_key(Cita::table_asegurado_fk()).await.map_err(
            lwc("Failed to relate cita.asegurado_id -> asegurado.id"),
        )?;

        // Populate sample doctors with specialties
        Doctor::populate_sample_data(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order (foreign keys will be automatically
        // dropped)
        manager
            .drop_table(Cita::drop_table())
            .await
            .map_err(lwc("Failed to drop Cita table"))?;

        manager
            .drop_table(Asegurado::drop_table())
            .await
            .map_err(lwc("Failed to drop Asegurado table"))?;

        manager
            .drop_table(Habitacion::drop_table())
            .await
            .map_err(lwc("Failed to drop Habitacion table"))?;

        manager
            .drop_table(DoctorEspecialidad::drop_table())
            .await
            .map_err(lwc("Failed to drop DoctorEspecialidad table"))?;

        manager
            .drop_table(Especialidad::drop_table())
            .await
            .map_err(lwc("Failed to drop Especialidad table"))?;

        manager
            .drop_table(Patient::drop_table())
            .await
            .map_err(lwc("Failed to drop Patient table"))?;

        manager
            .drop_table(Doctor::drop_table())
            .await
            .map_err(lwc("Failed to drop Doctor table"))?;

        manager // This will also drop related indexes
            .drop_table(User::drop_table())
            .await
            .map_err(lwc("Failed to drop User table"))?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Cedula,
    Passport,
}

impl User {
    /// Name of the unique index on [`User::Cedula`] column
    /// # dead_code: No usage found yet for this constant
    #[expect(dead_code)]
    const INDEX_CEDULA: &'static str = "user_cedula_idx";

    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(User::Table)
            .comment("Base user authentication table")
            .col(uuid(User::Id).primary_key().comment("Test Comment"))
            .col(string_len(User::Cedula, 20).unique_key())
            .col(text_null(User::Passport).unique_key()) // Make unique here since there's no special requirements
            .to_owned()
    }

    pub fn drop_table() -> TableDropStatement {
        Table::drop().table(User::Table).if_exists().to_owned()
    }

    pub fn create_cedula_index() -> IndexCreateStatement {
        Index::create()
            .table(User::Table)
            .col(User::Cedula)
            .unique()
            .include(User::Id)
            .take()
    }
}

#[derive(DeriveIden)]
pub enum Doctor {
    Table,
    Id,
    Name,
    PasswordHash,
}

impl Doctor {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(Doctor::Table)
            .if_not_exists()
            .col(uuid(Doctor::Id).primary_key())
            .col(text(Doctor::Name))
            .col(text(Doctor::PasswordHash))
            .to_owned()
    }

    pub fn table_user_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(Doctor::Table, Doctor::Id)
            .to(User::Table, User::Id)
            .on_delete(ForeignKeyAction::NoAction)
            .on_update(ForeignKeyAction::Restrict)
            .take()
    }

    pub fn drop_table() -> TableDropStatement {
        Table::drop().table(Doctor::Table).if_exists().to_owned()
    }

    async fn populate_sample_data(
        manager: &SchemaManager<'_>,
    ) -> Result<(), DbErr> {
        // Sample doctors data: (user_id, cedula, passport, name, password_hash,
        // specialties)
        let doctors = [
            (
                "01234567-89ab-cdef-0123-456789abcdef",
                "001-1234567-8",
                None,
                "Dr. María González",
                "$argon2id$v=19$m=19456,t=2,p=1$KcQXe/xfSmh2PofgP9/\
                 6DA$ypct83GJKYvIycX1A+XZEdvk3ig55007poqqwl0qeB0", /* 12345678 */
                vec![1, 2], // Medicina General, Medicina Familiar
            ),
            (
                "01234567-89ab-cdef-0123-456789abcde0",
                "001-2345678-9",
                Some("P12345678"),
                "Dr. Carlos Rodríguez",
                "$argon2id$v=19$m=19456,t=2,\
                 p=1$mfJGD5eIEn5q7dAKTD3DTg$GJlv3KP05+G8EftAKw+ITdBO3reFJD7Vja3WFA4Ssyk",
                vec![11, 3], // Cardiología, Medicina Interna
            ),
            (
                "01234567-89ab-cdef-0123-456789abcde1",
                "001-3456789-0",
                None,
                "Dr. Ana Martínez",
                "$argon2id$v=19$m=19456,t=2,\
                 p=1$mwXJO7rw70+vorZqnTYnxg$jiO3dEgWdKj27NBCaSlFYrdCnNHXF07UPhrxKR2gHCs",
                vec![4, 11], // Pediatría, Cardiología
            ),
            (
                "01234567-89ab-cdef-0123-456789abcde2",
                "001-4567890-1",
                None,
                "Dr. Luis Fernández",
                "$argon2id$v=19$m=19456,t=2,p=1$jANKBYUJx1ujGcArJqaWaA$Mz9v/\
                 DSqA1gVyO0Rwr8X3ntLOaasXX6LCcBJZMwCdEc",
                vec![9, 12, 13], // Cirugía General, Neurología, Neurocirugía
            ),
            (
                "01234567-89ab-cdef-0123-456789abcde3",
                "001-5678901-2",
                Some("P87654321"),
                "Dr. Carmen López",
                "$argon2id$v=19$m=19456,t=2,\
                 p=1$5ZYJcaPMG2nK0MvBFvElXQ$OsrmX4h2NEuo+3kQs9j1jhOPCNMkjmXRarXjZCOEtKQ",
                vec![6, 20], // Ginecología y Obstetricia, Dermatología
            ),
        ];

        // Insert users first
        let mut user_insert = Query::insert()
            .into_table(User::Table)
            .columns([User::Id, User::Cedula, User::Passport])
            .to_owned();

        for (user_id, cedula, passport, _, _, _) in doctors.iter() {
            let passport_value = match passport {
                Some(p) => SimpleExpr::Value((*p).into()),
                None => SimpleExpr::Value(Value::String(None)),
            };
            let uuid = Uuid::parse_str(user_id).expect("Invalid UUID");
            user_insert.values_panic([
                SimpleExpr::Value(uuid.into()),
                SimpleExpr::Value((*cedula).into()),
                passport_value,
            ]);
        }

        manager
            .exec_stmt(user_insert)
            .await
            .map_err(lwc("Failed to insert sample users for doctors"))?;

        // Insert doctors
        let mut doctor_insert = Query::insert()
            .into_table(Doctor::Table)
            .columns([Doctor::Id, Doctor::Name, Doctor::PasswordHash])
            .to_owned();

        for (user_id, _, _, name, password_hash, _) in doctors.iter() {
            let uuid = Uuid::parse_str(user_id).expect("Invalid UUID");
            doctor_insert.values_panic([
                SimpleExpr::Value(uuid.into()),
                SimpleExpr::Value((*name).into()),
                SimpleExpr::Value((*password_hash).into()),
            ]);
        }

        manager
            .exec_stmt(doctor_insert)
            .await
            .map_err(lwc("Failed to insert sample doctors"))?;

        // Insert doctor specialties
        let mut specialty_insert = Query::insert()
            .into_table(DoctorEspecialidad::Table)
            .columns([
                DoctorEspecialidad::DoctorId,
                DoctorEspecialidad::EspecialidadId,
                DoctorEspecialidad::CertificationDate,
                DoctorEspecialidad::Activo,
            ])
            .to_owned();

        for (user_id, _, _, _, _, specialties) in doctors.iter() {
            let uuid = Uuid::parse_str(user_id).expect("Invalid UUID");
            let cert_date =
                Date::from_calendar_date(2020, time::Month::January, 15)
                    .expect("Invalid date");
            for specialty_id in specialties.iter() {
                specialty_insert.values_panic([
                    SimpleExpr::Value(uuid.into()),
                    SimpleExpr::Value((*specialty_id).into()),
                    SimpleExpr::Value(cert_date.into()),
                    SimpleExpr::Value(true.into()),
                ]);
            }
        }

        manager
            .exec_stmt(specialty_insert)
            .await
            .map_err(lwc("Failed to insert doctor specialties"))?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Patient {
    Table,
    Id,
    Name,
    PasswordHash,
}

impl Patient {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(Patient::Table)
            .if_not_exists()
            .col(uuid(Patient::Id).primary_key())
            .col(text(Patient::Name))
            .col(text(Patient::PasswordHash))
            .to_owned()
    }

    pub fn table_user_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(Patient::Table, Patient::Id)
            .to(User::Table, User::Id)
            .on_delete(ForeignKeyAction::NoAction)
            .on_update(ForeignKeyAction::Restrict)
            .take()
    }

    pub fn drop_table() -> TableDropStatement {
        Table::drop().table(Patient::Table).if_exists().to_owned()
    }
}

#[derive(DeriveIden)]
pub enum Especialidad {
    Table,
    Id,
    Nombre,
    ImgPath,
}

impl Especialidad {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(Especialidad::Table)
            .if_not_exists()
            .col(pk_auto(Especialidad::Id).small_integer())
            .col(text_uniq(Especialidad::Nombre))
            .col(text_null(Especialidad::ImgPath))
            .to_owned()
    }

    pub fn drop_table() -> TableDropStatement {
        Table::drop().table(Especialidad::Table).if_exists().to_owned()
    }

    async fn populate_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        let specialties = [
            // Core Primary Care
            ("Medicina General", "/images/specialties/general-medicine.jpg"),
            ("Medicina Familiar", "/images/specialties/family-medicine.jpg"),
            ("Medicina Interna", "/images/specialties/internal-medicine.jpg"),
            ("Pediatría", "/images/specialties/pediatrics.jpg"),
            ("Geriatría", "/images/specialties/geriatrics.jpg"),
            // Women's Health
            ("Ginecología y Obstetricia", "/images/specialties/obgyn.jpg"),
            // Emergency & Critical Care
            (
                "Medicina de Emergencias",
                "/images/specialties/emergency-medicine.jpg",
            ),
            ("Medicina Crítica", "/images/specialties/critical-care.jpg"),
            // Major Surgery
            ("Cirugía General", "/images/specialties/general-surgery.jpg"),
            ("Anestesiología", "/images/specialties/anesthesiology.jpg"),
            // Cardiovascular
            ("Cardiología", "/images/specialties/cardiology.jpg"),
            (
                "Cirugía Cardiovascular",
                "/images/specialties/cardiovascular-surgery.jpg",
            ),
            // Neurological
            ("Neurología", "/images/specialties/neurology.jpg"),
            ("Neurocirugía", "/images/specialties/neurosurgery.jpg"),
            // Mental Health
            ("Psiquiatría", "/images/specialties/psychiatry.jpg"),
            // Musculoskeletal
            (
                "Traumatología y Ortopedia",
                "/images/specialties/orthopedics.jpg",
            ),
            ("Reumatología", "/images/specialties/rheumatology.jpg"),
            // Sensory Organs
            ("Oftalmología", "/images/specialties/ophthalmology.jpg"),
            ("Otorrinolaringología", "/images/specialties/otolaryngology.jpg"),
            // Skin
            ("Dermatología", "/images/specialties/dermatology.jpg"),
            // Urogenital
            ("Urología", "/images/specialties/urology.jpg"),
            // Digestive System
            ("Gastroenterología", "/images/specialties/gastroenterology.jpg"),
            // Respiratory
            ("Neumología", "/images/specialties/pulmonology.jpg"),
            // Endocrine
            ("Endocrinología", "/images/specialties/endocrinology.jpg"),
            // Blood & Cancer
            ("Hematología", "/images/specialties/hematology.jpg"),
            ("Oncología", "/images/specialties/oncology.jpg"),
            // Infectious Disease
            ("Infectología", "/images/specialties/infectious-disease.jpg"),
            // Diagnostic
            ("Radiología", "/images/specialties/radiology.jpg"),
            ("Patología", "/images/specialties/pathology.jpg"),
            // Additional Surgical Specialties
            ("Cirugía Plástica", "/images/specialties/plastic-surgery.jpg"),
            ("Cirugía Torácica", "/images/specialties/thoracic-surgery.jpg"),
            ("Cirugía Vascular", "/images/specialties/vascular-surgery.jpg"),
            // Rehabilitation
            (
                "Medicina Física y Rehabilitación",
                "/images/specialties/physical-medicine.jpg",
            ),
            // Specialized Care
            ("Medicina del Dolor", "/images/specialties/pain-management.jpg"),
            (
                "Medicina Preventiva",
                "/images/specialties/preventive-medicine.jpg",
            ),
            (
                "Medicina Ocupacional",
                "/images/specialties/occupational-medicine.jpg",
            ),
            (
                "Alergología e Inmunología",
                "/images/specialties/allergy-immunology.jpg",
            ),
            ("Nefrología", "/images/specialties/nephrology.jpg"),
            ("Medicina Nuclear", "/images/specialties/nuclear-medicine.jpg"),
        ];
        let mut insert = Query::insert()
            .into_table(Especialidad::Table)
            .columns([Especialidad::Nombre, Especialidad::ImgPath])
            .to_owned();

        for (name, img_path) in specialties.iter() {
            insert.values_panic([(*name).into(), (*img_path).into()]);
        }

        manager
            .exec_stmt(insert)
            .await
            .map_err(lwc("Failed to insert specialties"))?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum DoctorEspecialidad {
    Table,
    DoctorId,
    EspecialidadId,
    CertificationDate,
    Activo,
}

impl DoctorEspecialidad {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(uuid_null(Self::DoctorId))
            .col(small_integer_null(Self::EspecialidadId))
            .col(date_null(Self::CertificationDate))
            .col(boolean(Self::Activo).default(true))
            .primary_key(
                Index::create().col(Self::DoctorId).col(Self::EspecialidadId),
            )
            .to_owned()
    }

    pub fn table_doctor_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(Self::Table, Self::DoctorId)
            .to(Doctor::Table, Doctor::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Restrict)
            .take()
    }

    pub fn table_especialidad_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(Self::Table, Self::EspecialidadId)
            .to(Especialidad::Table, Especialidad::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Restrict)
            .take()
    }

    pub fn drop_table() -> TableDropStatement {
        Table::drop().table(DoctorEspecialidad::Table).if_exists().to_owned()
    }
}

#[derive(DeriveIden)]
pub enum Habitacion {
    Table,
    Id,
    Numero,
    Descripcion,
    Piso,
}

impl Habitacion {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(Habitacion::Table)
            .if_not_exists()
            .col(pk_auto(Habitacion::Id))
            .col(small_integer_uniq(Habitacion::Numero))
            .col(text_null(Habitacion::Descripcion))
            .col(string_len(Habitacion::Piso, 20))
            .to_owned()
    }

    pub fn drop_table() -> TableDropStatement {
        Table::drop().table(Habitacion::Table).if_exists().to_owned()
    }
}

#[derive(DeriveIden)]
pub enum Asegurado {
    Table,
    Id,
    UserId,
    /// Identificador expedido por la aseguradora
    NaturalId,
    NombreAseguradora,
    Description,
}

impl Asegurado {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(Asegurado::Table)
            .if_not_exists()
            .col(uuid(Asegurado::Id).primary_key())
            .col(uuid_null(Asegurado::UserId))
            .col(text(Asegurado::NaturalId))
            .col(text(Asegurado::NombreAseguradora))
            .col(text_null(Asegurado::Description))
            .to_owned()
    }

    pub fn table_user_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(Asegurado::Table, Asegurado::UserId)
            .to(Patient::Table, Patient::Id)
            .on_delete(ForeignKeyAction::SetNull)
            .on_update(ForeignKeyAction::Restrict)
            .take()
    }

    pub fn drop_table() -> TableDropStatement {
        Table::drop().table(Asegurado::Table).if_exists().to_owned()
    }
}

#[derive(DeriveIden)]
pub enum Cita {
    Table,
    Id,
    DoctorId,
    PacienteId,
    AseguradoId,
    Fecha,
    TimestampEnd,
    Estado,
    Hospital,
}

impl Cita {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(Cita::Table)
            .if_not_exists()
            .col(uuid(Cita::Id).primary_key())
            .col(uuid_null(Cita::DoctorId))
            .col(uuid_null(Cita::PacienteId))
            .col(uuid_null(Cita::AseguradoId))
            .col(timestamp(Cita::Fecha))
            .col(timestamp_null(Cita::TimestampEnd))
            .col(string_len(Cita::Estado, 16))
            .col(string_len_null(Cita::Hospital, 100))
            .to_owned()
    }

    pub fn table_doctor_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(Cita::Table, Cita::DoctorId)
            .to(Doctor::Table, Doctor::Id)
            .on_delete(ForeignKeyAction::SetNull)
            .on_update(ForeignKeyAction::Restrict)
            .take()
    }

    pub fn table_patient_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(Cita::Table, Cita::PacienteId)
            .to(Patient::Table, Patient::Id)
            .on_delete(ForeignKeyAction::SetNull)
            .on_update(ForeignKeyAction::Restrict)
            .take()
    }

    pub fn table_asegurado_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(Cita::Table, Cita::AseguradoId)
            .to(Asegurado::Table, Asegurado::Id)
            .on_delete(ForeignKeyAction::SetNull)
            .on_update(ForeignKeyAction::Restrict)
            .take()
    }

    pub fn drop_table() -> TableDropStatement {
        Table::drop().table(Cita::Table).if_exists().to_owned()
    }
}
