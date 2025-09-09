use actix_web::{App, HttpServer, Responder, get, web};
use color_eyre::eyre::Context;

mod migrator_main;
pub use migrator_main::main as run_migrations;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> color_eyre::eyre::Result<()> {
    hk_api::init_env();
    hk_api::logger_init();

    let _db_url =
        run_migrations().await.context("Failed to run database migrations")?;

    let config = hk_api::tls::init("./.data")
        .context("Failed to initialize TLS configuration")?;

    HttpServer::new(|| App::new().service(greet))
        .bind_rustls_0_23(("127.0.0.1", 8080), config)
        .context("Failed to bind to TLS socket")?
        .run()
        .await
        .context("Error occurred while running server")?;

    Ok(())
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}
