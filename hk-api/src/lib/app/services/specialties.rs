use super::prelude::*;
use crate::domain::Paged;
use crate::domain::dto::specialty::*;
pub trait SpecialtyService {
    async fn get_specialties(
        &self,
        pagination: impl Into<Pagination>,
    ) -> Result<Paged<Specialty>>;
}

impl SpecialtyService for AppState {
    async fn get_specialties(
        &self,
        pagination: impl Into<Pagination>,
    ) -> Result<Paged<Specialty>> {
        let pagination = pagination.into();
        let specialties = self
            .db
            .get_specialties(pagination.clone())
            .await
            .map(|items| {
                Paged::new_with_transform(items, pagination, Specialty::from)
            })
            .map_err(AppError::err_with("Failed to get specialties"))?;

        Ok(specialties)
    }
}
