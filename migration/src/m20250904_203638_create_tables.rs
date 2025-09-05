use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

pub use super::pk_auto;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Username))
                    .col(string(User::Pasword))
                    .to_owned(),
            )
            .await
            .inspect_err(|e| {
                tracing::warn!("Failed to create user table: {}", e)
            })?;

        manager
            .create_table(
                Table::create()
                    .table(Doctor::Table)
                    .if_not_exists()
                    .col(pk_auto(Doctor::Id))
                    .col(string(Doctor::UserId))
                    .to_owned(),
            )
            .await
            .inspect_err(|e| {
                tracing::warn!("Failed to create user table: {}", e)
            })?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
            .inspect_err(|e| tracing::warn!("Failed to drop user table: {}", e))?;
        manager
            .drop_table(Table::drop().table(Doctor::Table).to_owned())
            .await
            .inspect_err(|e| {
                tracing::warn!("Failed to drop doctor table: {}", e)
            })?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Pasword,
}

#[derive(DeriveIden)]
enum Doctor {
    Table,
    Id,
    UserId,
}
