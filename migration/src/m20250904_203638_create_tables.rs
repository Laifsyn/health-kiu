use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

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
            .create_foreign_key(Asegurado::table_patient_fk())
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
            .col(string(User::Cedula).char_len(20).unique_key())
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
    UserId,
    PasswordHash,
}

impl Doctor {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(Doctor::Table)
            .if_not_exists()
            .col(uuid(Doctor::Id).primary_key())
            .col(uuid_null(Doctor::UserId).unique_key())
            .col(text(Doctor::PasswordHash))
            .to_owned()
    }

    pub fn table_user_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(Doctor::Table, Doctor::UserId)
            .to(User::Table, User::Id)
            .on_delete(ForeignKeyAction::SetNull)
            .on_update(ForeignKeyAction::Restrict)
            .take()
    }

    pub fn drop_table() -> TableDropStatement {
        Table::drop().table(Doctor::Table).if_exists().to_owned()
    }
}

#[derive(DeriveIden)]
pub enum Patient {
    Table,
    Id,
    UserId,
    PasswordHash,
}

impl Patient {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(Patient::Table)
            .if_not_exists()
            .col(uuid(Patient::Id).primary_key())
            .col(uuid_null(Patient::UserId).unique_key())
            .col(text(Patient::PasswordHash))
            .to_owned()
    }

    pub fn table_user_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(Patient::Table, Patient::UserId)
            .to(User::Table, User::Id)
            .on_delete(ForeignKeyAction::SetNull)
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
}

impl Especialidad {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(Especialidad::Table)
            .if_not_exists()
            .col(crate::pk_auto(Especialidad::Id).small_integer())
            .col(string_len(Especialidad::Nombre, 100))
            .to_owned()
    }

    pub fn drop_table() -> TableDropStatement {
        Table::drop().table(Especialidad::Table).if_exists().to_owned()
    }
}

#[derive(DeriveIden)]
pub enum DoctorEspecialidad {
    Table,
    DoctorId,
    EspecialidadId,
    FechaCertificacion,
    Activo,
}

impl DoctorEspecialidad {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(DoctorEspecialidad::Table)
            .if_not_exists()
            .col(uuid(DoctorEspecialidad::DoctorId))
            .col(small_integer(DoctorEspecialidad::EspecialidadId))
            .col(date_null(DoctorEspecialidad::FechaCertificacion))
            .col(boolean(DoctorEspecialidad::Activo).default(true))
            .primary_key(
                Index::create()
                    .col(DoctorEspecialidad::DoctorId)
                    .col(DoctorEspecialidad::EspecialidadId),
            )
            .to_owned()
    }

    pub fn table_doctor_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(DoctorEspecialidad::Table, DoctorEspecialidad::DoctorId)
            .to(Doctor::Table, Doctor::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Restrict)
            .take()
    }

    pub fn table_especialidad_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(DoctorEspecialidad::Table, DoctorEspecialidad::EspecialidadId)
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
            .col(crate::pk_auto(Habitacion::Id))
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
    PatientId,
    /// Identificador expedido por la aseguradora
    NaturalId,
    Description,
}

impl Asegurado {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(Asegurado::Table)
            .if_not_exists()
            .col(uuid(Asegurado::Id).primary_key())
            .col(uuid_null(Asegurado::PatientId))
            .col(string_len(Asegurado::NaturalId, 50).unique_key())
            .col(text_null(Asegurado::Description))
            .to_owned()
    }

    pub fn table_patient_fk() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .from(Asegurado::Table, Asegurado::PatientId)
            .to(User::Table, User::Id)
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
    Estado,
}

impl Cita {
    pub fn create_table() -> TableCreateStatement {
        Table::create()
            .table(Cita::Table)
            .if_not_exists()
            .col(uuid(Cita::Id).primary_key())
            .col(uuid_null(Cita::DoctorId))
            .col(uuid_null(Cita::PacienteId))
            .col(timestamp(Cita::Fecha))
            .col(uuid_null(Cita::AseguradoId))
            .col(string_len(Cita::Estado, 50))
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
