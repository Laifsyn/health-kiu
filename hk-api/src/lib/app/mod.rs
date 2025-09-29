//! External User's interaction to the system.
//!
//! The [`application (self)`](self) layer abstracts how the outside world
//! (e.g., API handlers) interacts with the business logic and its adapters
//! (e.g., database or third parties APIs).

pub use app_state::AppState;
pub mod app_state;
pub mod error;
mod services;

/// Type alias for app's results.
pub type Result<T, E = AppError> = std::result::Result<T, E>;
pub(crate) use error::AppError;

/// Convenience re-exports for [`crate::app`].
mod prelude {
    #![allow(unused_imports)]
    pub(crate) use crate::app::AppError as Error;
    pub use crate::app::Result;
    pub use crate::domain::dto::{DoctorId, SpecialtyId};
    pub use crate::domain::{Name, Pagination};
    pub use crate::repo::prelude::*;
}
