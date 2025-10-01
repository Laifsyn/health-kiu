//! Describes the application's state and its dependencies.

use std::sync::Arc;

use sea_orm::prelude::*;

use crate::adapters::crypto::ArgonHasher;
use crate::repo::OrmDB;
/// The main application state.
/// Holds the database connection and other shared resources.
pub struct AppState {
    /// Password Hasher
    pub(super) hasher: Arc<ArgonHasher>,
    pub(super) db: OrmDB,
}

impl AppState {
    #[cfg(test)]
    const TEST_DB_ENV_VAR: &'static str = "API_TEST_DB_URL";

    /// Creates a new instance of the application with a database connection
    /// using the [`Self::TEST_DB_ENV_VAR`] environment variable.
    #[cfg(test)]
    pub async fn new_for_test() -> Arc<Self> {
        use std::sync::OnceLock;

        use tokio::sync::OnceCell as TokioOnceCell;

        crate::init_env();
        static DB: TokioOnceCell<OrmDB> = TokioOnceCell::const_new();
        static INIT: OnceLock<Arc<AppState>> = OnceLock::new();
        let db = DB.get_or_init(test_db).await;
        let db = db.clone();
        let this = INIT.get_or_init(|| {
            Arc::new(Self { db, hasher: Self::default_hasher() })
        });
        this.clone()
    }

    fn default_hasher() -> Arc<ArgonHasher> { Arc::new(ArgonHasher::default()) }
}

#[cfg(test)]
async fn test_db() -> OrmDB {
    use sea_orm::Database;

    let database_url =
        std::env::var(AppState::TEST_DB_ENV_VAR).unwrap_or_else(|_| {
            panic!(
                "{} must be set in order to run tests",
                AppState::TEST_DB_ENV_VAR
            )
        });

    Database::connect(&database_url)
        .await
        .map(OrmDB::from_inner)
        .expect("Failed to connect to the database")
}

/// Public implementation of the application state.
impl AppState {
    /// Returns a reference to the inner database connection.
    pub fn inner_connection(&self) -> &DatabaseConnection {
        &self.db.connection()
    }
}
