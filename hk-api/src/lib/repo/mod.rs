use std::num::NonZero;

use models::especialidad;
use models::prelude::*;
use sea_orm::{ConnectionTrait, EntityTrait, QueryOrder, QuerySelect};
pub trait Repository {
    type DbConn: ConnectionTrait;

    fn connection(&self) -> &Self::DbConn;

    async fn get_paginated_specialties(
        &self,
        conn: &Self::DbConn,
        offset: Option<NonZero<u64>>,
        limit: u16,
    ) -> Result<Vec<especialidad::Model>, sea_orm::DbErr> {
        let specialties = Especialidad::find()
            .order_by_asc(especialidad::Column::Id)
            .limit(Some(limit as u64))
            .offset(offset.map(NonZero::get))
            .all(conn)
            .await?;

        Ok(specialties)
    }
}
