use core::fmt;

use models::Ulid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Different kinds of users in the system.
pub enum UserKinds {
    Patient(Ulid),
    Doctor(Ulid),
}

impl fmt::Display for UserKinds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserKinds::Patient(id) => write!(f, "patient:{id}"),
            UserKinds::Doctor(id) => write!(f, "doctor:{id}"),
        }
    }
}

impl<'de> Deserialize<'de> for UserKinds {
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
            ("patient") => Ok(UserKinds::Patient(id()?)),
            ("doctor") => Ok(UserKinds::Doctor(id()?)),
            _ => Err(serde::de::Error::custom("unknown user kind")),
        }
    }
}

impl Serialize for UserKinds {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
