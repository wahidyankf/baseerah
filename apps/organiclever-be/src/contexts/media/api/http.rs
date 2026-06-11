//! HTTP handler for the media-convert endpoint.

use axum::{
    Router, body::Bytes, extract::State, http::StatusCode, response::IntoResponse, routing::post,
};

use crate::app::AppState;
use crate::messaging::crane_client;

/// `POST /api/v1/media/convert` — convert PDF via crane NATS request/reply.
///
/// Returns `503 Service Unavailable` when no NATS client is configured
/// (e.g. in PostgreSQL-only integration-test mode).
async fn convert(State(state): State<AppState>, body: Bytes) -> impl IntoResponse {
    let Some(ref nats) = state.nats else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "NATS client not configured".to_string(),
        );
    };
    match crane_client::convert_via_nats(nats, body.to_vec()).await {
        Ok(markdown) => (StatusCode::OK, markdown),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

/// Build media HTTP routes.
pub fn routes() -> Router<AppState> {
    Router::new().route("/media/convert", post(convert))
}
