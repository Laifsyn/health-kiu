pub use app::AppState;
pub use utils::{ResultExt, *};

pub use crate::adapters::{http, repo};

mod adapters;
pub mod app;
pub(crate) mod domain;
mod routes;
mod utils;

use std::sync::Arc;
use axum::{Router, routing::{get, post}};
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
        .route("/api/doctors/{doctor_id}/available-dates", get(routes::appointments::get_available_dates))
        .route("/api/doctors/{doctor_id}/appointments", post(routes::appointments::book_appointment))
        .layer(cors)
        .with_state(state)
}
