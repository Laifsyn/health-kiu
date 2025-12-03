use std::path::{Path, PathBuf};

use dotenvy::{dotenv, dotenv_override, from_filename};

pub fn init_env() -> Option<&'static Path> {
    pub static ENV_INITIALIZED: std::sync::OnceLock<Option<PathBuf>> =
        std::sync::OnceLock::new();
    ENV_INITIALIZED
        .get_or_init(|| {
            if cfg!(debug_assertions) {
                from_filename(".env.development").ok();
            }
            dotenv_override().ok()
        })
        .as_deref()
}
