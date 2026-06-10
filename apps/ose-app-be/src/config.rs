//! Application configuration loaded from environment variables.

use std::env;

const DEFAULT_DATABASE_URL: &str = "postgres://ose_app:ose_app@localhost:5432/ose_app";
const DEFAULT_PORT: u16 = 8302_u16;
const DEFAULT_CORS_ORIGINS: &str = "*";
const DEFAULT_OPENROUTER_MODEL: &str = "openrouter/auto";
const DEFAULT_OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_PORT: &str = "OSE_APP_BE_PORT";
const ENV_CORS_ORIGINS: &str = "OSE_APP_BE_CORS_ORIGINS";
const ENV_OPENROUTER_API_KEY: &str = "OSE_APP_BE_OPENROUTER_API_KEY";
const ENV_OPENROUTER_MODEL: &str = "OSE_APP_BE_OPENROUTER_MODEL";
const ENV_OPENROUTER_BASE_URL: &str = "OSE_APP_BE_OPENROUTER_BASE_URL";

/// Runtime configuration for the `ose-app-be` server.
pub struct Config {
    /// `PostgreSQL` connection URL.
    pub database_url: String,
    /// TCP port to listen on.
    pub port: u16,
    /// Allowed CORS origins (comma-separated or `"*"`).
    pub cors_origins: String,
    /// `OpenRouter` API key.
    pub openrouter_api_key: String,
    /// `OpenRouter` model identifier.
    pub openrouter_model: String,
    /// `OpenRouter` base URL.
    pub openrouter_base_url: String,
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
        let openrouter_api_key = lookup(ENV_OPENROUTER_API_KEY).unwrap_or_default();
        let openrouter_model =
            lookup(ENV_OPENROUTER_MODEL).unwrap_or_else(|_| DEFAULT_OPENROUTER_MODEL.to_owned());
        let openrouter_base_url = lookup(ENV_OPENROUTER_BASE_URL)
            .unwrap_or_else(|_| DEFAULT_OPENROUTER_BASE_URL.to_owned());
        Self {
            database_url,
            port,
            cors_origins,
            openrouter_api_key,
            openrouter_model,
            openrouter_base_url,
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
    /// * `port` — pass `""` to use the default (`8302`).
    /// * `cors_origins` — pass `""` to use the default (`"*"`).
    /// * `openrouter_api_key` — pass `""` to use empty default.
    /// * `openrouter_model` — pass `""` to use `"openrouter/auto"`.
    /// * `openrouter_base_url` — pass `""` to use the default base URL.
    #[must_use]
    pub fn from_env_with(
        database_url: &str,
        port: &str,
        cors_origins: &str,
        openrouter_api_key: &str,
        openrouter_model: &str,
        openrouter_base_url: &str,
    ) -> Self {
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
        let openrouter_api_key = openrouter_api_key.to_owned();
        let openrouter_model = if openrouter_model.is_empty() {
            DEFAULT_OPENROUTER_MODEL.to_owned()
        } else {
            openrouter_model.to_owned()
        };
        let openrouter_base_url = if openrouter_base_url.is_empty() {
            DEFAULT_OPENROUTER_BASE_URL.to_owned()
        } else {
            openrouter_base_url.to_owned()
        };
        Self {
            database_url,
            port,
            cors_origins,
            openrouter_api_key,
            openrouter_model,
            openrouter_base_url,
        }
    }
}
