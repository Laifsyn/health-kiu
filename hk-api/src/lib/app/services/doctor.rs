use super::prelude::*;
use crate::domain::Paged;
use crate::domain::dto::doctor::*;
use crate::domain::dto::specialty::Specialty;

pub trait DoctorService {
    async fn get_doctors_by_specialty(
        &self,
        specialty_id: SpecialtyId,
        pagination: impl Into<Pagination>,
    ) -> AppResult<Option<(Specialty, Paged<Doctor>)>>;
}

impl DoctorService for AppState {
    async fn get_doctors_by_specialty(
        &self,
        specialty_id: SpecialtyId,
        pagination: impl Into<Pagination>,
    ) -> AppResult<Option<(Specialty, Paged<Doctor>)>> {
        let Some(doctors) = self
            .db
            .get_doctors_by_specialty(specialty_id, pagination)
            .await
            .map_err(AppError::map_err_with(
                "Failed to get doctors by specialty",
            ))?
        else {
            return Ok(None);
        };

        // Ok(doctors)
        todo!()
    }
}
