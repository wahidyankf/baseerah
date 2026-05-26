//! Domain types for the health bounded context.

use serde::Serialize;

/// Represents the health status of the application.
#[derive(Debug, Serialize)]
pub struct HealthStatus {
    /// The current health status string (e.g., `"ok"`).
    pub status: String,
}
