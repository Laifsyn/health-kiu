use sea_orm_migration::prelude::*;
#[async_std::main]
async fn main() {
    // Run the migration
    cli::run_cli(migration::Migrator).await;
    tracing::info!("Migration completed successfully");
}
