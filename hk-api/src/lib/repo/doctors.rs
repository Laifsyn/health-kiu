use models::doctor::Model as DbDoctor;
use models::especialidad::Model as DbEspecialidad;
use sea_orm::prelude::*;

use crate::domain::Pagination;
use crate::domain::dto::SpecialtyId;
pub trait DoctorRepo {
    async fn get_doctors_by_specialty(
        &self,
        specialty_id: SpecialtyId,
        pagination: impl Into<Pagination>,
    ) -> Result<Option<(DbEspecialidad, Vec<DbDoctor>)>, DbErr>;
}
