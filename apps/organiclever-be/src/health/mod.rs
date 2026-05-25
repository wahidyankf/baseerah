//! Health check endpoint handler.

use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

/// Returns `{"status": "ok"}` with HTTP 200.
///
/// # Errors
///
/// This handler never returns an error; the return type satisfies the Axum handler trait.
pub async fn get_health() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"status": "ok"})))
}
