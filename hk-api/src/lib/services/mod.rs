//! Http middlewares and services.

pub(crate) mod doctor;
pub mod error;
mod middleware;

/// Type alias for service results.
pub type Result<T, E = ServiceError> = std::result::Result<T, E>;
pub(crate) use error::ServiceError;

mod prelude {
    pub use crate::domain::dto::{DoctorId, SpecialtyId};
    pub use crate::domain::{Name, Pagination};
    pub use crate::repo::prelude::*;
    pub use crate::services::Result;
    pub(crate) use crate::services::ServiceError as Error;
}
