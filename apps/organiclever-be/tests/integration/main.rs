//! Integration tests for `organiclever-be` — cucumber-rs BDD harness.
//!
//! Covers the gherkin features in
//! `specs/apps/organiclever/behavior/organiclever-be/gherkin/`.

use cucumber::{World, given, then, when};
use organiclever_be::app::{self, AppState};
use organiclever_be::contexts::db;
use organiclever_be::contexts::messaging::status as messaging_status;

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
#[given("the API is running")]
async fn the_api_is_running(world: &mut ApiWorld) {
    // Integration tests are PostgreSQL-only — NATS is not available.
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

/// Send GET `/health` as an operations engineer.
#[when("an operations engineer sends GET /health")]
async fn ops_engineer_sends_get_health(world: &mut ApiWorld) {
    send_get_health(world).await;
}

/// Send GET `/health` as an unauthenticated engineer.
#[when("an unauthenticated engineer sends GET /health")]
async fn unauthenticated_engineer_sends_get_health(world: &mut ApiWorld) {
    send_get_health(world).await;
}

/// Assert the last response had HTTP status 200.
#[then("the response status code should be 200")]
fn response_status_code_200(world: &mut ApiWorld) {
    assert_eq!(world.last_status, 200, "expected HTTP 200");
}

/// Assert the JSON `status` field equals `expected`.
#[then(expr = "the health status should be {string}")]
#[allow(clippy::needless_pass_by_value)] // cucumber-rs {string} captures must be owned String
fn health_status_should_be(world: &mut ApiWorld, expected: String) {
    let body: serde_json::Value = serde_json::from_str(&world.last_body).expect("parse JSON body");
    let actual = body
        .get("status")
        .and_then(serde_json::Value::as_str)
        .expect("status field in JSON body");
    assert_eq!(actual, expected, "health status mismatch");
}

/// Assert the response body does not contain component-level health details.
#[then("the response should not include detailed component health information")]
fn no_component_health_details(world: &mut ApiWorld) {
    assert!(
        !world.last_body.contains("components"),
        "response must not include component details but got: {}",
        world.last_body
    );
}

// ── Migration step definitions ─────────────────────────────────────────────────

/// Drop any existing `_sqlx_migrations` tracking data so the scenario starts fresh.
#[given("the database has no applied migrations")]
async fn database_has_no_applied_migrations(_world: &mut ApiWorld) {
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
#[when("the backend runs its migration routine")]
async fn backend_runs_migration_routine(world: &mut ApiWorld) {
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
#[then("the migrations table records at least one applied migration")]
fn migrations_table_has_entries(world: &mut ApiWorld) {
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
    ApiWorld::run("../../specs/apps/organiclever/behavior/organiclever-be/gherkin").await;
}
