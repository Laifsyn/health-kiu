use sea_orm::prelude::*;

use super::prelude::*;
use crate::domain::Pagination;
pub trait SpecialtyRepo {
    async fn get_specialties(
        &self,
        paging: impl Into<Pagination>,
    ) -> Result<Vec<DbEspecialidad>, DbErr>;
}
