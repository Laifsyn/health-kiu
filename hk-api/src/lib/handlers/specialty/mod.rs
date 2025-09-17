use axum::extract::Query;

use crate::handlers::{PaginatedReq, PaginatedResp};

pub async fn available_specialties(
    Query(pagination): Query<PaginatedReq>,
) -> PaginatedResp<()> {
    println!("{:?}", pagination);
    unimplemented!();
}
