use strum_macros::{EnumIs, EnumString};

use crate::domain::dto::UserId;

/// Represents authenticated user information.
pub struct Auth {
    pub(crate) id: UserId,
    pub(crate) name: String,
    pub(crate) role: UserRole,
}

#[derive(Clone, PartialEq, Eq, EnumString, EnumIs)]
#[strum(serialize_all = "lowercase")]
pub enum UserRole {
    Patient,
    Doctor,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Patient => "patient",
            UserRole::Doctor => "doctor",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "patient" => Some(UserRole::Patient),
            "doctor" => Some(UserRole::Doctor),
            _ => None,
        }
    }
}
