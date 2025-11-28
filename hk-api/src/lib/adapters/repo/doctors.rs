use entities::Especialidad;
use sea_orm::prelude::*;

use super::prelude::*;
use crate::domain::Pagination;
use crate::domain::dto::SpecialtyId;

/// Repository trait for doctor-related database operations.
pub trait DoctorRepo {
    async fn get_doctors_by_specialty(
        &self,
        specialty_id: SpecialtyId,
        pagination: impl Into<Pagination>,
    ) -> Result<Option<(DbEspecialidad, Vec<(DbDoctor, DbUser)>)>, DbErr>;
}

impl DoctorRepo for OrmDB {
    async fn get_doctors_by_specialty(
        &self,
        specialty_id: SpecialtyId,
        pagination: impl Into<Pagination>,
    ) -> Result<Option<(DbEspecialidad, Vec<(DbDoctor, DbUser)>)>, DbErr> {
        let pagination = pagination.into();
        let results = self
            .select_paginated::<Especialidad>(pagination)
            .filter(especialidad::Column::Id.eq(specialty_id.0))
            .find_also_related(doctor::Entity)
            .order_by_asc(doctor::Column::Name)
            .all(self.connection())
            .await?;

        let Some(specialty) =
            results.first().map(|(especialidad, _)| especialidad.clone())
        else {
            return Ok(None);
        };

        let doctors: Vec<DbDoctor> =
            results.into_iter().filter_map(|(_, doctor)| doctor).collect();

        // Load user data for each doctor
        let mut doctors_with_users = Vec::new();
        for doctor in doctors {
            let user = user::Entity::find_by_id(doctor.id)
                .one(self.connection())
                .await?
                .ok_or_else(|| DbErr::RecordNotFound(format!("User not found for doctor {}", doctor.id)))?;
            doctors_with_users.push((doctor, user));
        }

        Ok(Some((specialty, doctors_with_users)))
    }
}
