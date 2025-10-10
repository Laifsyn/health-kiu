use color_eyre::eyre::{self, ContextCompat, eyre};

use super::prelude::*;
use crate::adapters::crypto::PasswordHasher;

pub trait PasswordHashService {
    fn hash_password(&self, password: &[u8]) -> AppResult<String>;
    fn verify_password(&self, hash: &str, password: &[u8]) -> bool;
}

impl PasswordHashService for AppState {
    fn hash_password(&self, password: &[u8]) -> AppResult<String> {
        self.hasher
            .hash_password(password)
            .map_err(|e| {
                // FIXME: We shouldn't log the password source
                eyre!(
                    "Failed to hash password (source: {}): {}",
                    password_source_display(password),
                    e
                )
            })
            .map_err(AppError::from)
    }

    fn verify_password(&self, hash: &str, password: &[u8]) -> bool {
        self.hasher.verify_password(hash, password)
    }
}

/// Parse the given bytes as a UTF-8 string, or as a hex dump if not valid
/// UTF-8.
fn password_source_display(bytes: &[u8]) -> String {
    const PRINT_LIMIT: usize = 32;
    let len = bytes.len();
    let bytes = &bytes[..len.min(PRINT_LIMIT)];
    let err = match str::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(_) => format!("{:x?}", bytes),
    };
    if len > PRINT_LIMIT {
        format!("{}... (+{})", err, len - PRINT_LIMIT)
    } else {
        err
    }
}
