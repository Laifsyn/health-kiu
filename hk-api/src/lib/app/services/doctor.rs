use super::prelude::*;

pub trait DoctorService {
    async fn get_doctors_by_specialty(
        &self,
        specialty_id: SpecialtyId,
        pagination: impl Into<Pagination>,
    ) -> AppResult<Option<(DbEspecialidad, Vec<DbDoctor>)>>;
}
