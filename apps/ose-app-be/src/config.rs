//! Application configuration loaded from environment variables via `envy`.
//!
//! `dotenvy::dotenv().ok()` loads `.env.local` for local runs (no-op in CI).
//! `envy::from_env` deserializes into [`Config`] using the `SCREAMING_SNAKE` ↔
//! field-name mapping. Required fields (`database_url`, messaging URLs) fail fast
//! when absent; optional fields use typed `#[serde(default)]` helpers.

use serde::Deserialize;

/// Default TCP port for the server.
fn default_port() -> u16 {
    8302
}

/// Default CORS origins (allow all).
fn default_cors_origins() -> String {
    "*".to_owned()
}

/// Default `OpenRouter` model identifier.
fn default_openrouter_model() -> String {
    "openrouter/auto".to_owned()
}

/// Default `OpenRouter` base URL.
fn default_openrouter_base_url() -> String {
    "https://openrouter.ai/api/v1".to_owned()
}

/// Messaging-specific configuration (NATS + crane).
#[derive(Deserialize)]
pub struct MessagingConfig {
    /// NATS server URL (required — no default).
    pub ose_app_be_nats_url: String,
    /// crane-be base URL for PDF-to-Markdown conversion (required — no default).
    pub ose_app_be_crane_url: String,
}

/// Runtime configuration for the `ose-app-be` server.
#[derive(Deserialize)]
pub struct Config {
    /// `PostgreSQL` connection URL (required — no default).
    pub database_url: String,
    /// TCP port to listen on.
    #[serde(default = "default_port")]
    pub ose_app_be_port: u16,
    /// Allowed CORS origins (comma-separated or `"*"`).
    #[serde(default = "default_cors_origins")]
    pub ose_app_be_cors_origins: String,
    /// `OpenRouter` API key (optional).
    #[serde(default)]
    pub ose_app_be_openrouter_api_key: String,
    /// `OpenRouter` model identifier.
    #[serde(default = "default_openrouter_model")]
    pub ose_app_be_openrouter_model: String,
    /// `OpenRouter` base URL.
    #[serde(default = "default_openrouter_base_url")]
    pub ose_app_be_openrouter_base_url: String,

    // ── Messaging ──────────────────────────────────────────────────────────
    /// NATS server URL (required — no default).
    pub ose_app_be_nats_url: String,
    /// crane-be base URL for PDF-to-Markdown conversion (required — no default).
    pub ose_app_be_crane_url: String,
}

impl Config {
    /// Load configuration from the process environment, optionally seeded by
    /// `.env.local` via `dotenvy`.
    ///
    /// Returns `Err` if any required field is absent or cannot be parsed.
    ///
    /// # Errors
    ///
    /// Returns [`envy::Error`] when a required environment variable is missing
    /// or a value cannot be deserialized into the expected type.
    pub fn load() -> Result<Self, envy::Error> {
        dotenvy::dotenv().ok();
        envy::from_env::<Self>()
    }
}
