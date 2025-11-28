use axum::extract::{Query, State};
use dto::ApiSpecialty;

use super::prelude::*;

pub async fn get_specialties(
    Query(pagination): Query<PaginatedReq>,
    State(app): StateApp,
) -> ResultPaged<ApiSpecialty> {
    let e = app
        .get_specialties(pagination.clone())
        .await
        .map(|e| PagedResp::from_paged_with_transform(e, ApiSpecialty::from))?;

    Ok(e.json())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::routes::dto::PaginationLimit;
    use crate::{init_env, logger_init};

    // FIXME: Delete this test because should be an integration test
    #[tokio::test]
    async fn test_available_specialties() {
        init_env();
        logger_init();

        let app = AppState::new_for_test().await;
        let pagination =
            PaginatedReq { offset: 0, count: PaginationLimit::new(5).unwrap() };
        todo!()
        // let result = specialties_get(Query(pagination), State(app)).await;
        // assert!(result.is_ok());
        // let Json(resp) = result.unwrap();
        // assert_serializeable(&resp);
        // let json = serde_json::to_string_pretty(&resp).unwrap();
        // println!("Response JSON: {}", json);
    }
    pub const fn assert_serializeable<T: serde::Serialize>(_: &T) {}
}
