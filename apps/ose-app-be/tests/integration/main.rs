//! Integration tests for `ose-app-be` — cucumber-rs BDD harness.
//!
//! Covers the gherkin features in
//! `specs/apps/ose/behavior/app-be/gherkin/`.

use cucumber::{World, given, then, when};
use ose_app_be::app::{self, AppState};
use ose_app_be::contexts::db;
use ose_app_be::contexts::messaging::status as messaging_status;

/// World context shared across all cucumber step implementations.
#[derive(Debug, Default, World)]
pub struct ApiWorld {
    /// Base URL of the running test server (e.g., `http://127.0.0.1:12345`).
    pub base_url: String,
    /// HTTP status code from the most recent response.
    pub last_status: u16,
    /// Response body from the most recent request.
    pub last_body: String,
    /// Count of applied migrations recorded after running migrations.
    pub migration_count: i64,
}

// ── HTTP health step definitions ──────────────────────────────────────────────

/// Send GET `/api/v1/health` and store the response status and body in world.
async fn send_get_health(world: &mut ApiWorld) {
    let url = format!("{}/api/v1/health", world.base_url);
    let resp = reqwest::get(&url)
        .await
        .expect("GET /api/v1/health request");
    world.last_status = resp.status().as_u16();
    world.last_body = resp.text().await.expect("response body text");
}

/// Spin up the Axum server on an ephemeral port and record the base URL in world.
#[given("the ose-app-be service is running")]
async fn the_service_is_running(world: &mut ApiWorld) {
    let app_state = AppState {
        nats: None,
        messaging_status: messaging_status::new_shared(),
    };
    let router = app::router(app_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind ephemeral port");
    let port = listener.local_addr().expect("get local addr").port();
    tokio::spawn(async move {
        axum::serve(listener, router).await.expect("axum serve");
    });
    world.base_url = format!("http://127.0.0.1:{port}");
}

/// Send GET `/api/v1/health`.
#[when("I send GET /api/v1/health")]
async fn i_send_get_health(world: &mut ApiWorld) {
    send_get_health(world).await;
}

/// Assert the last response had HTTP status 200.
#[then("the response status is 200")]
fn response_status_is_200(world: &mut ApiWorld) {
    assert_eq!(world.last_status, 200, "expected HTTP 200");
}

/// Assert the JSON `status` field equals `expected`.
#[then(expr = "the response body has a {string} field equal to {string}")]
#[allow(clippy::needless_pass_by_value)]
fn response_body_field_equals(world: &mut ApiWorld, field: String, expected: String) {
    let body: serde_json::Value = serde_json::from_str(&world.last_body).expect("parse JSON body");
    let actual = body
        .get(&field)
        .and_then(serde_json::Value::as_str)
        .expect("field in JSON body");
    assert_eq!(actual, expected, "{field} field mismatch");
}

// ── Migration step definitions ─────────────────────────────────────────────────

/// Drop any existing `_sqlx_migrations` tracking data so the scenario starts fresh.
#[given("the ose-app-be database has no applied migrations")]
async fn ose_app_be_database_has_no_applied_migrations(_world: &mut ApiWorld) {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests");
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("connect to database");
    // Drop the sqlx migrations tracking table if it exists so we can verify
    // that run_migrations creates it and records entries.
    sqlx::query("DROP TABLE IF EXISTS _sqlx_migrations")
        .execute(&pool)
        .await
        .expect("drop _sqlx_migrations");
}

/// Run the `db::run_migrations` helper directly (the same helper called on boot).
#[when("the ose-app-be backend runs its migration routine")]
async fn ose_app_be_backend_runs_migration_routine(world: &mut ApiWorld) {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests");
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("connect to database");
    db::run_migrations(&pool)
        .await
        .expect("migrations must run without error");

    // Count rows to confirm migrations were recorded.
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations")
        .fetch_one(&pool)
        .await
        .expect("query _sqlx_migrations count");
    world.migration_count = count;
}

/// Assert at least one migration was recorded.
#[then("the ose-app-be migrations table records at least one applied migration")]
fn ose_app_be_migrations_table_has_entries(world: &mut ApiWorld) {
    assert!(
        world.migration_count > 0,
        "expected at least one row in _sqlx_migrations, got {}",
        world.migration_count
    );
}

// ── Entry point ────────────────────────────────────────────────────────────────

/// Entry point for the cucumber-rs test runner.
#[tokio::main]
async fn main() {
    ApiWorld::run("../../specs/apps/ose/behavior/app-be/gherkin").await;
}
