use super::prelude::*;
use crate::adapters::crypto::PasswordHasher;
use crate::domain::dto::patient::Patient;
use crate::domain::{Auth, UserRole};
use crate::repo::dto::{RegisterDbDoctor, RegisterPatientPayload};

/// Domain's data required to register a new user.
pub struct RegisterData<'a> {
    pub(crate) role: UserRole,
    pub(crate) cedula: String,
    pub(crate) passport: Option<String>,
    pub(crate) name: String,
    pub(crate) password: &'a [u8],
}

pub trait RegisterService {
    async fn register(&self, data: RegisterData<'_>) -> Result<Auth>;
}

impl RegisterService for AppState {
    async fn register(&self, data: RegisterData<'_>) -> Result<Auth> {
        let RegisterData { role, cedula, name, password, passport } = data;

        match role {
            UserRole::Patient => {
                let password_hash = self.hasher.hash_password(password)?;
                let payload = RegisterPatientPayload {
                    name,
                    password_hash,
                    passport,
                    cedula,
                    id: None,
                };

                let (patient, _user) =
                    self.db.register_patient(payload).await?;
                let auth = Auth {
                    id: patient.id.into(),
                    name: patient.name,
                    role: UserRole::Patient,
                };
                Ok(auth)
            }
            UserRole::Doctor => {
                let password_hash = self.hasher.hash_password(password)?;
                let payload = RegisterDbDoctor {
                    name,
                    password_hash,
                    passport,
                    cedula,
                    id: None,
                };

                let (doctor, _user) = self.db.register_doctor(payload).await?;
                let auth = Auth {
                    id: doctor.id.into(),
                    name: doctor.name,
                    role: UserRole::Doctor,
                };
                Ok(auth)
            }
        }
    }
}
