use argon2::password_hash::rand_core::le;
use sea_orm::prelude::*;

use super::prelude::*;
use crate::domain::UserRole;

pub trait LoginRepo {
    async fn verify_patient(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<DbPatient>>;

    async fn verify_doctor(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<DbDoctor>>;
}

impl LoginRepo for OrmDB {
    async fn verify_patient(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<DbPatient>> {
        let patient = models::patient::Entity::find()
            .find_also_related(models::user::Entity)
            // Cedula is used to identify the user
            .filter(user::Column::Cedula.eq(username))
            .one(self.connection())
            .await?
            .and_then(|(patient, user)| user.and_then(|u| Some((patient, u))));
        if patient.is_none() {
            return Ok(None);
        }
        let (patient, user) = patient.expect("None checked above");
        todo!()
    }

    async fn verify_doctor(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<DbDoctor>> {
        todo!()
    }
}
