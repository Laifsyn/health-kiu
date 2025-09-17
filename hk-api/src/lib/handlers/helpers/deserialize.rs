use serde::Deserializer;
use serde::de::DeserializeOwned;

/// Deserializes into [`Option<T>`] where `T` implements Deserialize.
/// Returns None on any deserialization error.
pub fn deserialize_optional<'de, T, D>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    T::deserialize(deserializer).map(Some).or_else(|_| Ok(None))
}
