//! Axum router and middleware configuration.

use axum::Router;
use tower_http::cors::CorsLayer;

use crate::contexts::health::api::http as health_http;

/// Build and return the application router with CORS middleware.
pub fn router() -> Router {
    Router::new()
        .nest("/api/v1", api_router())
        .layer(CorsLayer::permissive())
}

/// Build the versioned API sub-router.
fn api_router() -> Router {
    health_http::routes()
}
