pub use super::prelude::*;
use crate::domain::dto::DoctorId;
use crate::domain::dto::utils::id_wrapper;

id_wrapper!(
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UserId(pub Ulid)
);

impl From<UserId> for sea_orm::Value {
    fn from(id: UserId) -> Self { sea_orm::Value::from(id.0) }
}

impl From<DoctorId> for UserId {
    fn from(doctor_id: DoctorId) -> Self { Self(doctor_id.into_inner()) }
}

impl From<UserId> for DoctorId {
    fn from(user_id: UserId) -> Self { Self(user_id.into_inner()) }
}
