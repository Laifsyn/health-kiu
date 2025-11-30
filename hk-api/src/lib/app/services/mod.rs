//!
//! Services are analogous to `'users'` "use cases" or "interactions" with the
//! system.
//!
//! ## 29-sep-2025
//!
//! Basically holds what the handlers can do to the system.

pub mod appointments;
pub mod doctor;
pub mod login;
pub mod register;
pub mod specialties;

/// Convenience re-exports for [`crate::app::services`].
mod prelude {
    #![allow(unused_imports)]
    pub use crate::AppState;
    pub(crate) use crate::app::AppError;
    pub use crate::app::AppResult;
    pub use crate::domain::dto::{DoctorId, SpecialtyId, UserId};
    pub use crate::domain::{Name, Pagination};
    pub use crate::repo::prelude::*;
}
