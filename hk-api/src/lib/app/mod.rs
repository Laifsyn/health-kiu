//! External User's interaction to the system.
//!
//! The [`application (self)`](self) layer abstracts how the outside world
//! (e.g., API handlers) interacts with the business logic and its adapters
//! (e.g., database or third parties APIs).

/// Re-exports for `application layer`.
pub use app_state::AppState;
pub mod app_state;
pub mod error;
/// API for interacting with the services layer.
pub mod services;

/// Type alias for `application layer's` results.
pub type AppResult<T, E = AppError> = std::result::Result<T, E>;
pub(crate) use error::AppError;

/// Convenience re-exports for Services.
pub mod services_prelude {
    pub use services::doctor::DoctorService;
    pub use services::password_hash::PasswordHashService;
    pub use services::specialties::SpecialtyService;

    use super::services;
}
