//! Application use cases for the health bounded context.

use super::domain::HealthStatus;

/// Returns the current health status of the application.
///
/// This is a pure function — no I/O, no `axum` dependency.
#[must_use]
pub fn get_health() -> HealthStatus {
    HealthStatus {
        status: "healthy".to_owned(),
    }
}
