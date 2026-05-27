//! Integration tests for `ose-app-be` — cucumber-rs BDD harness.
//!
//! Covers `specs/apps/ose-app/behavior/be/gherkin/health/health.feature`.

use cucumber::{World, given, then, when};
use ose_app_be::app;

/// World context shared across all cucumber step implementations.
#[derive(Debug, Default, World)]
pub struct ApiWorld {
    /// Base URL of the running test server (e.g., `http://127.0.0.1:12345`).
    pub base_url: String,
    /// HTTP status code from the most recent response.
    pub last_status: u16,
    /// Response body from the most recent request.
    pub last_body: String,
}

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
    let router = app::router();
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

/// Entry point for the cucumber-rs test runner.
#[tokio::main]
async fn main() {
    ApiWorld::run("../../specs/apps/ose-app/behavior/be/gherkin/health/health.feature").await;
}
