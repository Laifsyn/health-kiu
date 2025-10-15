use std::sync::Arc;

use crate::tls::{init_certificates, pem_file_path};

pub trait PasswordHasher {
    type Error;
    fn hash_password(
        &self,
        password: impl AsRef<[u8]>,
    ) -> Result<String, Self::Error>;
    fn verify_password(&self, hash: &str, password: impl AsRef<[u8]>) -> bool;
}

#[derive(Default)]
pub struct ArgonHasher {
    hasher: argon2::Argon2<'static>,
}

impl PasswordHasher for ArgonHasher {
    type Error = argon2::password_hash::Error;

    fn hash_password(
        &self,
        password: impl AsRef<[u8]>,
    ) -> Result<String, Self::Error> {
        use argon2::password_hash::rand_core::OsRng;
        use argon2::password_hash::{PasswordHasher as PH, SaltString};
        let salt = SaltString::generate(&mut OsRng);
        let password = password.as_ref();
        self.hasher.hash_password(password, &salt).map(|h| h.to_string())
    }

    fn verify_password(&self, hash: &str, password: impl AsRef<[u8]>) -> bool {
        use argon2::password_hash::{PasswordHash as PH, PasswordVerifier};

        let Ok(parsed_hash) = PH::new(hash) else {
            tracing::warn!("Failed to parse password hash: {}", hash);
            return false;
        };
        self.hasher.verify_password(password.as_ref(), &parsed_hash).is_ok()
    }
}

/// Holds the private and public keys used for signing and verifying JWTs.
pub struct AppKeys {
    pub private: Arc<[u8]>,
    pub public: Arc<[u8]>,
}

impl AppKeys {
    pub const USER_KEYS: &'static str = "./.data/jwt_users";

    /// Reads from the File System, and returns the stored keys wrappy with
    /// [`Arc`].
    pub fn new() -> Self {
        let (cert, key) = pem_file_path(Self::USER_KEYS)
            .expect("Failed to initialize jwt tokens certificates");
        let private =
            std::fs::read(&key).expect("Failed to read private key file");
        let public =
            std::fs::read(&cert).expect("Failed to read public key file");
        Self { private: Arc::from(private), public: Arc::from(public) }
    }
}
