use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use color_eyre::eyre::Context;
use hk_api::create_app;
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

    let app_state = init_app_state().await?;

    let app = create_app(app_state)
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

/// Initializes the application state.
async fn init_app_state() -> color_eyre::Result<hk_api::AppState> {
    hk_api::AppState::new(None).await
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello {}!", name)
}
async fn health_check() -> impl IntoResponse { (StatusCode::OK, "Hello world") }
