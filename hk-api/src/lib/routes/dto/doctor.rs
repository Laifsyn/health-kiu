use sea_orm::prelude::Uuid;
use serde::Serialize;

use super::prelude::*;
#[derive(Serialize)]
/// A Doctor object returned by the API.
pub struct ApiDoctor {
    /// The unique identifier of the doctor.
    pub id: Uuid,
    pub cedula: String,
    pub passport: Option<String>,
    pub name: ApiName,
}

impl From<Doctor> for ApiDoctor {
    fn from(doctor: Doctor) -> Self {
        let Doctor { id, cedula, passport, nombre, password_hash: _ } = doctor;
        let id = id.0.as_uuid();
        let name = ApiName::from(nombre);

        Self { id, cedula, passport, name }
    }
}
