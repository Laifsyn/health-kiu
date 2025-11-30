use super::prelude::*;
use crate::adapters::crypto::PasswordHasher;
use crate::domain::dto::doctor::Doctor;
use crate::domain::dto::patient::Patient;

pub trait LoginService {
    async fn login_doctor(
        &self,
        username: &str,
        password: &[u8],
    ) -> AppResult<Option<Doctor>>;

    async fn login_patient(
        &self,
        username: &str,
        password: &[u8],
    ) -> AppResult<Option<Patient>>;

    fn verify_password(&self, hash: &str, password: &[u8]) -> bool;
}

impl LoginService for AppState {
    fn verify_password(&self, hash: &str, password: &[u8]) -> bool {
        self.hasher.verify_password(hash, password)
    }

    async fn login_doctor(
        &self,
        username: &str,
        password: &[u8],
    ) -> AppResult<Option<Doctor>> {
        let fetched_users = self.db.get_doctor_by_cedula(username).await?;
        let Some(fetched_users) = fetched_users else {
            return Ok(None);
        };
        let (doctor, _user) = &fetched_users;

        let verified = self.verify_password(&doctor.password_hash, password);
        if !verified {
            return Err(AppError::invalid_password());
        }
        Ok(Some(fetched_users.into()))
    }

    async fn login_patient(
        &self,
        username: &str,
        password: &[u8],
    ) -> AppResult<Option<Patient>> {
        let fetched_users = self.db.get_patient_by_cedula(username).await?;
        let Some(fetched_users) = fetched_users else {
            return Ok(None);
        };
        let (patient, _user) = &fetched_users;

        let verified = self.verify_password(&patient.password_hash, password);
        if !verified {
            return Err(AppError::invalid_password());
        }
        Ok(Some(fetched_users.into()))
    }
}

// fn hash_password(this: &AppState, password: &[u8]) -> Result<String> {
//     this.hasher
//         .hash_password(password)
//         .map_err(|e| {
//             // FIXME: We shouldn't log the password source
//             eyre!(
//                 "Failed to hash password (source: {}): {}",
//                 password_source_display(password),
//                 e
//             )
//         })
//         .map_err(AppError::from)
// }

// /// Parse the given bytes as a UTF-8 string, or as a hex dump if not valid
// /// UTF-8.
// fn password_source_display(bytes: &[u8]) -> String {
//     const PRINT_LIMIT: usize = 32;
//     let len = bytes.len();
//     let bytes = &bytes[..len.min(PRINT_LIMIT)];
//     let err = match str::from_utf8(bytes) {
//         Ok(s) => s.to_string(),
//         Err(_) => format!("{:x?}", bytes),
//     };
//     if len > PRINT_LIMIT {
//         format!("{}... (+{})", err, len - PRINT_LIMIT)
//     } else {
//         err
//     }
// }
