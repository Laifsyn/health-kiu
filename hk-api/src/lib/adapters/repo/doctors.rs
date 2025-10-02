use entities::Especialidad;
use sea_orm::prelude::*;

use super::prelude::*;
use crate::domain::Pagination;
use crate::domain::dto::SpecialtyId;
/// Type alias for a [`Doctor`](doctor::Model) along with their associated
/// [`User`](user::Model) record.
pub type DoctorUser = (DbDoctor, DbUser);

/// Repository trait for doctor-related database operations.
pub(crate) trait DoctorRepo {
    async fn get_doctors_by_specialty(
        &self,
        specialty_id: SpecialtyId,
        pagination: Pagination,
    ) -> repo::Result<Option<(DbEspecialidad, Vec<DoctorUser>)>>;

    async fn get_doctors(
        &self,
        pagination: Pagination,
    ) -> repo::Result<Vec<DoctorUser>>;

    async fn get_doctor(&self, id: Ulid) -> repo::Result<Option<DoctorUser>>;
}

impl DoctorRepo for OrmDB {
    /// Retrieves doctors associated with a specific specialty, ordering them by
    /// name.
    async fn get_doctors_by_specialty(
        &self,
        specialty_id: SpecialtyId,
        pagination: Pagination,
    ) -> repo::Result<Option<(DbEspecialidad, Vec<DoctorUser>)>> {
        let results = self
            .select_paginated::<doctor::Entity>(pagination)
            .find_also_related(especialidad::Entity)
            .find_also_related(user::Entity)
            .filter(especialidad::Column::Id.eq(specialty_id.0))
            .order_by_asc(doctor::Column::Name)
            .all(self.connection())
            .await?;

        // extract the first specialty (if any) and all associated doctors
        // let Some(specialty) =
        //     results.first().map(|(especialidad, _)| especialidad.clone())
        // else {
        //     return Ok(None);
        // };

        // assert!(
        //     results.iter().all(|(e, _)| e.id == specialty.id),
        //     "Fetched doctors must be members of the same specialty"
        // );

        // let doctors: Vec<DoctorUser> =
        //     results.into_iter().filter_map(|(_, doctor)| doctor).collect();

        // Ok(Some((specialty, doctors)))
        todo!()
    }

    /// Retrieves a paginated list of doctors, ordered by name.
    async fn get_doctors(
        &self,
        pagination: Pagination,
    ) -> repo::Result<Vec<DoctorUser>> {
        let total = doctor::Entity::find().count(self.connection()).await?;
        let doctors = self
            .select_paginated::<doctor::Entity>(pagination)
            .order_by_asc(doctor::Column::Name)
            .all(self.connection())
            .await?;

        // Ok(doctors)
        todo!()
    }

    async fn get_doctor(&self, id: Ulid) -> repo::Result<Option<DoctorUser>> {
        let r = doctor::Entity::find_by_id(id.as_uuid())
            .find_also_related(user::Entity)
            .one(self.connection())
            .await;
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use sea_orm::prelude::*;
    use sea_orm::{DbBackend, JoinType, QuerySelect, QueryTrait};

    use super::*;
    #[test]
    fn test() {
        let results = doctor::Entity::find()
            .find_also_related(user::Entity)
            .find_also_related(especialidad::Entity)
            .filter(especialidad::Column::Id.eq(11))
            .order_by_asc(doctor::Column::Name)
            .build(DbBackend::Postgres);
        // let results = especialidad::Entity::find_by_id(11_i16)
        //     .find_also_related(doctor::Entity)
        //     .tbl_col_as((user::Entity, user::Column::Cedula), "user_cedula")
        //     .tbl_col_as((user::Entity, user::Column::Passport),
        // "user_passport")     .find_also_related(user::Entity)
        //     .join_rev(JoinType::LeftJoin, user::Relation::Doctor.def())
        //     .order_by_asc(doctor::Column::Name)
        //     .build(DbBackend::Postgres);
        println!("result:\n{results:#}");
    }
}
