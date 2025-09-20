/// Pagination struct to handle offset and limit for paginated queries.
///
/// - `offset`: The starting point for the records to fetch.
/// - `limit`: The maximum number of records to fetch, capped at
///   [`Pagination::MAX_OFFSET`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[readonly::make]
pub struct Pagination {
    pub offset: u64,
    pub limit: u16,
}

#[derive(thiserror::Error, Debug)]
#[error("Pagination parameters are out of bounds")]
pub struct OutOfBoundsPagination;

impl Pagination {
    /// Maximum limit allowed for pagination queries. Used to avoid huge data
    /// fetching.
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
