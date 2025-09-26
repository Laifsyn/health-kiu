mod r#impl;

use sea_orm::prelude::*;
/// The main application state.
/// Holds the database connection and other shared resources.
pub struct ServerApp {
    repo: DatabaseConnection,
}

impl ServerApp {
    const TEST_DB_ENV_VAR: &'static str = "API_TEST_DB_URL";

    /// Creates a new instance of the application with a database connection
    /// using the [`Self::TEST_DB_ENV_VAR`] environment variable.
    #[cfg(test)]
    pub async fn new_for_test() -> std::sync::Arc<Self> {
        use sea_orm::Database;
        use tokio::sync::OnceCell as TokioOnceCell;

        crate::init_env();
        static DB: TokioOnceCell<DatabaseConnection> =
            TokioOnceCell::const_new();
        let db = DB
            .get_or_init(|| {
                async {
                    let database_url = std::env::var(Self::TEST_DB_ENV_VAR)
                        .unwrap_or_else(|_| {
                            panic!(
                                "{} must be set in order to run tests",
                                Self::TEST_DB_ENV_VAR
                            )
                        });

                    Database::connect(&database_url)
                        .await
                        .expect("Failed to connect to the database")
                }
            })
            .await;
        let db = db.clone();
        std::sync::Arc::new(Self { repo: db })
    }

    pub fn connection(&self) -> &DatabaseConnection { &self.repo }
}
