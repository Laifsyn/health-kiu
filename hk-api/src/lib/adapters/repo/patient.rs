use sea_orm::prelude::*;

use super::prelude::*;
pub type PatientUser = (DbPatient, DbUser);

pub trait PatientRepo {
    async fn get_patient(
        &self,
        id: impl Into<Ulid>,
    ) -> AppResult<Option<PatientUser>>;

    async fn get_patient_by_cedula(
        &self,
        cedula: &str,
    ) -> AppResult<Option<PatientUser>>;

    async fn register_patient(
        &self,
        payload: RegisterPatientPayload,
    ) -> AppResult<PatientUser>;
}

impl PatientRepo for OrmDB {
    async fn get_patient(
        &self,
        id: impl Into<Ulid>,
    ) -> AppResult<Option<PatientUser>> {
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
    ) -> AppResult<Option<PatientUser>> {
        let patient: Option<(DbPatient, Option<DbUser>)> =
            models::patient::Entity::find()
                .find_also_related(models::user::Entity)
                .filter(user::Column::Cedula.eq(cedula))
                .one(self.connection())
                .await?;

        Ok(patient.and_then(|(p, u)| u.map(|user_model| (p, user_model))))
    }

    async fn register_patient(
        &self,
        payload: RegisterPatientPayload,
    ) -> AppResult<PatientUser> {
        use sea_orm::ActiveValue::Set;
        let RegisterPatientPayload {
            name,
            password_hash,
            passport,
            cedula,
            id,
        } = payload;

        let patient_id: Uuid = id.unwrap_or_default().as_uuid();

        let user_active_model = user::ActiveModel {
            id: Set(patient_id),
            passport: Set(passport),
            cedula: Set(cedula),
        };

        let patient_active_model = patient::ActiveModel {
            id: Set(patient_id),
            name: Set(name.to_string()),
            password_hash: Set(password_hash.to_string()),
        };

        let txn = self.connection().begin().await?;

        let user_model = user_active_model.insert(&txn).await?;
        let patient_model = patient_active_model.insert(&txn).await?;

        txn.commit().await?;

        Ok((patient_model, user_model))
    }
}
