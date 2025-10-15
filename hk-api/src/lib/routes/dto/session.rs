use std::marker::PhantomData;

use argon2::PasswordHash;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use models::Ulid;
use secrecy::SecretString;

use crate::AppState;
use crate::domain::UserRole;
use crate::routes::ApiError;
use crate::routes::dto::token::UserToken;
use crate::routes::dto::user::UserKinds;

pub struct Session<T> {
    user_id: Ulid,
    _role: PhantomData<T>,
}

enum TokenError {
    Missing,
    Invalid,
    Expired,
}

impl From<TokenError> for ApiError {
    fn from(value: TokenError) -> Self {
        match value {
            TokenError::Missing => {
                ApiError::unauthorized().context("Missing authentication token")
            }
            TokenError::Invalid => {
                ApiError::unauthorized().context("Invalid authentication token")
            }
            TokenError::Expired => {
                ApiError::unauthorized().context("Expired authentication token")
            }
        }
    }
}

pub type DoctorSession = Session<Doctor>;
pub type PatientSession = Session<Patient>;

pub struct Doctor;
pub struct Patient;

impl FromRequestParts<AppState> for UserToken {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // get cookies
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .expect("extracting cookies should be infallible");

        let token = jar.get("token").ok_or(TokenError::Missing)?;
        let token_data = UserToken::decode(token.value(), state.public_key())
            .map_err(|_| TokenError::Invalid)?;
        Ok(token_data)
    }
}

impl FromRequestParts<AppState> for DoctorSession {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = UserToken::from_request_parts(parts, state).await?;

        // check role
        match &token.user_kind() {
            UserKinds::Doctor(id) => {
                Ok(Self { user_id: *id, _role: PhantomData })
            }
            _ => {
                Err(ApiError::unauthorized()
                    .context("User does not have doctor permissions"))
            }
        }
    }
}

impl FromRequestParts<AppState> for PatientSession {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = UserToken::from_request_parts(parts, state).await?;

        // check role
        match &token.user_kind() {
            UserKinds::Patient(id) => {
                Ok(Self { user_id: *id, _role: PhantomData })
            }
            _ => {
                Err(ApiError::unauthorized()
                    .context("User does not have patient permissions"))
            }
        }
    }
}
