//! `OrganicLever` backend — Axum entry point.

#![forbid(unsafe_code)]

use organiclever_be::{
    app::{self, AppState},
    config::Config,
    contexts::db,
    messaging::{client as nats_client, jetstream_demo, status as messaging_status},
};
use tracing_subscriber::EnvFilter;

/// Start the `OrganicLever` backend HTTP server.
///
/// Reads configuration from environment variables, connects to the database
/// and applies any pending migrations, connects to NATS, runs the `JetStream`
/// demo, then binds the HTTP server to the configured port.
/// Panics on listener bind failure or server error — both are fatal startup
/// conditions with no meaningful recovery path.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::load().expect("failed to load configuration from environment");

    // Connect to the database and run all pending migrations before serving.
    let pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .expect("failed to connect to PostgreSQL");
    db::run_migrations(&pool)
        .await
        .expect("database migration failed");

    // Connect to NATS (fail-fast if unreachable).
    let nats = nats_client::connect(&config.organiclever_be_nats_url)
        .await
        .expect("failed to connect to NATS");

    // Run JetStream durable demo and record outcome.
    let shared_status = messaging_status::new_shared();
    let demo_result = jetstream_demo::run(&nats).await;
    {
        let mut status = shared_status.lock().await;
        status.jetstream_demo = Some(match demo_result {
            Ok(s) => s,
            Err(e) => format!("failed: {e}"),
        });
    }

    let app_state = AppState {
        nats: Some(nats),
        messaging_status: shared_status,
    };
    let router = app::router(app_state);

    let addr = format!("0.0.0.0:{}", config.organiclever_be_port);
    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind port");

    axum::serve(listener, router).await.expect("server error");
}
