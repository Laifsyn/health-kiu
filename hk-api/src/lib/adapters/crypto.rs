pub trait PasswordHasher {
    fn hash_password(&self, password: &[u8]) -> Option<String>;
    fn verify_password(&self, hash: &str, password: &[u8]) -> bool;
}

pub struct ArgonHasher {
    hasher: argon2::Argon2<'static>,
}

impl PasswordHasher for ArgonHasher {
    fn hash_password(&self, password: &[u8]) -> Option<String> {
        use argon2::password_hash::rand_core::OsRng;
        use argon2::password_hash::{PasswordHasher as PH, SaltString};

        let salt = SaltString::generate(&mut OsRng);
        self.hasher.hash_password(password, &salt).ok().map(|h| h.to_string())
    }

    fn verify_password(&self, hash: &str, password: &[u8]) -> bool {
        use argon2::password_hash::{PasswordHash as PH, PasswordVerifier};

        let Ok(parsed_hash) = PH::new(hash) else {
            tracing::warn!("Failed to parse password hash: {}", hash);
            return false;
        };
        self.hasher.verify_password(password, &parsed_hash).is_ok()
    }
}

impl Default for ArgonHasher {
    fn default() -> Self {
        // FIXME: Default Argon2 hasher might not be what we want.
        Self { hasher: argon2::Argon2::default() }
    }
}
