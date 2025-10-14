use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};

use crate::domain::UserRole;
use crate::routes::dto::user::UserKinds;

pub type TokenClaim = TokenData<Claim>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
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
