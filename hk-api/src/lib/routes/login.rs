use axum::extract::State;
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Basic;

use super::prelude::*;
use crate::routes::dto::ApiUserId;

type ApiAuthUser = ApiUserId;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login/doctor", axum::routing::post(login_doctor))
        .route("/login/patient", axum::routing::post(login_patient))
}

/// Reads the `Authorization` header with [`Basic`] scheme to login a user.
///
/// TODO: Document how [`Basic`] decodes credentials.
pub async fn login_doctor(
    TypedHeader(credentials): TypedHeader<Authorization<Basic>>,
    State(state): StateApp,
) -> ApiResult<ApiAuthUser> {
    let username = credentials.0.username();
    let password = credentials.0.password().as_bytes();

    let doctor = state.login_doctor(username, password).await?.ok_or(
        ApiError::unauthorized_user_credentials()
            .context("Invalid cedula or password"),
    )?;

    Ok(Json(ApiUserId::Doctor(doctor.id.into())))
}

pub async fn login_patient(
    TypedHeader(credentials): TypedHeader<Authorization<Basic>>,
    State(state): StateApp,
) -> ApiResult<ApiAuthUser> {
    let username = credentials.0.username();
    let password = credentials.0.password();

    let patient =
        state.login_patient(username, password.as_bytes()).await?.ok_or(
            ApiError::unauthorized_user_credentials()
                .context("Invalid cedula or password"),
        )?;

    Ok(Json(ApiUserId::Patient(patient.id.into())))
}
