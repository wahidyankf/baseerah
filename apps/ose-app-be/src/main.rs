//! `ose-app-be` backend — Axum entry point.

#![forbid(unsafe_code)]

use ose_app_be::{
    app::{self, AppState},
    config::Config,
    messaging::{client as nats_client, jetstream_demo, status as messaging_status},
};
use tracing_subscriber::EnvFilter;

/// Start the `ose-app-be` backend HTTP server.
///
/// Reads configuration from environment variables, connects to NATS in a
/// background task (so the HTTP server starts even if NATS is temporarily
/// unavailable), then binds to the configured port. Panics on listener bind
/// failure or server error — both are fatal startup conditions.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::load().expect("failed to load configuration from environment");

    // Connect to NATS in a background task so the HTTP server starts even when
    // NATS is temporarily unreachable (e.g. integration tests run PostgreSQL-only).
    let nats_url = config.ose_app_be_nats_url.clone();
    let shared_status = messaging_status::new_shared();
    let status_for_task = shared_status.clone();

    let nats_handle = tokio::spawn(async move {
        match nats_client::connect(&nats_url).await {
            Ok(nats) => {
                let demo_result = jetstream_demo::run(&nats).await;
                {
                    let mut status = status_for_task.lock().await;
                    status.jetstream_demo = Some(match demo_result {
                        Ok(s) => s,
                        Err(e) => format!("failed: {e}"),
                    });
                }
                Some(nats)
            }
            Err(e) => {
                tracing::warn!("NATS unavailable at startup: {e}");
                {
                    let mut status = status_for_task.lock().await;
                    status.jetstream_demo = Some(format!("failed: {e}"));
                }
                None
            }
        }
    });

    // Wait briefly for NATS to connect (best-effort; HTTP server starts regardless).
    let nats_client = tokio::time::timeout(std::time::Duration::from_secs(5), nats_handle)
        .await
        .ok()
        .and_then(std::result::Result::ok)
        .flatten();

    let app_state = AppState {
        nats: nats_client,
        messaging_status: shared_status,
    };
    let router = app::router(app_state);

    let addr = format!("0.0.0.0:{}", config.ose_app_be_port);
    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind port");

    axum::serve(listener, router).await.expect("server error");
}
