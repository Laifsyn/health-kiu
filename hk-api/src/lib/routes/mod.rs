mod error;
mod helpers;
mod specialty;

/// A specialized [`Result`](core::result::Result) type for route handlers.
pub type Result<T, E = ApiError> = std::result::Result<T, E>;

use error::{ApiError, ErrorKind};
pub(super) use helpers::pagination::*;

use crate::domain::internal::OutOfBoundsPagination;

impl From<OutOfBoundsPagination> for ErrorKind {
    fn from(err: OutOfBoundsPagination) -> Self {
        let OutOfBoundsPagination {} = err;
        ErrorKind::BadRequest
    }
}
