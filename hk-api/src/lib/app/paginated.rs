use std::num::NonZero;

use models::especialidad;
use models::prelude::Especialidad;
use sea_orm::prelude::*;
use sea_orm::{DbErr, QueryOrder, QuerySelect};

use crate::ServerApp;
use crate::domain::internal::Pagination;

impl ServerApp {
    /// Builds a paginated select query for the given entity type `T`.
    ///
    /// # Arguments
    /// - `offset`: The offset to start fetching records from. If `None`, starts
    ///   from the beginning.
    /// - `limit`: The maximum number of records to fetch. Defaults to 1 when
    ///   smaller than 1 (0).
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
