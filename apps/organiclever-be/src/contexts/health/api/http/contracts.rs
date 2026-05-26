// [Judgment call]: openapi-generator rust-axum output was insufficient; types hand-written from spec
use serde::{Deserialize, Serialize};

/// HTTP response body for the health endpoint, mirroring the OpenAPI `HealthResponse` schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Service health status string (e.g. `"ok"`).
    pub status: String,
}
