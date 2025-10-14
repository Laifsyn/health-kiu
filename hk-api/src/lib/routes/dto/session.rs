use std::marker::PhantomData;

use argon2::PasswordHash;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use models::Ulid;
use secrecy::SecretString;

use crate::AppState;
use crate::routes::ApiError;

pub struct Session<T> {
    user_id: Ulid,
    user_name: String,
    _role: PhantomData<T>,
}

enum AuthError {
    MissingToken,
    InvalidToken,
    ExpiredToken,
}

impl From<AuthError> for ApiError {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::MissingToken => {
                ApiError::unauthorized().context("Missing authentication token")
            }
            AuthError::InvalidToken => {
                ApiError::unauthorized().context("Invalid authentication token")
            }
            AuthError::ExpiredToken => {
                ApiError::unauthorized().context("Expired authentication token")
            }
        }
    }
}

pub type DoctorSession = Session<Doctor>;
pub type PatientSession = Session<Patient>;

pub struct Doctor;
pub struct Patient;

impl<R> FromRequestParts<AppState> for Session<R> {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // get cookies
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .expect("extracting cookies should be infallible");
        jar.get("token").ok_or(AuthError::MissingToken)?;
    }
}
