use axum::{Router, http};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::{AppState, Ulid};

/// Instantiates the main application router with all routes and middlewares.
pub fn create_app(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173".parse::<http::HeaderValue>().unwrap(),
        )
        .allow_methods([http::Method::POST, http::Method::GET])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    Router::new()
        .nest("/api", crate::routes::router())
        .with_state(app_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http().make_span_with(
            |request: &http::Request<_>| {
                let request_id = Ulid::new();
                tracing::info_span!(
                    "http-request",
                    method = %request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                    request_id = %request_id
                )
            },
        ))
}
