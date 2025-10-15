use axum::extract::State;
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Basic;

use super::prelude::*;
#[allow(unused)]
pub async fn login(
    TypedHeader(credentials): TypedHeader<Authorization<Basic>>,
    State(state): StateApp,
) {
    todo!()
}
