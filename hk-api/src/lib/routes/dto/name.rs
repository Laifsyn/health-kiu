use std::borrow::Cow;

use serde::Serialize;

use crate::domain;

#[derive(Debug, Serialize, Clone)]
/// Structured name representation based on [VCard's N][vcard-n-field] field.
///
/// Breaks down a full name into its individual components:
/// `{prefix} {given} {middle} {family} {suffix}`.
///
/// [vcard-n-field]: https://en.wikipedia.org/wiki/VCard#Properties
pub struct Name<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<&'a str>,
    /// Full name reconstructed from the individual components.
    pub full_name: Cow<'a, str>,
}

impl<'a> From<&'a domain::Name> for Name<'a> {
    fn from(name: &'a domain::Name) -> Self {
        let family = name.family_name();
        let given = name.given_name();
        let middle = name.additional_names();
        let prefix = name.honorific_prefixes();
        let suffix = name.honorific_suffixes();
        let full_name = name.full_name().into();

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
        let dto_name = Name::from(&domain_name);

        let json = serde_json::to_string(&dto_name).unwrap();
        let expected = r#"{"prefix":"Dr.","given":"John","middle":"Michael","family":"Doe","suffix":"Jr.","full_name":"Dr. John Michael Doe, Jr."}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_serialization_partial_name() {
        let domain_name = domain::Name::new("Holland;Tom".to_string());
        let dto_name = Name::from(&domain_name);

        let json = serde_json::to_string(&dto_name).unwrap();
        let expected =
            r#"{"given":"Tom","family":"Holland","full_name":"Tom Holland"}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_serialization_single_name() {
        let domain_name = domain::Name::new("Einstein".to_string());
        let dto_name = Name::from(&domain_name);

        let json = serde_json::to_string(&dto_name).unwrap();
        let expected = r#"{"family":"Einstein","full_name":"Einstein"}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_serialization_with_suffix_only() {
        let domain_name = domain::Name::new("Brown;Alice;;;PhD".to_string());
        let dto_name = Name::from(&domain_name);

        let json = serde_json::to_string(&dto_name).unwrap();
        let expected = r#"{"given":"Alice","family":"Brown","suffix":"PhD","full_name":"Alice Brown, PhD"}"#;

        assert_eq!(json, expected);
    }
}
