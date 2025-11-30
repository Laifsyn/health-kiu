use std::panic::Location;

use color_eyre::eyre::Report;
use sea_orm_migration as som;
pub use sea_orm_migration::prelude::*;

mod m20250904_203638_create_tables;
mod m20251130_000000_add_timestamp_end;

/// Drop-in replacement for [`sea_orm_migration::schema::pk_auto`].
/// Replaces the `integer` type with [`big_integer`](ColumnType::BigInteger).
///
/// Create a primary key column with auto-increment feature.
pub fn pk_auto<T: IntoIden>(name: T) -> ColumnDef {
    som::schema::pk_auto(name).big_integer().take()
}

#[track_caller]
/// Helper function to log errors with the given context.
///
/// The returned closure can be used with [`Result::map_err`] to log the error
/// along with the provided context and the caller's location.
///
/// ```rust,ignore
/// use crate::log_with_context as lwc;
/// manager
///     .create_table(User::create_table())
///     .await
///     .map_err(lwc("Failed to create user table"))?;
/// ```
fn log_with_context(ctx: impl Into<String>) -> impl FnOnce(DbErr) -> DbErr {
    let ctx = ctx.into();
    let location = Location::caller();
    move |err| {
        let mut report = Report::new(err);
        report.handler_mut().track_caller(location);

        tracing::error!("{:?}", report);
        DbErr::Custom(ctx)
    }
}

// /// Postgres specific method to add an `INCLUDE` clause to indexes.
// /// to be used in conjunction with [`ColumnDef::extra`].
// ///
// /// Example:
// /// ```rust,ignore
// /// use sea_orm_migration::prelude::*;
// /// use sea_orm_migration::schema::*;
// /// #[derive(DeriveIden)]
// /// pub enum User {
// ///     Table,
// ///     Id,
// ///     Cedula,
// ///     Passport,
// /// }
// /// string(User::Cedula).unique_key().extra(pg_include(User::Id))
// /// ```
// pub fn pg_include<T: IntoIden>(iden: T) -> String {
//     format!("INCLUDE({})", iden.into_iden().to_string())
// }

/// Automatically add migrations via `sea-orm-cli migrate generate
/// 'create_tables'` for more manually adding migrations, see https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250904_203638_create_tables::Migration),
            Box::new(m20251130_000000_add_timestamp_end::Migration),
        ]
    }
}
