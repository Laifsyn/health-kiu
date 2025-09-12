use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

use crate::log_with_context as lwc;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(User::create_table())
            .await
            .map_err(lwc("Failed to create user table"))?;

        manager
            .create_index(User::create_cedula_index())
            .await
            .map_err(lwc("Failed to create user.cedula index"))?;

        manager
            .create_table(Doctor::create_table())
            .await
            .map_err(lwc("Failed to create doctor table"))?;

        manager
            .create_foreign_key(Doctor::table_user_fk())
            .await
            .map_err(lwc("Failed to relate doctor.user_id -> user.id"))?;

        manager
            .create_table(Patient::create_table())
            .await
            .map_err(lwc("Failed to create patient table"))?;

        manager
            .create_foreign_key(Patient::table_user_fk())
            .await
            .map_err(lwc("Failed to relate patient.user_id -> user.id"))?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager // This will also drop related indexes
            .drop_table(User::drop_table())
            .await
            .map_err(lwc("Failed to drop `User` table"))?;

        manager
            .drop_table(Doctor::drop_table())
            .await
            .map_err(lwc("Failed to drop `Doctor` table"))?;

        manager
            .drop_table(Patient::drop_table())
            .await
            .map_err(lwc("Failed to drop `Patient` table"))?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
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
enum Doctor {
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
enum Patient {
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
