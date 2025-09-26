use axum::extract::{Path, Query, State};
use itertools::Itertools;

use super::prelude::*;

/// Expects "doctors/{id}/by_specialty"
pub async fn doctors_get_by_specialty(
    Path(specialty_id): Path<i16>,
    pagination: MaybePaginated,
    State(app): StateApp,
) -> ResultPaged<dto::Doctor> {
    let specialty_id = domain_dto::SpecialtyId::try_from_inner(specialty_id)
        .map_err(ApiError::bad_request)?;
    let pagination = pagination.0.unwrap_or_default();

    let result =
        app.get_doctors_by_specialty(specialty_id, pagination.clone()).await?;

    match result {
        Some((_, doctors)) => {
            todo!()
        }
        None => {
            Err(ApiError::new_with_context(
                ErrorKind::NotFound,
                "Specialty not found",
            ))
        }
    }
}
