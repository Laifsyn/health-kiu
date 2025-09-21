use std::num::NonZero;

use models::especialidad;
use models::prelude::Especialidad;
use sea_orm::prelude::*;
use sea_orm::{DbErr, QueryOrder, QuerySelect};

use crate::ServerApp;
use crate::domain::Pagination;

impl ServerApp {
    /// Builds a paginated select query for the given entity type `T`.
    ///
    /// ```rs, ignore
    /// use models::prelude::Especialidad;
    /// app.select_paginated::<Especialidad>(Pagination::new(0, 10).unwrap());
    /// ```
    fn select_paginated<T>(&self, pagination: Pagination) -> Select<T>
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

    /// Fetches a paginated list of specialties from the database ordered by id.
    pub async fn get_specialties(
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
