use crate::domain::dto::SpecialtyId;
use crate::services::prelude::*;
pub trait DoctorService {
    async fn get_doctors_by_specialty(
        &self,
        specialty_id: SpecialtyId,
        pagination: impl Into<Pagination>,
    ) -> Result<Vec<DbDoctor>>;
}
