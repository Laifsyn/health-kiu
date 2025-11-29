use core::fmt;

use models::Ulid;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};

use crate::routes::dto;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Different kinds of users returned by the backend api.
pub enum ApiUserId {
    Patient(Ulid),
    Doctor(Ulid),
}

#[derive(serde::Serialize)]
#[serde(tag = "type", content = "content")]
/// Different kinds of users returned by the backend api.
pub enum ApiUser {
    Doctor(dto::ApiDoctor),
    Patient(dto::ApiPatient),
}

impl fmt::Display for ApiUserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiUserId::Patient(id) => write!(f, "patient:{id}"),
            ApiUserId::Doctor(id) => write!(f, "doctor:{id}"),
        }
    }
}

impl<'de> Deserialize<'de> for ApiUserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (variant, id) = <&str>::deserialize(deserializer)?
            .split_once(':')
            .ok_or(serde::de::Error::custom(
                "invalid user kind format, expected `<kind>:<id>`",
            ))?;

        // Defer id parsing to after variant matching
        let id = move || Ulid::try_from(id).map_err(serde::de::Error::custom);

        match (variant) {
            ("patient") => Ok(ApiUserId::Patient(id()?)),
            ("doctor") => Ok(ApiUserId::Doctor(id()?)),
            _ => Err(serde::de::Error::custom("unknown user kind")),
        }
    }
}

impl Serialize for ApiUserId {
    /// Manually implement serialization because [`Ulid`] does not implement
    /// `Serialize`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        let mut buf = [0u8; Ulid::BUFFER_LEN];
        match self {
            ApiUserId::Patient(id) => {
                map.serialize_entry("type", "patient")?;
                map.serialize_entry("id", id.as_str(&mut buf))?;
            }
            ApiUserId::Doctor(id) => {
                map.serialize_entry("type", "doctor")?;
                map.serialize_entry("id", id.as_str(&mut buf))?;
            }
        }
        map.end()
    }
}
