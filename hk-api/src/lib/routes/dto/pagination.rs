use std::marker::PhantomData;
use std::num::NonZero;

use serde::{Deserialize, Serialize};

use crate::domain::{OutOfBoundsPagination, Paged, Pagination};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(Deserialize)]
#[serde(default)]
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
pub struct PagedResp<T = PhantomData<()>> {
    /// The size of the page. Equivalent to [`Self::items.len()`][Self::items]
    pub page_size: u64,
    /// The offset to use for the next page, if any.
    pub next_offset: Option<u64>,
    pub has_more: bool,
    pub items: Vec<T>,
}

impl<T> PagedResp<T> {
    pub fn from_paged(paged_result: Paged<T>) -> Self {
        let Paged { mut next_offset, requested_size, mut items } = paged_result;
        let has_more = items.len() as u16 == requested_size;
        if has_more {
            items.pop();
            next_offset = next_offset.saturating_sub(1);
        }
        let page_size = items.len() as u64;
        let next_offset = if has_more { Some(next_offset) } else { None };

        PagedResp { page_size, next_offset, has_more, items }
    }

    pub fn from_paged_opt(paged_result: Option<Paged<T>>) -> Self {
        match paged_result {
            Some(paged) => Self::from_paged(paged),
            None => Self::empty(),
        }
    }

    pub fn from_paged_with_transform<U>(
        paged_result: Paged<U>,
        f: impl FnMut(U) -> T,
    ) -> Self {
        let paged_result = paged_result.transform(f);
        Self::from_paged(paged_result)
    }

    pub const fn empty() -> Self {
        PagedResp {
            page_size: 0,
            next_offset: None,
            has_more: false,
            items: Vec::new(),
        }
    }

    pub const fn json(self) -> axum::Json<Self> { axum::Json(self) }
}

impl<T> From<Paged<T>> for PagedResp<T> {
    /// Delegates to [`PagedResp::from_paged`]
    fn from(paged_result: Paged<T>) -> Self {
        PagedResp::from_paged(paged_result)
    }
}
impl<T> From<Option<Paged<T>>> for PagedResp<T> {
    /// Delegates to [`PagedResp::from_paged_opt`]
    fn from(paged_result: Option<Paged<T>>) -> Self {
        PagedResp::from_paged_opt(paged_result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_conversion() {
        let req = PaginatedReq::default();
        let pagination = Pagination::from(req.clone());

        assert_eq!(pagination.offset, req.offset, "No changes to offset");
        assert_eq!(
            pagination.limit,
            req.count.get() + 1,
            "Expects conversion count to be +1 higher"
        ); // +1 for checking more items
    }

    #[test]
    fn assert_count_is_len() {
        let items = vec![1, 2, 3, 4];
        let req = PaginatedReq {
            offset: 0,
            count: PaginationLimit::new(items.len() as u16 - 1).unwrap(),
        };

        let fetch_result =
            Paged::new(items.clone(), req.clone().into_pagination());
        let resp = PagedResp::from_paged(fetch_result);
        assert_eq!(
            resp.page_size as usize,
            resp.items.len(),
            "Expects page-size to be equal to items.len()"
        );
    }
}
