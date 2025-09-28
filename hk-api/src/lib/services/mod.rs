//! External User's interaction to the system.
//!
//! The [`services (self)`](self) layer abstracts how the outside world (e.g.,
//! API handlers) interacts with the system's core logic (e.g., repositories).

pub mod error;
/// Type alias for service results.
pub type Result<T, E = ServiceError> = std::result::Result<T, E>;
pub(crate) use error::ServiceError;

/// Convenience re-exports for [`crate::services`].
mod prelude {
    #![allow(unused_imports)]
    pub use crate::domain::dto::{DoctorId, SpecialtyId};
    pub use crate::domain::{Name, Pagination};
    pub use crate::repo::prelude::*;
    pub use crate::services::Result;
    pub(crate) use crate::services::ServiceError as Error;
}
