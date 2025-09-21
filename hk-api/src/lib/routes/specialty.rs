use std::sync::Arc;

use axum::Json;
use axum::extract::{Query, State};
use itertools::Itertools;

use super::Result;
use crate::app::ServerApp;
use crate::domain::{self};
use crate::routes::dto::{PaginatedReq, PaginatedResp, Specialty};

pub async fn specialties_get(
    Query(pag_params): Query<Option<PaginatedReq>>,
    State(app): State<Arc<ServerApp>>,
) -> Result<Json<PaginatedResp<Specialty>>> {
    let pagination = pag_params.unwrap_or_default();
    let items = app
        .get_specialties(pagination.clone())
        .await?
        .into_iter()
        .map(Specialty::from)
        .collect_vec();
    let resp = PaginatedResp::from_items(items, pagination);
    Ok(Json(resp))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::routes::dto::PaginationLimit;
    use crate::{init_env, logger_init};

    #[tokio::test]
    async fn test_available_specialties() {
        init_env();
        logger_init();

        let app = ServerApp::new_for_test().await;
        let pagination =
            PaginatedReq { offset: 0, count: PaginationLimit::new(5).unwrap() };
        let result = specialties_get(Query(None), State(app)).await;
        assert!(result.is_ok());
        let Json(resp) = result.unwrap();
        assert_serializeable(&resp);
        let json = serde_json::to_string_pretty(&resp).unwrap();
        println!("Response JSON: {}", json);
    }
    pub const fn assert_serializeable<T: serde::Serialize>(_: &T) {}
}
