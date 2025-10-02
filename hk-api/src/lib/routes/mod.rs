//! API handlers. System's Presentation layer.
mod doctor;
mod dto;
mod error;
mod specialty;

pub use error::{ApiError, ErrorKind};

/// Type alias for standard response. It's alias to std's
/// ['Result'](core::result::Result).
type Result<T, E = ApiError> = std::result::Result<T, E>;
/// Type alias for `JSON` response. It's alias to std's
/// ['Result'](core::result::Result).
type ApiResult<T, E = ApiError> = std::result::Result<Json<T>, E>;
/// Type alias for `paginated` response. It's a specialization to [`ApiResult`].
type ApiResultPaged<T, E = ApiError> = ApiResult<dto::PagedResp<T>, E>;

use axum::{Json, Router};

use crate::AppState;
use crate::app::AppError;
use crate::domain::OutOfBoundsPagination;

/// Convenience re-exports for [`crate::routes`].
mod prelude {
    #![allow(unused_imports)]
    use std::sync::Arc;

    use axum::extract;
    pub use dto::{PagedResp, PaginatedReq};

    pub use super::*;
    pub use crate::app::AppState;
    pub use crate::app::services_prelude::*;
    pub(crate) use crate::domain::dto as domain_dto;
    pub use crate::repo::*;
    /// Convenience alias for `State<AppState>`.
    ///
    /// # Check more
    ///
    /// - [`State<T>`](axum::extract::State) : State extractor from axum.
    /// - [`AppState`](crate::app::AppState): The Server's application state.
    pub type StateApp = extract::State<crate::app::AppState>;

    /// Convenience Alias for Paginated Queries.
    ///
    /// # Check more
    /// - [`Query<T>`](axum::extract::Query) : Query extractor from axum.
    /// - [`Option<T>`](core::option::Option) : Standard library's optional
    ///   type.
    /// - [`PaginatedReq`](crate::routes::dto::PaginatedReq) : Pagination
    ///   request DTO.
    pub type MaybePaginated = extract::Query<Option<PaginatedReq>>;
}

impl From<OutOfBoundsPagination> for ErrorKind {
    fn from(err: OutOfBoundsPagination) -> Self {
        let OutOfBoundsPagination {} = err;
        ErrorKind::BadRequest
    }
}

impl From<AppError> for ApiError {
    fn from(_value: AppError) -> Self { todo!("Finish Convert implementation") }
}

pub fn router() -> Router<AppState> {
    Router::new().nest("/doctor", doctor::router())
}
