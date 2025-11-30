use entities::Especialidad;
use sea_orm::prelude::*;

use super::prelude::*;
use crate::domain::Pagination;
pub trait SpecialtyRepo {
    async fn get_specialties(
        &self,
        paging: impl Into<Pagination>,
    ) -> AppResult<Vec<DbEspecialidad>, DbErr>;
}

impl SpecialtyRepo for OrmDB {
    /// Fetches a paginated list of specialties from the database ordered by id.
    async fn get_specialties(
        &self,
        paging: impl Into<Pagination>,
    ) -> AppResult<Vec<DbEspecialidad>, DbErr> {
        let paging = paging.into();
        self.select_paginated::<Especialidad>(paging)
            .order_by_asc(especialidad::Column::Id)
            .all(self.connection())
            .await
    }
}
