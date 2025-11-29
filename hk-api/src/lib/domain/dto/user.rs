use super::prelude::*;
use crate::domain::dto::utils::id_wrapper;
id_wrapper!(
    /// A shared ID type for both [`Doctor`][doc] and [`Patient`][patient].
    ///
    /// [doc]: crate::domain::dto::doctor::Doctor
    /// [patient]: crate::domain::dto::patient::Patient
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UserId(pub Ulid)
);

impl From<UserId> for sea_orm::Value {
    fn from(id: UserId) -> Self { sea_orm::Value::from(id.0) }
}

impl From<uuid::Uuid> for UserId {
    fn from(id: uuid::Uuid) -> Self { UserId(Ulid::from_uuid(id)) }
}
