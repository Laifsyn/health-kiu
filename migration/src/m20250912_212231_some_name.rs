#![expect(unused)]

use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // manager
        //     .create_table(
        //         Table::create()
        //             .table(Post::Table)
        //             .if_not_exists()
        //             .col(pk_auto(Post::Id))
        //             .col(string(Post::Title))
        //             .col(string(Post::Text))
        //             .to_owned(),
        //     )
        //     .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // manager.drop_table(Table::drop().table(Post::Table).to_owned()).
        // await?;
        Ok(())
    }
}
