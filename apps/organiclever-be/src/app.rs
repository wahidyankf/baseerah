//! Axum router and middleware configuration.

use axum::{Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use tower_http::cors::CorsLayer;

use crate::contexts::health::api::http as health_http;
use crate::contexts::media::api::http as media_http;
use crate::contexts::messaging::status::SharedMessagingStatus;

/// Application state shared across all handlers.
#[derive(Clone)]
pub struct AppState {
    /// NATS client for messaging operations.
    /// `None` in integration-test mode (`PostgreSQL`-only, no NATS running).
    pub nats: Option<async_nats::Client>,
    /// Shared messaging status.
    pub messaging_status: SharedMessagingStatus,
}

/// `GET /api/v1/system/status/messaging` — reports `JetStream` demo outcome.
async fn messaging_status_handler(State(state): State<AppState>) -> impl IntoResponse {
    let status = state.messaging_status.lock().await;
    let demo = status
        .jetstream_demo
        .clone()
        .unwrap_or_else(|| "pending".to_string());
    (
        StatusCode::OK,
        axum::Json(serde_json::json!({ "jetstream_demo": demo })),
    )
}

/// Build and return the application router.
pub fn router(app_state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", api_router())
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}

/// Build the versioned API sub-router.
///
/// Returns `Router<AppState>` so that `with_state(app_state)` at the top
/// level resolves all `State<AppState>` extractors. The health sub-router
/// is generic and coerces to `Router<AppState>` at the call site.
fn api_router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/system/status/messaging", get(messaging_status_handler))
        .merge(health_http::routes::<AppState>())
        .merge(media_http::routes())
}
