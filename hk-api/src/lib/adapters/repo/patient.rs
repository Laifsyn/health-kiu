use sea_orm::prelude::*;

use super::prelude::*;
pub type PatientUser = (DbPatient, DbUser);

pub trait PatientRepo {
    async fn get_patient(
        &self,
        id: impl Into<Ulid>,
    ) -> Result<Option<PatientUser>>;

    async fn get_patient_by_cedula(
        &self,
        cedula: &str,
    ) -> Result<Option<PatientUser>>;
}

impl PatientRepo for OrmDB {
    async fn get_patient(
        &self,
        id: impl Into<Ulid>,
    ) -> Result<Option<PatientUser>> {
        let patient: Option<(DbPatient, Option<DbUser>)> =
            models::patient::Entity::find()
                .find_also_related(models::user::Entity)
                .filter(models::patient::Column::Id.eq(id.into()))
                .one(self.connection())
                .await?;

        Ok(patient.and_then(|(p, u)| u.map(|user_model| (p, user_model))))
    }

    async fn get_patient_by_cedula(
        &self,
        cedula: &str,
    ) -> Result<Option<PatientUser>> {
        let patient: Option<(DbPatient, Option<DbUser>)> =
            models::patient::Entity::find()
                .find_also_related(models::user::Entity)
                .filter(user::Column::Cedula.eq(cedula))
                .one(self.connection())
                .await?;

        Ok(patient.and_then(|(p, u)| u.map(|user_model| (p, user_model))))
    }
}
