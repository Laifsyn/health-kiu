pub use app::AppState;
pub use utils::{ResultExt, *};

pub use crate::adapters::{http, repo};

mod adapters;
pub mod app;
pub(crate) mod domain;
mod routes;
mod utils;

use std::sync::Arc;
use axum::{Router, routing::get};
use tower_http::cors::{CorsLayer, Any};

/// Creates the application router with all API routes configured.
pub fn create_router(state: Arc<AppState>) -> Router {
    // Configure CORS to allow requests from frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // API routes
        .route("/api/specialties", get(routes::specialty::get_specialties))
        .route("/api/specialties/{id}/doctors", get(routes::doctor::doctors_by_specialty))
        .layer(cors)
        .with_state(state)
}
