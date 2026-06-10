//! `OrganicLever` backend — Axum entry point.

#![forbid(unsafe_code)]

use organiclever_be::{app, config::Config};
use tracing_subscriber::EnvFilter;

/// Start the `OrganicLever` backend HTTP server.
///
/// Reads configuration from environment variables and binds to the configured
/// port. Panics on listener bind failure or server error — both are fatal
/// startup conditions with no meaningful recovery path.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::load().expect("failed to load configuration from environment");
    let router = app::router();

    let addr = format!("0.0.0.0:{}", config.organiclever_be_port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind port");

    tracing::info!("listening on {addr}");
    axum::serve(listener, router).await.expect("server error");
}
