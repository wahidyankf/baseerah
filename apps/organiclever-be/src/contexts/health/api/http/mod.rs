/// Wire-format contract types for the health HTTP API (hand-written from `OpenAPI` spec).
pub mod contracts;

use axum::{Json, Router, http::StatusCode, routing::get};
use contracts::HealthResponse;

use crate::contexts::health::application;

/// Axum handler for `GET /health`.
pub async fn get_health_handler() -> (StatusCode, Json<HealthResponse>) {
    let status = application::get_health();
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: status.status,
        }),
    )
}

/// Returns the Axum sub-router for the health context.
///
/// Generic over `S` so it can be merged into any typed `Router<S>` without
/// requiring a state conversion. The handler itself is stateless.
pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/health", get(get_health_handler))
}
