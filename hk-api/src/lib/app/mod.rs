mod paginated;

use std::sync::Arc;

use sea_orm::Database;
use sea_orm::prelude::*;
use tokio::sync::OnceCell as TokioOnceCell;

pub struct ServerApp {
    repo: DatabaseConnection,
}

impl ServerApp {
    pub async fn new_for_test() -> Arc<Self> {
        crate::init_env();
        static DB: TokioOnceCell<DatabaseConnection> =
            TokioOnceCell::const_new();
        let db = DB
            .get_or_init(|| {
                async {
                    let database_url = std::env::var("API_TEST_DB_URL").expect(
                        "API_TEST_DB_URL must be set in order to run tests",
                    );
                    let db = Database::connect(&database_url)
                        .await
                        .expect("Failed to connect to the database");
                    db
                }
            })
            .await;
        let db = db.clone();
        Arc::new(Self { repo: db })
    }

    pub fn connection(&self) -> &DatabaseConnection { &self.repo }
}
