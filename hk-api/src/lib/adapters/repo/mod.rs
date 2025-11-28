//! This layer is responsible for interacting with the database.
//!
//!
//! It provides an abstraction over most the databases operations.
mod appointments;
mod doctors;
mod specialties;

pub use appointments::AppointmentRepo;
pub use doctors::DoctorRepo;
pub use specialties::SpecialtyRepo;

use crate::domain::Pagination;
pub trait Repository: DoctorRepo + SpecialtyRepo + AppointmentRepo {}
/// Exports repositories, and re-exports database models.
pub mod prelude {
    #![allow(unused_imports)]
    pub use models::asegurado::Model as DbAsegurado;
    pub use models::cita::Model as DbCita;
    pub use models::doctor::Model as DbDoctor;
    pub use models::doctor_especialidad::Model as DbDoctorEspecialidad;
    pub use models::especialidad::Model as DbEspecialidad;
    pub use models::habitacion::Model as DbHabitacion;
    pub use models::patient::Model as DbPatient;
    pub use models::user::Model as DbUser;
    pub use models::{prelude as entities, *};
    pub use sea_orm::QueryOrder;

    pub use super::OrmDB;
    pub use super::appointments::AppointmentRepo;
    pub use super::doctors::DoctorRepo;
    pub use super::specialties::SpecialtyRepo;
}

use sea_orm::prelude::*;

#[repr(transparent)]
#[derive(Clone)]
pub struct OrmDB(sea_orm::DatabaseConnection);

impl OrmDB {
    /// Creates a new instance of `OrmDB` from an existing `DatabaseConnection`.
    pub fn from_inner(db: sea_orm::DatabaseConnection) -> Self { Self(db) }

    /// Returns a reference to the underlying database connection.
    pub fn connection(&self) -> &sea_orm::DatabaseConnection { &self.0 }

    /// Builds a paginated select query for the given entity type `T`.
    ///
    /// ```rs, ignore
    /// 
    /// use models::especialidad;
    /// use models::prelude::Especialidad;
    /// use models::prelude::Especialidad;
    /// app.select_paginated::<Especialidad>(Pagination::new(0, 10).unwrap());
    /// ```
    pub fn select_paginated<T>(&self, pagination: Pagination) -> Select<T>
    where
        T: EntityTrait,
    {
        use std::num::NonZero;

        use sea_orm::QuerySelect;

        let (offset, limit) = pagination.tuple();
        let limit = std::cmp::max(limit.into(), 1) as u64;

        <T>::find()
            .limit(Some(limit))
            .offset(NonZero::new(offset).map(NonZero::get))
    }
}
