use axum::extract::{Path, Query, State};
use axum::routing::get;
use either::Either;
use serde::Serialize;
use uuid::Uuid;

use super::prelude::*;
use crate::routes::dto::{ApiDoctor, ApiSpecialty};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/by_specialty/{specialty_id}", get(doctors_by_specialty))
        .route("/", get(get_doctor_paginated))
        .route("/{doctor_id}", get(get_doctor))
}

#[derive(Serialize)]
/// Describes a specialty and its associated doctors.
pub struct DoctorsInSpecialty {
    specialty: ApiSpecialty,
    /// Doctors that have the specified specialty.
    #[serde(flatten)]
    doctors: PagedResp<ApiDoctor>,
}

/// Requests doctors that have a given specialty.
pub async fn doctors_by_specialty(
    Path(specialty_id): Path<i16>,
    Query(pagination): MaybePaginated,
    State(app): StateApp,
) -> ApiResult<DoctorsInSpecialty> {
    let specialty_id = domain_dto::SpecialtyId::try_from_inner(specialty_id)
        .map_err(ApiError::bad_request)?;
    let pagination = pagination.unwrap_or_default();

    let (specialty, doctors) = app
        .get_doctors_by_specialty(specialty_id.clone(), pagination)
        .await?
        .ok_or_else(|| ApiError::not_found(specialty_id.0))?;

    let doctors =
        PagedResp::from_paged_with_transform(doctors, ApiDoctor::from);
    let specialty = ApiSpecialty::from(specialty);

    let response = DoctorsInSpecialty { specialty, doctors };
    Ok(Json(response))
}

pub async fn get_doctor_paginated(
    Query(pagination): MaybePaginated,
    State(app): StateApp,
) -> ApiResultPaged<ApiDoctor> {
    let pagination = pagination.unwrap_or_default();
    let doctors = app
        .get_doctors(pagination)
        .await
        .map(|e| PagedResp::from_paged_with_transform(e, ApiDoctor::from))?;

    Ok(doctors.json())
}

pub async fn get_doctor(
    _id: Path<Uuid>,
) -> ApiResult<Either<ApiDoctor, PagedResp<ApiDoctor>>> {
    todo!()
}
