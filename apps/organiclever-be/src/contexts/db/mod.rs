//! `db` bounded context — database schema lifecycle.
//!
//! Cross-cutting infrastructure concern: applies pending sqlx migrations on
//! boot before the HTTP server starts.

pub mod infrastructure;

// Re-export the public API so callers can use `db::run_migrations` directly.
pub use infrastructure::run_migrations;
