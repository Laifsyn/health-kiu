use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};

use crate::domain::UserRole;
use crate::routes::ApiError;
use crate::routes::dto::user::UserKinds;

#[repr(transparent)]
/// Token used to identify a system's user
pub struct UserToken(pub TokenData<UserClaim>);

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaim {
    iss: String, // Optional. Issuer
    /// Expiration time (as UTC timestamp)
    exp: usize,
    /// Issued at (as UTC timestamp)
    iat: usize,
    /// Not Before (as UTC timestamp)
    nbf: usize,
    /// Subject (whom token refers to)
    sub: UserKinds,
}

impl UserToken {
    /// Algorithm used for signing the token
    const ALGORITHM: jsonwebtoken::Algorithm = jsonwebtoken::Algorithm::ES384;
    /// Token expiration time in seconds
    const EXPIRATION: usize = 1800;

    // seconds
    pub fn decode(token: &str, public_key: &[u8]) -> Result<Self, ApiError> {
        // FIXME: We could cache this `DecodingKey`
        let decoding_key = jsonwebtoken::DecodingKey::from_ec_pem(public_key)
            .map_err(ApiError::internal)?;
        let validation = jsonwebtoken::Validation::new(Self::ALGORITHM);

        jsonwebtoken::decode(token, &decoding_key, &validation)
            .map(Self)
            .map_err(ApiError::internal)
    }

    pub fn encode(&self, private_key: &[u8]) -> Result<String, ApiError> {
        // FIXME: We could cache this `EncodingKey`
        let encoding_key = jsonwebtoken::EncodingKey::from_ec_pem(private_key)
            .map_err(ApiError::internal)?;
        jsonwebtoken::encode(&self.0.header, &self.0.claims, &encoding_key)
            .map_err(ApiError::internal)
    }

    pub fn new(user: UserKinds) -> Self {
        let now = chrono::Utc::now().timestamp() as usize;
        let claim = UserClaim {
            iss: "health-kiu".to_string(),
            exp: now + Self::EXPIRATION,
            iat: now,
            nbf: now,
            sub: user,
        };
        Self(TokenData {
            header: jsonwebtoken::Header::new(Self::ALGORITHM),
            claims: claim,
        })
    }

    pub fn user_kind(&self) -> &UserKinds { &self.0.claims.sub }
}
