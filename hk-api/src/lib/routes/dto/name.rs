use serde::Serialize;

use crate::domain;

#[derive(Debug, Serialize, Clone)]
/// Structured name representation based on [VCard's N][vcard-n-field] field.
///
/// Breaks down a full name into its individual components:
/// `{prefix} {given} {middle} {family} {suffix}`.
///
/// [vcard-n-field]: https://en.wikipedia.org/wiki/VCard#Properties
pub struct ApiName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// Full name reconstructed from the individual components.
    pub full_name: String,
}

impl From<domain::Name> for ApiName {
    fn from(name: domain::Name) -> Self {
        let family = name.family_name().map(Into::into);
        let given = name.given_name().map(Into::into);
        let middle = name.additional_names().map(Into::into);
        let prefix = name.honorific_prefixes().map(Into::into);
        let suffix = name.honorific_suffixes().map(Into::into);
        let full_name = name.full_name();

        Self { prefix, given, middle, family, suffix, full_name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain;

    #[test]
    fn test_json_serialization_complete_name() {
        let domain_name =
            domain::Name::new("Doe;John;Michael;Dr.;Jr.".to_string());
        let dto_name = ApiName::from(domain_name);

        let json = serde_json::to_string(&dto_name).unwrap();
        let expected = r#"{"prefix":"Dr.","given":"John","middle":"Michael","family":"Doe","suffix":"Jr.","full_name":"Dr. John Michael Doe, Jr."}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_serialization_partial_name() {
        let domain_name = domain::Name::new("Holland;Tom".to_string());
        let dto_name = ApiName::from(domain_name);

        let json = serde_json::to_string(&dto_name).unwrap();
        let expected =
            r#"{"given":"Tom","family":"Holland","full_name":"Tom Holland"}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_serialization_single_name() {
        let domain_name = domain::Name::new("Einstein".to_string());
        let dto_name = ApiName::from(domain_name);

        let json = serde_json::to_string(&dto_name).unwrap();
        let expected = r#"{"family":"Einstein","full_name":"Einstein"}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_serialization_with_suffix_only() {
        let domain_name = domain::Name::new("Brown;Alice;;;PhD".to_string());
        let dto_name = ApiName::from(domain_name);

        let json = serde_json::to_string(&dto_name).unwrap();
        let expected = r#"{"given":"Alice","family":"Brown","suffix":"PhD","full_name":"Alice Brown, PhD"}"#;

        assert_eq!(json, expected);
    }
}
