use std::num::NonZero;

use serde::{Deserialize, Serialize};

/// Structure representing a paginated response.
#[derive(Debug)]
#[derive(Serialize)]
pub struct PaginatedResp<T> {
    /// The size of each page.
    pub page_size: u64,
    /// The offset to use for the next page, if any.
    pub next_offset: Option<u64>,
    pub has_more: bool,
    pub items: Vec<T>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Deserialize)]
/// Structure representing pagination parameters for requests.
pub struct PaginatedReq {
    /// Offset from the start. Default is 0 (no skipping).
    pub offset: Option<u64>,
    /// Amount of items to return. Default is 10, max is
    /// [`PaginationLimit::MAX`].
    #[serde(deserialize_with = "PaginationLimit::deserialize_opt")]
    pub count: Option<PaginationLimit>,
}

/// Newtype wrapper around [`NonZero<u16>`] representing the limit of items to
/// return in a paginated request.
/// Defaults to [`PaginationLimit::DEFAULT`], maxes at [`PaginationLimit::MAX`].
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Deserialize)]
#[readonly::make]
pub struct PaginationLimit(pub NonZero<u16>);

impl PaginationLimit {
    pub const DEFAULT: u16 = 10;
    pub const MAX: u16 = 1024;

    pub fn new(value: u16) -> Option<Self> {
        if value > Self::MAX { None } else { NonZero::new(value).map(Self) }
    }

    pub fn deserialize_opt<'de, D>(
        deserializer: D,
    ) -> Result<Option<PaginationLimit>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(u16::deserialize(deserializer)
            .map(Self::new)
            .unwrap_or(Some(Self::default())))
    }
}

impl Default for PaginationLimit {
    fn default() -> Self { Self(NonZero::new(Self::DEFAULT).unwrap()) }
}
