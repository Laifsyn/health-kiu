mod entities;
mod id;
/// Flattened re-exports for easier access to Models' types.
pub use entities::*;
pub use id::Id;

/// Re-exports Models' Entities
pub mod prelude {
    pub use super::entities::prelude::*;
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::time::Duration;

    use sea_orm::{EntityTrait, QuerySelect};
    use tokio::sync::OnceCell;

    use super::prelude::*;

    fn init_env() -> Option<PathBuf> {
        if cfg!(debug_assertions) {
            dotenvy::from_filename(".env.development").ok();
        }
        dotenvy::dotenv().ok()
    }

    async fn database_connection() -> &'static sea_orm::DatabaseConnection {
        static DB_CONN: OnceCell<sea_orm::DatabaseConnection> =
            OnceCell::const_new();
        DB_CONN
            .get_or_init(|| {
                async {
                    init_env();
                    sea_orm::Database::connect(
                        sea_orm::ConnectOptions::new(
                            std::env::var("API_TEST_DB_URL").unwrap(),
                        )
                        .max_connections(5)
                        .connect_timeout(Duration::from_secs(30))
                        .idle_timeout(Duration::from_secs(10))
                        .to_owned(),
                    )
                    .await
                    .unwrap()
                }
            })
            .await
    }

    #[tokio::test]
    async fn it_works() {
        let conn = database_connection().await;
        let cursor = Especialidad::find()
            .offset(None)
            .limit(20)
            .all(conn)
            .await
            .unwrap();
        println!("{:#?}", cursor);
    }
}
