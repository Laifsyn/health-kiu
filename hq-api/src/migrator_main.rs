use migration::{DbErr, sea_orm};
use scopeguard::defer;

/// Executes migrations using the `Migrator` defined in the `migration` crate.
/// returns the database URL if successful.
pub async fn main() -> Result<String, DbErr> {
    use migration::{Migrator, MigratorTrait};
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::info!("Starting migrations on database: {}", database_url);
    defer! {
        tracing::info!("Migrations complete.");
    };
    let connection = sea_orm::Database::connect(&database_url).await?;
    Migrator::up(&connection, None).await?;
    Ok(database_url)
}
