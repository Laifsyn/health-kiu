use std::num::NonZero;

use serde::{Deserialize, Serialize};

use crate::domain::internal::{OutOfBoundsPagination, Pagination};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(Deserialize)]
/// Structure representing pagination parameters for requests.
pub struct PaginatedReq {
    /// Offset from the start. Default is 0 (no skipping).
    pub offset: u64,
    /// Amount of items to fetch. Default is 10, max is
    /// [`PaginationLimit::MAX`].
    pub count: PaginationLimit,
}

impl PaginatedReq {
    pub const DEFAULT_LIMIT: u16 = PaginationLimit::DEFAULT_VALUE;
    pub const DEFAULT_OFFSET: u64 = 0;
    pub const MAX_COUNT: u16 = PaginationLimit::MAX;

    pub fn unwrap(self) -> (u64, u16) {
        let limit = self.count.get();
        (self.offset, limit)
    }

    /// Converts a [`Pagination`] instance from a [`PaginatedReq`].
    /// # Note
    /// - Adds 1 to the count to check for more items.
    pub const fn into_pagination(self) -> Pagination {
        let PaginatedReq { offset, count } = self;

        // Add 1 to check for more items
        let limit = count.get() + 1;
        const { assert!(Pagination::MAX_LIMIT == PaginatedReq::MAX_COUNT + 1) };
        Pagination::new(offset, limit)
            .expect("Self::limit is in bounds to Pagination's MAX_LIMIT")
    }
}

impl From<PaginatedReq> for Pagination {
    /// Delegates to [`PaginatedReq::into_pagination`]
    fn from(req: PaginatedReq) -> Self { req.into_pagination() }
}

/// Newtype wrapper around [`NonZero<u16>`].  Represents the max size of items
/// to fetch.
///
/// # See more
/// - Max value: [`PaginationLimit::MAX`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[readonly::make]
pub struct PaginationLimit(pub NonZero<u16>);

impl PaginationLimit {
    /// Default amount of items to search for.
    pub const DEFAULT_VALUE: u16 = 10;
    /// Maximum amount of items to search for.
    // -1 to account for "more items" check
    pub const MAX: u16 = Pagination::MAX_LIMIT - 1;

    /// Default amount of items to search for.
    pub fn new(value: u16) -> Option<Self> {
        if value > Self::MAX { None } else { NonZero::new(value).map(Self) }
    }

    pub const fn get(self) -> u16 { self.0.get() }
}

impl<'de> Deserialize<'de> for PaginationLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let limit = u16::deserialize(deserializer)?;
        Self::new(limit).ok_or_else(|| {
            serde::de::Error::custom(OutOfBoundsPagination.to_string())
        })
    }
}

impl Default for PaginationLimit {
    fn default() -> Self {
        PaginationLimit(NonZero::new(Self::DEFAULT_VALUE).unwrap())
    }
}

/// Structure representing a paginated response.
#[derive(Debug)]
#[derive(Serialize)]
pub struct PaginatedResp<T> {
    /// The size of the page. Equivalent to [`Self::items.len()`][Self::items]
    pub page_size: u64,
    /// The offset to use for the next page, if any.
    pub next_offset: Option<u64>,
    pub has_more: bool,
    pub items: Vec<T>,
}

impl<T> PaginatedResp<T> {
    pub fn from_items(
        mut items: Vec<T>,
        pagination: PaginatedReq,
    ) -> PaginatedResp<T> {
        let count = pagination.count.get() as usize;
        let has_more = items.len() > count;

        debug_assert!(
            items.len() <= count + 1,
            "Backend/Database should've at most fetched `count + 1` items"
        );

        // Truncate to exact count if we have more
        items.truncate(count);

        let page_size = items.len() as u64;

        PaginatedResp {
            page_size,
            next_offset: Some(pagination.offset + page_size),
            has_more,
            items,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn test_conversion() {
        let req = PaginatedReq {
            offset: 5,
            count: PaginationLimit::new(20).unwrap(),
        };
        let pagination = Pagination::from(req.clone());

        assert_eq!(pagination.offset, req.offset);
        assert_eq!(pagination.limit, req.count.get() + 1); // +1 for checking more items
    }
}
