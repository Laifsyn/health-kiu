use models::Ulid;
use secrecy::SecretString;

use super::prelude::*;
use crate::domain::Name;
use crate::domain::dto::utils::id_wrapper;

id_wrapper! {
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PatientId(pub Ulid)
}

impl From<PatientId> for sea_orm::Value {
    fn from(id: PatientId) -> Self { sea_orm::Value::from(id.0) }
}

#[derive(Clone)]
pub struct Patient {
    pub id: PatientId,
    pub cedula: String,
    pub passport: Option<String>,
    pub nombre: Name,
    pub password_hash: SecretString,
}

impl Patient {
    pub fn from_models(patient: DbPatient, user: DbUser) -> Self {
        // FIXME: We should return an error instead of panicking.
        assert_eq!(
            patient.id, user.id,
            "Patient and User models must have the same id"
        );
        let DbPatient { id: _, name, password_hash } = patient;
        let DbUser { id, cedula, passport } = user;
        Patient {
            id: PatientId::from_inner(id.into()),
            cedula,
            passport,
            nombre: Name::new(name),
            password_hash: SecretString::from(password_hash),
        }
    }
}
