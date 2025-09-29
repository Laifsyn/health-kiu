use super::prelude::*;
use crate::domain::dto::specialty::*;
pub trait SpecialtyService {
    async fn get_all_specialties(
        &self,
        pagination: impl Into<Pagination>,
    ) -> AppResult<Vec<Specialty>>;
}
