use sea_orm::TransactionTrait;
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
    ) -> AppResult<Option<(DbEspecialidad, Vec<DoctorUser>)>>;

    async fn get_doctors(
        &self,
        pagination: Pagination,
    ) -> AppResult<Vec<DoctorUser>>;

    async fn get_doctor(
        &self,
        id: impl Into<Ulid>,
    ) -> AppResult<Option<DoctorUser>>;

    async fn get_appointments(
        &self,
        id: impl Into<Ulid>,
    ) -> AppResult<Vec<DbCita>>;

    async fn get_doctor_by_cedula(
        &self,
        cedula: &str,
    ) -> AppResult<Option<DoctorUser>>;

    async fn register_doctor(
        &self,
        payload: RegisterDbDoctor,
    ) -> AppResult<DoctorUser>;
}

impl DoctorRepo for OrmDB {
    /// Retrieves doctors associated with a specific specialty, ordering them by
    /// name.
    async fn get_doctors_by_specialty(
        &self,
        specialty_id: SpecialtyId,
        pagination: Pagination,
    ) -> AppResult<Option<(DbEspecialidad, Vec<DoctorUser>)>> {
        let results: Vec<(DbDoctor, Option<DbEspecialidad>, Option<DbUser>)> =
            self.select_paginated::<doctor::Entity>(pagination)
                .find_also_related(especialidad::Entity)
                .find_also_related(user::Entity)
                .filter(especialidad::Column::Id.eq(specialty_id))
                .order_by_asc(doctor::Column::Name)
                .all(self.connection())
                .await?
                .into_iter()
                .filter(|(_, e, _)| e.is_some())
                .collect();

        // extract the first specialty (if any) and all associated doctors
        let Some(specialty) = results
            .first()
            .and_then(|(_, especialidad, ..)| especialidad.clone())
        else {
            return Ok(None);
        };

        assert!(
            results.iter().all(|(_, _, user)| user.is_some()),
            "All doctors must have an associated user record"
        );

        assert!(
            results
                .iter()
                .all(|(_, e, ..)| e.as_ref().unwrap().id == specialty.id),
            "Fetched doctors must be members of the same specialty"
        );

        let docs: Vec<DoctorUser> = results
            .into_iter()
            .filter_map(|(d, _, u)| u.map(|u| (d, u)))
            .collect();

        Ok(Some((specialty, docs)))
    }

    /// Retrieves a paginated list of doctors, ordered by name.
    async fn get_doctors(
        &self,
        pagination: Pagination,
    ) -> AppResult<Vec<DoctorUser>> {
        let doctors = self
            .select_paginated::<doctor::Entity>(pagination)
            .find_also_related(user::Entity)
            .order_by_asc(doctor::Column::Name)
            .all(self.connection())
            .await?;
        let doctors =
            doctors.into_iter().filter_map(flatten_doctor_user).collect();
        Ok(doctors)
    }

    async fn get_doctor(
        &self,
        id: impl Into<Ulid>,
    ) -> AppResult<Option<DoctorUser>> {
        doctor::Entity::find_by_id(id.into())
            .find_also_related(user::Entity)
            .one(self.connection())
            .await
            .map(|t| t.and_then(flatten_doctor_user))
    }

    async fn get_appointments(
        &self,
        id: impl Into<Ulid>,
    ) -> AppResult<Vec<DbCita>> {
        cita::Entity::find()
            .find_also_related(doctor::Entity)
            .filter(cita::Column::DoctorId.eq(id.into()))
            .order_by(cita::Column::Fecha, sea_orm::Order::Asc)
            .all(self.connection())
            .await?;
        unimplemented!("I don't know what to write here yet")
    }

    async fn get_doctor_by_cedula(
        &self,
        cedula: &str,
    ) -> AppResult<Option<DoctorUser>> {
        doctor::Entity::find()
            .find_also_related(user::Entity)
            .filter(user::Column::Cedula.eq(cedula))
            .one(self.connection())
            .await
            .map(|t| t.and_then(flatten_doctor_user))
    }

    async fn register_doctor(
        &self,
        payload: RegisterDbDoctor,
    ) -> AppResult<DoctorUser> {
        use sea_orm::ActiveValue::Set;
        let RegisterDbDoctor { name, password_hash, passport, cedula, id } =
            payload;

        let doctor_id: Uuid = id.unwrap_or_default().as_uuid();

        let user_active_model = user::ActiveModel {
            id: Set(doctor_id),
            passport: Set(passport),
            cedula: Set(cedula),
        };

        let doctor_active_model = doctor::ActiveModel {
            id: Set(doctor_id),
            name: Set(name),
            password_hash: Set(password_hash),
        };

        let txn = self.connection().begin().await?;

        let user: DbUser = user_active_model.insert(&txn).await?;
        let doctor: DbDoctor = doctor_active_model.insert(&txn).await?;

        txn.commit().await?;

        Ok((doctor, user))
    }
}

/// Helper function to flatten a tuple of ([`Doctor`](DbDoctor),
/// Option<[`User`](DbUser)>) into an Option<([`DoctorUser`])>
fn flatten_doctor_user(
    doctor_user: (DbDoctor, Option<DbUser>),
) -> Option<DoctorUser> {
    let (doctor, user) = doctor_user;
    if user.is_none() {
        tracing::warn!(
            "Doctor with id {} has no associated user record",
            doctor.id
        );
        return None;
    }
    user.map(|user| (doctor, user))
}

#[cfg(test)]
mod tests {

    use sea_orm::prelude::*;
    use sea_orm::{DbBackend, QueryTrait};

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
