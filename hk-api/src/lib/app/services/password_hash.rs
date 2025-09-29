use color_eyre::eyre::ContextCompat;

use super::prelude::*;
use crate::adapters::crypto::PasswordHasher;

pub trait PasswordHashService {
    fn hash_password(&self, password: &[u8]) -> AppResult<String>;
    fn verify_password(&self, hash: &str, password: &[u8]) -> bool;
}

impl PasswordHashService for AppState {
    fn hash_password(&self, password: &[u8]) -> AppResult<String> {
        let pass = |bytes: &[u8]| {
            const PRINT_LIMIT: usize = 32;
            let len = bytes.len();
            let bytes = &bytes[..len.min(PRINT_LIMIT)];
            let err = match str::from_utf8(&bytes) {
                Ok(s) => s.to_string(),
                Err(_) => format!("{:x?}", bytes),
            };
            if len > PRINT_LIMIT {
                format!("{}... (+{})", err, len - PRINT_LIMIT)
            } else {
                err
            }
        };
        self.hasher
            .hash_password(password)
            .wrap_err_with(|| {
                format!("Failed to hash password: {}", pass(password))
            })
            .map_err(AppError::new)
    }

    fn verify_password(&self, hash: &str, password: &[u8]) -> bool {
        self.hasher.verify_password(hash, password)
    }
}
