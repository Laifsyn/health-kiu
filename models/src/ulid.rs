//! A wrapper around `ulid::Ulid` to be compatible with sea-orm's type
//! model.
//!
//! # Alternatives
//!
//! - Why not [uuid::Uuid]?
//!
//! [`ulid::Ulid`] is a universally unique lexicographically sortable
//! identifier. Also is capable to generate monotonically increasing ids via
//! [ulid::Ulid::increment]
use core::fmt;
use std::convert::From;
use std::ops::{Deref, DerefMut};

use sea_orm::sea_query;
use uuid::Uuid;

use crate::doctor;

/// A newtype wrapper around `ulid::Ulid` to be compatible with sea-orm's type
/// model.
#[derive(Clone, PartialEq, Eq, Copy, Hash)]
#[repr(transparent)]
pub struct Ulid(pub ulid::Ulid);

impl Ulid {
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Ulid(ulid::Ulid::from_bytes(uuid.into_bytes()))
    }

    /// Converts the ULID to a UUID.
    pub const fn as_uuid(&self) -> Uuid { Uuid::from_bytes(self.0.to_bytes()) }

    /// Generates a new ULID.
    pub fn new() -> Self { Self::default() }
}

impl Default for Ulid {
    fn default() -> Self { Self(ulid::Ulid::new()) }
}

impl fmt::Debug for Ulid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(id) = self;
        write!(f, "{id:?}")
    }
}

impl Deref for Ulid {
    type Target = ulid::Ulid;

    fn deref(&self) -> &Self::Target {
        let Self(id) = self;
        id
    }
}

impl From<Ulid> for Uuid {
    fn from(value: Ulid) -> Self { value.as_uuid() }
}

impl From<Ulid> for ulid::Ulid {
    fn from(value: Ulid) -> Self { value.0 }
}

impl DerefMut for Ulid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Self(id) = self;
        id
    }
}

impl From<Uuid> for Ulid {
    fn from(source: Uuid) -> Self { Self::from_uuid(source) }
}

impl From<Ulid> for sea_query::Value {
    fn from(source: Ulid) -> Self {
        let uuid = Uuid::from_bytes(source.to_bytes());
        sea_orm::Value::from(uuid)
    }
}

impl sea_orm::TryGetable for Ulid {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        idx: I,
    ) -> std::result::Result<Self, sea_orm::TryGetError> {
        <Uuid as sea_orm::TryGetable>::try_get_by(res, idx).map(Ulid::from_uuid)
    }
}

impl sea_orm::sea_query::ValueType for Ulid {
    fn try_from(
        v: sea_orm::Value,
    ) -> std::result::Result<Self, sea_orm::sea_query::ValueTypeErr> {
        <Uuid as sea_orm::sea_query::ValueType>::try_from(v)
            .map(Ulid::from_uuid)
    }

    fn type_name() -> std::string::String { stringify!(Ulid).to_owned() }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::Uuid
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::Uuid
    }
}

impl sea_orm::sea_query::Nullable for Ulid {
    fn null() -> sea_orm::Value {
        <Uuid as sea_orm::sea_query::Nullable>::null()
    }
}

impl fmt::Display for Ulid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(id) = self;
        write!(f, "{id}")
    }
}
