use std::num::NonZero;

use models::prelude::*;
use models::{doctor, especialidad};
use sea_orm::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};

use crate::ServerApp;
use crate::domain::dto::{DoctorId, SpecialtyId};
use crate::domain::*;
use crate::repo::prelude::*;

impl ServerApp {
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
        // Destructure pagination parameters
        let (offset, limit) = pagination.tuple();
        let limit = std::cmp::max(limit.into(), 1) as u64;

        <T>::find()
            .limit(Some(limit))
            .offset(NonZero::new(offset).map(NonZero::get))
    }
}

impl DoctorRepo for ServerApp {
    async fn get_doctors_by_specialty(
        &self,
        specialty_id: SpecialtyId,
        pagination: impl Into<Pagination>,
    ) -> Result<Option<(DbEspecialidad, Vec<DbDoctor>)>, DbErr> {
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

        Ok(Some((specialty, doctors)))
    }
}
