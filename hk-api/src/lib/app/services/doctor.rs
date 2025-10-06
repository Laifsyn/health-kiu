use super::prelude::*;
use crate::domain::Paged;
use crate::domain::dto::doctor::*;
use crate::domain::dto::specialty::Specialty;

pub(crate) trait DoctorService {
    async fn get_doctors(
        &self,
        pagination: impl Into<Pagination>,
    ) -> AppResult<Paged<Doctor>>;

    async fn get_doctor(&self, id: UserId) -> AppResult<Option<Doctor>>;

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
        let pagination = pagination.into();
        let Some(doctors) = self
            .db
            .get_doctors_by_specialty(specialty_id, pagination)
            .await
            .map_err(AppError::err_with(
                "Failed to get doctors by specialty",
            ))?
        else {
            return Ok(None);
        };

        // Ok(doctors)
        todo!()
    }

    async fn get_doctors(
        &self,
        pagination: impl Into<Pagination>,
    ) -> AppResult<Paged<Doctor>> {
        let pagination = pagination.into();
        let result = self.db.get_doctors(pagination.clone()).await;
        todo!()
        // Ok(Paged::new(result, pagination))
    }

    async fn get_doctor(&self, id: UserId) -> AppResult<Option<Doctor>> {
        let doctor = self.db.get_doctor(id.0).await;
        // if let None = doctor {
        //     return Ok(None);
        // }
        // let doctor = doctor.unwrap();
        todo!()
    }
}
