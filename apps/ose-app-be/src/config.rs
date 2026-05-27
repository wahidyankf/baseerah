//! Application configuration loaded from environment variables.

use std::env;

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
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://ose_app:ose_app@localhost:5432/ose_app".to_owned());
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8302_u16);
        let cors_origins = env::var("CORS_ORIGINS").unwrap_or_else(|_| "*".to_owned());
        let openrouter_api_key = env::var("OPENROUTER_API_KEY").unwrap_or_default();
        let openrouter_model =
            env::var("OPENROUTER_MODEL").unwrap_or_else(|_| "openrouter/auto".to_owned());
        let openrouter_base_url = env::var("OPENROUTER_BASE_URL")
            .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_owned());
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
            "postgres://ose_app:ose_app@localhost:5432/ose_app".to_owned()
        } else {
            database_url.to_owned()
        };
        let port: u16 = port.parse().unwrap_or(8302_u16);
        let cors_origins = if cors_origins.is_empty() {
            "*".to_owned()
        } else {
            cors_origins.to_owned()
        };
        let openrouter_api_key = openrouter_api_key.to_owned();
        let openrouter_model = if openrouter_model.is_empty() {
            "openrouter/auto".to_owned()
        } else {
            openrouter_model.to_owned()
        };
        let openrouter_base_url = if openrouter_base_url.is_empty() {
            "https://openrouter.ai/api/v1".to_owned()
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
