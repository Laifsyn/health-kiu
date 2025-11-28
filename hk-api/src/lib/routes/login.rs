use axum::extract::State;
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Basic;

#[derive(serde::Serialize)]
#[serde(tag = "type", content = "content")]
pub enum UserKind {
    Doctor(dto::ApiDoctor),
    Patient(dto::ApiPatient),
}

type UserLoginResp = UserKind;

use super::prelude::*;
#[allow(unused)]
pub async fn login(
    TypedHeader(credentials): TypedHeader<Authorization<Basic>>,
    State(state): StateApp,
) -> ApiResult<UserLoginResp> {
    let username = credentials.0.username();
    let password = credentials.0.password().as_bytes();

    let doctor_opt = state.login_doctor(username, password).await?;

    if let Some(doctor) = doctor_opt {
        return Ok(Json(UserKind::Doctor(doctor.into())));
    }

    let patient_opt = state.login_patient(username, password).await?;

    if let Some(patient) = patient_opt {
        return Ok(Json(UserKind::Patient(patient.into())));
    }
    let invalid_credentials =
        ApiError::unauthorized().context("Invalid username or password");
    Err(invalid_credentials)
}
