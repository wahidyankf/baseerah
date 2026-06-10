//! Application configuration loaded from environment variables.

use std::env;

/// Default database connection URL used when `DATABASE_URL` is not set.
const DEFAULT_DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/organiclever";
/// Default TCP port when `ORGANICLEVER_BE_PORT` is not set.
const DEFAULT_PORT: u16 = 8202_u16;
/// Default allowed CORS origins when `ORGANICLEVER_BE_CORS_ORIGINS` is not set.
const DEFAULT_CORS_ORIGINS: &str = "*";

/// Environment variable name for the database connection URL.
const ENV_DATABASE_URL: &str = "DATABASE_URL";
/// Environment variable name for the server port.
const ENV_PORT: &str = "ORGANICLEVER_BE_PORT";
/// Environment variable name for allowed CORS origins.
const ENV_CORS_ORIGINS: &str = "ORGANICLEVER_BE_CORS_ORIGINS";

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
        Self::from_env_fn(|key| env::var(key))
    }

    /// Load configuration using a custom env-var lookup function.
    ///
    /// This seam exists so unit tests can supply a mock lookup without
    /// mutating the process environment (which requires `unsafe` in edition
    /// 2024).
    ///
    /// Production code must use [`Config::from_env`] instead.
    pub fn from_env_fn<F>(lookup: F) -> Self
    where
        F: Fn(&str) -> Result<String, env::VarError>,
    {
        let database_url =
            lookup(ENV_DATABASE_URL).unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned());
        let port = lookup(ENV_PORT)
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(DEFAULT_PORT);
        let cors_origins =
            lookup(ENV_CORS_ORIGINS).unwrap_or_else(|_| DEFAULT_CORS_ORIGINS.to_owned());
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
            DEFAULT_DATABASE_URL.to_owned()
        } else {
            database_url.to_owned()
        };
        let port: u16 = port.parse().unwrap_or(DEFAULT_PORT);
        let cors_origins = if cors_origins.is_empty() {
            DEFAULT_CORS_ORIGINS.to_owned()
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
