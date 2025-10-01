/// Pagination parameters for database queries.
/// Used to limit the number of records fetched and to avoid huge data fetching.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[readonly::make]
pub struct Pagination {
    /// The starting point for the records to fetch.
    pub offset: u64,
    /// The maximum number of records to fetch, capped at
    ///   [`Pagination::MAX_OFFSET`].
    pub limit: u16,
}

#[derive(thiserror::Error, Debug)]
#[error("Pagination parameters are out of bounds")]
pub struct OutOfBoundsPagination;

impl Pagination {
    /// Maximum limit allowed per pagination.
    pub const MAX_LIMIT: u16 = 1024;

    pub const fn new(offset: u64, limit: u16) -> Option<Self> {
        if limit == 0 || limit > Self::MAX_LIMIT {
            None
        } else {
            Some(Self { offset, limit })
        }
    }

    /// Destructures [`Self`] as `(offset, limit)`.
    pub const fn tuple(self) -> (u64, u16) {
        let Pagination { offset, limit } = self;
        (offset, limit)
    }
}

impl Default for Pagination {
    fn default() -> Self { Self { offset: 0, limit: 10 } }
}

/// A Result of a paginated fetch to the database.
pub struct Paged<T> {
    /// Offset to use for the next page
    pub next_offset: u64,
    /// Requested page size.
    pub requested_size: u16,
    pub items: Vec<T>,
}

impl<T> Paged<T> {
    pub fn new(items: Vec<T>, pagination: Pagination) -> Self {
        Paged {
            next_offset: pagination.offset + items.len() as u64,
            requested_size: pagination.limit,
            items,
        }
    }

    /// Creates a new paged result by transforming the items using the provided
    /// function.
    pub fn new_with_transform<U>(
        items: Vec<U>,
        pagination: Pagination,
        f: impl FnMut(U) -> T,
    ) -> Self {
        let items = items.into_iter().map(f).collect();
        Paged::new(items, pagination)
    }

    /// Applies the provided function to transform the items in the paged
    /// result.
    pub fn transform<U>(self, f: impl FnMut(T) -> U) -> Paged<U> {
        let Paged { next_offset, requested_size, items } = self;
        let items = items.into_iter().map(f).collect();
        Paged { next_offset, requested_size, items }
    }
}

#[cfg(test)]
mod test {
    pub use super::*;
    #[test]
    fn destructuring_position() {
        let p = Pagination { offset: 10, limit: 20 };

        let Pagination { offset, limit } = p.clone();
        let (tuple_offset, tuple_limit) = p.clone().tuple();

        assert_eq!(tuple_offset, offset);
        assert_eq!(tuple_limit, limit);
    }
}
