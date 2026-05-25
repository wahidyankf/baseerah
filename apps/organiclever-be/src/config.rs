//! Application configuration loaded from environment variables.

use std::env;

/// Runtime configuration for the organiclever-be server.
pub struct Config {
    /// `PostgreSQL` connection URL.
    pub database_url: String,
    /// TCP port to listen on.
    pub port: u16,
    /// Allowed CORS origins (comma-separated or `"*"`).
    pub cors_origins: String,
}

impl Config {
    /// Load configuration from environment variables with defaults.
    ///
    /// All environment variables have fallback defaults so this function
    /// always succeeds.
    #[must_use]
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/organiclever".to_owned()
        });
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8202_u16);
        let cors_origins = env::var("CORS_ORIGINS").unwrap_or_else(|_| "*".to_owned());
        Self {
            database_url,
            port,
            cors_origins,
        }
    }

    /// Build a `Config` from explicit string values, falling back to defaults
    /// when an argument is empty.
    ///
    /// This constructor is intended for unit testing where mutating the process
    /// environment via `std::env::set_var`/`remove_var` (which are `unsafe` in
    /// Rust edition 2024) should be avoided.
    ///
    /// # Arguments
    ///
    /// * `database_url` — pass `""` to use the default.
    /// * `port` — pass `""` to use the default (`8202`).
    /// * `cors_origins` — pass `""` to use the default (`"*"`).
    #[must_use]
    pub fn from_env_with(database_url: &str, port: &str, cors_origins: &str) -> Self {
        let database_url = if database_url.is_empty() {
            "postgres://postgres:postgres@localhost:5432/organiclever".to_owned()
        } else {
            database_url.to_owned()
        };
        let port: u16 = port.parse().unwrap_or(8202_u16);
        let cors_origins = if cors_origins.is_empty() {
            "*".to_owned()
        } else {
            cors_origins.to_owned()
        };
        Self {
            database_url,
            port,
            cors_origins,
        }
    }
}
