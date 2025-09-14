use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

use crate::log_with_context as lwc;
use crate::m20250904_203638_create_tables::{Doctor, Patient, User};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Especialidad table
        manager
            .create_table(Especialidad::create_table())
            .await
            .map_err(lwc("Failed to create especialidad table"))?;

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
            .map_err(lwc(
                "Failed to relate asegurado.paciente_id -> patient.id"
            ))?;

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

        // Create Itinerario table
        manager
            .create_table(Itinerario::create_table())
            .await
            .map_err(lwc("Failed to create itinerario table"))?;

        manager
            .create_foreign_key(Itinerario::table_doctor_fk())
            .await
            .map_err(lwc(
                "Failed to relate itinerario.doctor_id -> doctor.id"
            ))?;

        manager
            .create_foreign_key(Itinerario::table_habitacion_fk())
            .await
            .map_err(lwc(
                "Failed to relate itinerario.habitacion_id -> habitacion.id"
            ))?;

        // Add foreign key to Doctor table for especialidad_id
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("doctor"))
                    .add_column(integer_null(Alias::new("especialidad_id")))
                    .to_owned(),
            )
            .await
            .map_err(lwc(
                "Failed to add especialidad_id column to doctor table"
            ))?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Alias::new("doctor"), Alias::new("especialidad_id"))
                    .to(Especialidad::Table, Especialidad::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::Restrict)
                    .take(),
            )
            .await
            .map_err(lwc(
                "Failed to relate doctor.especialidad_id -> especialidad.id"
            ))?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Only need to explicitly handle the doctor table alterations
        // since we're not dropping the doctor table itself
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("doctor"))
                    .name("doctor_especialidad_id_fkey") // FIXME: Postgres-specific foreign key name format
                    .take(),
            )
            .await
            .map_err(
                lwc("Failed to drop doctor.especialidad_id foreign key"),
            )?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("doctor"))
                    .drop_column(Alias::new("especialidad_id"))
                    .to_owned(),
            )
            .await
            .map_err(lwc(
                "Failed to drop especialidad_id column from doctor table"
            ))?;

        // Drop tables in reverse order (foreign keys will be automatically
        // dropped)
        manager
            .drop_table(Itinerario::drop_table())
            .await
            .map_err(lwc("Failed to drop Itinerario table"))?;

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
            .drop_table(Especialidad::drop_table())
            .await
            .map_err(lwc("Failed to drop Especialidad table"))?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Especialidad {
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
enum Habitacion {
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
enum Asegurado {
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
enum Cita {
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
