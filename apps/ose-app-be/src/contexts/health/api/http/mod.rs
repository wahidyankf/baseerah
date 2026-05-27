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
pub fn routes() -> Router {
    Router::new().route("/health", get(get_health_handler))
}
