/// String representation of [VCard's N][vcard-n-field] field.
///
/// It's described as
///
/// `{Family Name};{Given Name};{Additional Names};{Honorific
/// Prefixes};{Honorific Suffixes}`.
///
/// [vcard-n-field]: https://en.wikipedia.org/wiki/VCard#Properties
#[derive(Clone)]
pub struct Name(String);

impl Name {
    pub fn new(name: String) -> Self {
        let parts: Vec<_> = name
            .split(';')
            .map(str::trim)
            .chain(std::iter::repeat(""))
            .take(5)
            .collect();

        Self(parts.join(";"))
    }

    pub fn as_slice(&self) -> [&str; 5] {
        let mut parts = [""; 5];
        for (i, part) in self.0.split(';').take(5).enumerate() {
            // Skip trimming because `Self::new` trims on instantiation.
            parts[i] = part;
        }
        parts
    }

    pub fn as_raw(&self) -> &str { &self.0 }

    pub fn family_name(&self) -> Option<&str> {
        let field = self.as_slice()[0];
        if field.is_empty() { None } else { Some(field) }
    }

    pub fn given_name(&self) -> Option<&str> {
        let field = self.as_slice()[1];
        if field.is_empty() { None } else { Some(field) }
    }

    pub fn additional_names(&self) -> Option<&str> {
        let field = self.as_slice()[2];
        if field.is_empty() { None } else { Some(field) }
    }

    pub fn honorific_prefixes(&self) -> Option<&str> {
        let field = self.as_slice()[3];
        if field.is_empty() { None } else { Some(field) }
    }

    pub fn honorific_suffixes(&self) -> Option<&str> {
        let field = self.as_slice()[4];
        if field.is_empty() { None } else { Some(field) }
    }

    /// Returns the full name. Reconstructs the full name with the following
    /// format: `{Honorific Prefixes} {Given Name} {Additional Names}
    /// {Family Name}, {Honorific Suffixes}`.
    pub fn full_name(&self) -> String {
        let mut parts = Vec::with_capacity(4);

        if let Some(prefixes) = self.honorific_prefixes() {
            parts.push(prefixes);
        }
        if let Some(given) = self.given_name() {
            parts.push(given);
        }
        if let Some(additional) = self.additional_names() {
            parts.push(additional);
        }
        if let Some(family) = self.family_name() {
            parts.push(family);
        }

        let mut result = parts.join(" ");

        if let Some(suffixes) = self.honorific_suffixes() {
            if !result.is_empty() {
                result.push_str(", ");
            }
            result.push_str(suffixes);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_name() {
        let name = Name::new("Doe;John;Michael;Dr.;Jr.".to_string());

        assert_eq!(name.family_name(), Some("Doe"));
        assert_eq!(name.given_name(), Some("John"));
        assert_eq!(name.additional_names(), Some("Michael"));
        assert_eq!(name.honorific_prefixes(), Some("Dr."));
        assert_eq!(name.honorific_suffixes(), Some("Jr."));
        assert_eq!(name.full_name(), "Dr. John Michael Doe, Jr.");
    }

    #[test]
    fn test_incomplete_name_padding() {
        let name = Name::new("Holland;Tom".to_string());

        assert_eq!(name.family_name(), Some("Holland"));
        assert_eq!(name.given_name(), Some("Tom"));
        assert_eq!(name.additional_names(), None);
        assert_eq!(name.honorific_prefixes(), None);
        assert_eq!(name.honorific_suffixes(), None);
        assert_eq!(name.full_name(), "Tom Holland");
    }

    #[test]
    fn test_empty_fields() {
        let name = Name::new("Smith;;Jane;;".to_string());

        assert_eq!(name.family_name(), Some("Smith"));
        assert_eq!(name.given_name(), None);
        assert_eq!(name.additional_names(), Some("Jane"));
        assert_eq!(name.honorific_prefixes(), None);
        assert_eq!(name.honorific_suffixes(), None);
        assert_eq!(name.full_name(), "Jane Smith");
    }

    #[test]
    fn test_only_family_name() {
        let name = Name::new("Einstein".to_string());

        assert_eq!(name.family_name(), Some("Einstein"));
        assert_eq!(name.given_name(), None);
        assert_eq!(name.additional_names(), None);
        assert_eq!(name.honorific_prefixes(), None);
        assert_eq!(name.honorific_suffixes(), None);
        assert_eq!(name.full_name(), "Einstein");
    }

    #[test]
    fn test_with_suffixes_only() {
        let name = Name::new("Brown;Alice;;;PhD".to_string());

        assert_eq!(name.family_name(), Some("Brown"));
        assert_eq!(name.given_name(), Some("Alice"));
        assert_eq!(name.additional_names(), None);
        assert_eq!(name.honorific_prefixes(), None);
        assert_eq!(name.honorific_suffixes(), Some("PhD"));
        assert_eq!(name.full_name(), "Alice Brown, PhD");
    }

    #[test]
    fn test_as_slice() {
        let name = Name::new("Doe;John;Michael;Dr.;Jr.".to_string());
        let slice = name.as_slice();

        assert_eq!(slice, ["Doe", "John", "Michael", "Dr.", "Jr."]);
    }

    #[test]
    fn test_as_slice_padded() {
        let name = Name::new("Smith;Jane".to_string());
        let slice = name.as_slice();

        assert_eq!(slice, ["Smith", "Jane", "", "", ""]);
    }

    #[test]
    fn test_trimming_on_creation() {
        let name = Name::new(" Doe ; John ; Michael ; Dr. ; Jr. ".to_string());

        assert_eq!(name.family_name(), Some("Doe"));
        assert_eq!(name.given_name(), Some("John"));
        assert_eq!(name.additional_names(), Some("Michael"));
        assert_eq!(name.honorific_prefixes(), Some("Dr."));
        assert_eq!(name.honorific_suffixes(), Some("Jr."));
        assert_eq!(name.as_raw(), "Doe;John;Michael;Dr.;Jr.");
    }

    #[test]
    fn test_as_raw() {
        let name = Name::new("Smith;Jane;Marie".to_string());
        assert_eq!(name.as_raw(), "Smith;Jane;Marie;;");

        let complete_name = Name::new("Doe;John;Michael;Dr.;Jr.".to_string());
        assert_eq!(complete_name.as_raw(), "Doe;John;Michael;Dr.;Jr.");

        let single_name = Name::new("Einstein".to_string());
        assert_eq!(single_name.as_raw(), "Einstein;;;;");
    }
}
