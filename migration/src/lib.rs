use sea_orm_migration as som;
pub use sea_orm_migration::prelude::*;

mod m20250904_203638_create_tables;

/// Automatically add migrations via `sea-orm-cli migrate generate
/// 'create_tables'` for more manually adding migrations, see https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250904_203638_create_tables::Migration)]
    }
}

/// Drop-in replacement for [`sea_orm_migration::schema::pk_auto`].
/// Replaces the `integer` type with [`big_integer`](ColumnType::BigInteger).
///
/// Create a primary key column with auto-increment feature.
pub fn pk_auto<T: IntoIden>(name: T) -> ColumnDef {
    som::schema::pk_auto(name).big_integer().take()
}
