//! Integration tests for `organiclever-be` — cucumber-rs BDD harness.
//!
//! Covers `specs/apps/organiclever/behavior/organiclever-be/gherkin/health/health-check.feature`.

use cucumber::{World, given, then, when};
use organiclever_be::app;

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
#[given("the API is running")]
async fn the_api_is_running(world: &mut ApiWorld) {
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

/// Entry point for the cucumber-rs test runner.
#[tokio::main]
async fn main() {
    ApiWorld::run("../../specs/apps/organiclever/behavior/organiclever-be/gherkin/health/health-check.feature")
        .await;
}
