//! HTTP API layer for the health bounded context.
//!
//! Provides the Axum handler and route registration for the health endpoint.

use axum::{Json, Router, http::StatusCode, routing::get};
use serde_json::{Value, json};

use crate::contexts::health::application;

/// Axum handler for `GET /health`.
///
/// Delegates to the application use case and serialises the result as JSON.
///
/// # Errors
///
/// This handler never returns an error; the return type satisfies the Axum handler trait.
pub async fn get_health_handler() -> (StatusCode, Json<Value>) {
    let status = application::get_health();
    (StatusCode::OK, Json(json!({"status": status.status})))
}

/// Returns the Axum sub-router for the health context.
pub fn routes() -> Router {
    Router::new().route("/health", get(get_health_handler))
}
