use std::time::Duration;

use color_eyre::eyre::{Context, Result};
use migration::{Migrator, MigratorTrait, sea_orm};
use scopeguard::defer;
use tokio::task::LocalSet;
use tracing::{Level, info_span, span};

const IS_RELEASE: bool = !(cfg!(test) || cfg!(debug_assertions));
const DB_URL_VAR: &str = "API_DB_URL";
const TEST_DB_URL_VAR: &str = "API_TEST_DB_URL";

/// Executes migrations using the `Migrator` defined in the `migration` crate.
/// returns the database URL if successful.
pub async fn main() -> Result<String> {
    let _span = span!(Level::INFO, "prod-db").entered();

    let handle = if !IS_RELEASE {
        tracing::info!("Applying migrations to test database.");
        Some(tokio::task::spawn_blocking(run_test_migrations))
    } else {
        None
    };

    let db_url = migrations(DB_URL_VAR).await?;
    if let Some(handle) = handle
        && !handle.is_finished()
    {
        tracing::info!("Waiting for test database migrations to finish.");
        let _ = handle.await;
    }
    tracing::info!("Migrations completed.");
    Ok(db_url)
}

fn run_test_migrations() -> Result<String> {
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async move {
        let local = LocalSet::new();
        local
            .run_until(async move {
                let _spam = info_span!("test-db").entered();
                defer!(tracing::info!(
                    "Test database migrations thread finished."
                ));
                migrations(TEST_DB_URL_VAR).await.inspect_err(|e| {
                    tracing::error!(
                        error = %e,
                        "Failed to run test database migrations",
                    );
                })
            })
            .await
    })
}

async fn migrations(db_url_env_var: &str) -> Result<String> {
    let db_url = std::env::var(db_url_env_var)
        .wrap_err_with(|| format!("Expects {db_url_env_var} to be set"))?;

    // FIXME: Even if redacted on release, passwords could potentially leak if
    // logs are mishandled.
    {
        let db_url = if !IS_RELEASE { db_url.as_str() } else { "[REDACTED]" };
        tracing::info!("Connecting to database via: {db_url}");
    }

    // Use some lightweight connections to avoid unexpected timeouts when
    // connecting with supabase
    let connection = sea_orm::Database::connect(
        sea_orm::ConnectOptions::new(&db_url)
            .max_connections(5)
            .connect_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(10))
            .to_owned(),
    )
    .await
    .wrap_err("Failed to connect to database")?;

    Migrator::up(&connection, None).await?;
    connection
        .close()
        .await
        .inspect_err(|e| tracing::error!(error=?e,"Error closing the pool"))
        .ok();
    Ok(db_url)
}
