use models::especialidad;
use models::prelude::Especialidad;
use sea_orm::QueryOrder;
use sea_orm::prelude::*;

use crate::ServerApp;
use crate::domain::Pagination;

pub trait SpecialtyRepo {
    async fn get_specialties(
        &self,
        paging: impl Into<Pagination>,
    ) -> Result<Vec<especialidad::Model>, DbErr>;
}

impl SpecialtyRepo for ServerApp {
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
