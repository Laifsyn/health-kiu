use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Cita::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Cita::TimestampEnd)
                            .timestamp()
                            .null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Cita::Table)
                    .drop_column(Cita::TimestampEnd)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Cita {
    Table,
    TimestampEnd,
}
