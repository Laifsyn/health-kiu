use crate::domain::dto::UserId;

pub struct Auth {
    id: UserId,
    name: String,
    role: UserRole,
}

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

    pub fn is_patient(&self) -> bool { matches!(self, UserRole::Patient) }

    pub fn is_doctor(&self) -> bool { matches!(self, UserRole::Doctor) }
}
