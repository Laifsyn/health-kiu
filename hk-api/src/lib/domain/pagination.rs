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
