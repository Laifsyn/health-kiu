use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use color_eyre::eyre::Context;
use tracing::info;

mod migrator_main;
use hk_api::tls::get_rustls_config;
use hk_api::AppState;
pub use migrator_main::main as run_migrations;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> color_eyre::eyre::Result<()> {
    hk_api::init_env();
    hk_api::logger_init();
    // Initialize the TLS provider
    *hk_api::tls::PROVIDER_INIT;

    let db_url =
        run_migrations().await.context("Failed to run database migrations")?;

    let config = get_rustls_config("./.data")
        .await
        .context("Failed to get rustls config")?;

    // Initialize application state
    let app_state = Arc::new(AppState::new(&db_url).await.context("Failed to initialize app state")?);

    let app = hk_api::create_router(app_state)
        .route("/hello/{name}", get(greet))
        .route("/health_check", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    info!("listening on https://{}/", addr);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .context("Error occurred while running server")?;

    Ok(())
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello {}!", name)
}
async fn health_check() -> impl IntoResponse { (StatusCode::OK, "Hello world") }
