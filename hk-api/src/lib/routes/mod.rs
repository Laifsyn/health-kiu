//! Service's route handlers and related DTO.
pub mod dto;
mod error;
mod specialty;

/// A specialized [`Result`](core::result::Result) type for route handlers.
pub type Result<T, E = ApiError> = std::result::Result<T, E>;

pub use error::{ApiError, ErrorKind};

use crate::domain::OutOfBoundsPagination;

impl From<OutOfBoundsPagination> for ErrorKind {
    fn from(err: OutOfBoundsPagination) -> Self {
        let OutOfBoundsPagination {} = err;
        ErrorKind::BadRequest
    }
}
