use actix_web::{App, HttpServer, Responder, get, web};
use color_eyre::eyre::Context;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> color_eyre::eyre::Result<()> {
    hq_api::init_env();
    hq_api::logger_init();

    let config = hq_api::tls::init("./.data").context("Failed to initialize TLS configuration")?;

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
