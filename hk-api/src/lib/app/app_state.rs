//! Describes the application's state and its dependencies.

use sea_orm::prelude::*;
/// The main application state.
/// Holds the database connection and other shared resources.
pub struct AppState {
    repo: DatabaseConnection,
}

impl AppState {
    #[cfg(test)]
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

impl AppState {
    /// Builds a paginated select query for the given entity type `T`.
    ///
    /// ```rs, ignore
    /// 
    /// use models::especialidad;
    /// use models::prelude::Especialidad;
    /// use models::prelude::Especialidad;
    /// app.select_paginated::<Especialidad>(Pagination::new(0, 10).unwrap());
    /// ```
    pub fn select_paginated<T>(&self, pagination: Pagination) -> Select<T>
    where
        T: EntityTrait,
    {
        // Destructure pagination parameters
        let (offset, limit) = pagination.tuple();
        let limit = std::cmp::max(limit.into(), 1) as u64;

        <T>::find()
            .limit(Some(limit))
            .offset(NonZero::new(offset).map(NonZero::get))
    }
}

impl DoctorRepo for AppState {
    async fn get_doctors_by_specialty(
        &self,
        specialty_id: SpecialtyId,
        pagination: impl Into<Pagination>,
    ) -> Result<Option<(DbEspecialidad, Vec<DbDoctor>)>, DbErr> {
        let pagination = pagination.into();
        let results = self
            .select_paginated::<Especialidad>(pagination)
            .filter(especialidad::Column::Id.eq(specialty_id.0))
            .find_also_related(doctor::Entity)
            .order_by_asc(doctor::Column::Name)
            .all(self.connection())
            .await?;

        let Some(specialty) =
            results.first().map(|(especialidad, _)| especialidad.clone())
        else {
            return Ok(None);
        };

        let doctors: Vec<DbDoctor> =
            results.into_iter().filter_map(|(_, doctor)| doctor).collect();

        Ok(Some((specialty, doctors)))
    }
}

impl SpecialtyRepo for AppState {
    /// Fetches a paginated list of specialties from the database ordered by id.
    async fn get_specialties(
        &self,
        paging: impl Into<Pagination>,
    ) -> Result<Vec<especialidad::Model>, DbErr> {
        let paging = paging.into();
        self.select_paginated::<Especialidad>(paging)
            .order_by_asc(especialidad::Column::Id)
            .all(self.connection())
            .await
    }
}
