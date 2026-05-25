//! Axum router and middleware configuration.

use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;

use crate::health;

/// Build and return the application router with CORS middleware.
pub fn router() -> Router {
    Router::new()
        .nest("/api/v1", api_router())
        .layer(CorsLayer::permissive())
}

/// Build the versioned API sub-router.
fn api_router() -> Router {
    Router::new().route("/health", get(health::get_health))
}
