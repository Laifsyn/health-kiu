//! Describes the application's state and its dependencies.

use std::sync::Arc;

use sea_orm::ConnectOptions;
use sea_orm::prelude::*;

use crate::adapters::crypto::{AppKeys, ArgonHasher};
use crate::repo::OrmDB;
/// The main application state.
/// Holds the database connection and other shared resources.
#[derive(Clone)]
pub struct AppState {
    /// Password Hasher
    pub(super) hasher: Arc<ArgonHasher>,
    /// Database Connection
    pub(super) db: OrmDB,
    pub(super) public_key: Arc<[u8]>,
    pub(super) private_key: Arc<[u8]>,
}

impl AppState {
    /// Default environment variable for the database URL.
    pub const DB_URL_ENV_VAR: &'static str = "API_DB_URL";
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
            let AppKeys { private, public } = Self::keys();
            Arc::new(Self {
                db,
                hasher: Self::default_hasher(),
                public_key: public,
                private_key: private,
            })
        });
        this.clone()
    }

    /// Creates a new instance of the application state.
    pub async fn new(db_url: Option<String>) -> color_eyre::eyre::Result<Self> {
        use sea_orm::Database;

        let database_url = db_url.unwrap_or_else(|| {
            std::env::var(Self::DB_URL_ENV_VAR).expect(
                "DATABASE_URL must be set in order to run the application",
            )
        });

        let mut connect_options = ConnectOptions::new(database_url.clone());
        // TODO: Use env variables to configure the connection options.
        connect_options.max_connections(5);

        let db_conn = Database::connect(connect_options).await?;
        let db = OrmDB::from_inner(db_conn);
        let AppKeys { private, public } = Self::keys();

        Ok(Self {
            db,
            hasher: Self::default_hasher(),
            public_key: public,
            private_key: private,
        })
    }

    pub fn public_key(&self) -> &[u8] { self.public_key.as_ref() }

    pub fn private_key(&self) -> &[u8] { self.private_key.as_ref() }

    /// Reads from the [`fs`](std::fs), and returns the stored keys.
    ///
    /// # PANICS
    /// Panics if the keys cannot be read from the file system.
    fn keys() -> AppKeys { AppKeys::new() }

    /// Generates a default password hasher.
    fn default_hasher() -> Arc<ArgonHasher> { Arc::new(ArgonHasher::default()) }

    /// Returns a reference to the inner database connection.
    pub fn inner_connection(&self) -> &DatabaseConnection {
        self.db.connection()
    }

    /// Returns a reference to the database for repository operations.
    pub fn db(&self) -> &OrmDB {
        &self.db
    }
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
