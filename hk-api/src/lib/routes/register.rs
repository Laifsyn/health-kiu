use core::fmt;

use axum::extract::Query;
use axum::routing::post;
use tracing::instrument;

use super::prelude::*;
use crate::app::services::register::{RegisterData, RegisterService};
use crate::domain::UserRole;
use crate::routes::dto::ApiUserId;

#[derive(serde::Deserialize)]
pub struct RegisterRequest {
    cedula: Option<String>,
    password: String,
    name: String,
    passport: Option<String>,
}

impl fmt::Debug for RegisterRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisterRequest")
            .field("cedula", &self.cedula)
            .field("name", &self.name)
            .field("passport", &self.passport)
            .field("password", &"****")
            .finish()
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/patient", post(register_patient))
        .route("/doctor", post(register_doctor))
}

/// Registers a patient user, and returns the user's id.
#[instrument(skip(state), name = "Register Patient", level = "debug")]
pub async fn register_patient(
    Query(request): Query<RegisterRequest>,
    State(state): StateApp,
) -> ApiResult<ApiUserId> {
    let RegisterRequest { cedula, passport, name, password } = request;
    // Ensure at least one of cedula or passport is provided
    let (cedula, passport) = match (cedula, passport) {
        (None, None) => {
            Err(ApiError::bad_request(
                "Either 'cedula' or 'passport' must be provided",
            ))
        }
        (None, Some(passport)) => Ok((passport.clone(), Some(passport))),
        (Some(cedula), None) => Ok((cedula, None)),
        (Some(cedula), Some(p)) => Ok((cedula, Some(p))),
    }?;

    let password_bytes = password.as_bytes();
    let data = RegisterData {
        role: UserRole::Patient,
        cedula,
        passport,
        name,
        password: password_bytes,
    };

    let auth = state.register(data).await?;
    assert!(matches!(auth.role, UserRole::Patient));
    Ok(Json(ApiUserId::Patient(auth.id.into())))
}

/// Registers a doctor user, and returns the user's id.
#[instrument(skip(state), name = "Register Doctor", level = "debug")]
async fn register_doctor(
    Query(request): Query<RegisterRequest>,
    State(state): StateApp,
) -> ApiResult<ApiUserId> {
    let RegisterRequest { cedula, passport, name, password } = request;
    // Ensure at least one of cedula or passport is provided
    let (cedula, passport) = match (cedula, passport) {
        (None, None) => {
            Err(ApiError::bad_request(
                "Either 'cedula' or 'passport' must be provided",
            ))
        }
        (None, Some(passport)) => Ok((passport.clone(), Some(passport))),
        (Some(cedula), None) => Ok((cedula, None)),
        (Some(cedula), Some(p)) => Ok((cedula, Some(p))),
    }?;

    let password_bytes = password.as_bytes();
    let data = RegisterData {
        role: UserRole::Doctor,
        cedula,
        passport,
        name,
        password: password_bytes,
    };

    let auth = state.register(data).await?;
    assert!(matches!(auth.role, UserRole::Doctor));
    Ok(Json(ApiUserId::Doctor(auth.id.into())))
}
