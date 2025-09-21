//! Data Transfer Objects for incoming and outgoing data.
mod pagination;
mod specialty;

pub use pagination::{PaginatedReq, PaginatedResp, PaginationLimit};
pub use specialty::*;
