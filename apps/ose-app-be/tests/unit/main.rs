//! Unit tests for ose-app-be library.

mod config_tests {
    use std::env;

    use ose_app_be::config::Config;

    /// Helper: build a mock env-var lookup from a static list of key-value pairs.
    fn mock_env(
        pairs: &'static [(&'static str, &'static str)],
    ) -> impl Fn(&str) -> Result<String, env::VarError> {
        |key: &str| {
            pairs
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| v.to_string())
                .ok_or(env::VarError::NotPresent)
        }
    }

    // RED: Config must read OSE_APP_BE_PORT, not PORT.
    // Fails until ENV_PORT constant is renamed to "OSE_APP_BE_PORT".
    #[test]
    fn prefixed_port_key_resolves_to_port_value() {
        let config = Config::from_env_fn(mock_env(&[("OSE_APP_BE_PORT", "8399")]));
        assert_eq!(config.port, 8399_u16);
    }

    // RED: Config must read OSE_APP_BE_CORS_ORIGINS, not CORS_ORIGINS.
    // Fails until ENV_CORS_ORIGINS constant is renamed to "OSE_APP_BE_CORS_ORIGINS".
    #[test]
    fn prefixed_cors_origins_key_resolves_to_cors_value() {
        let config = Config::from_env_fn(mock_env(&[(
            "OSE_APP_BE_CORS_ORIGINS",
            "https://app.oseplatform.com",
        )]));
        assert_eq!(config.cors_origins, "https://app.oseplatform.com");
    }

    // RED: Config must read OSE_APP_BE_OPENROUTER_API_KEY, not OPENROUTER_API_KEY.
    // Fails until ENV_OPENROUTER_API_KEY is renamed to "OSE_APP_BE_OPENROUTER_API_KEY".
    #[test]
    fn prefixed_openrouter_api_key_resolves_to_key_value() {
        let config = Config::from_env_fn(mock_env(&[(
            "OSE_APP_BE_OPENROUTER_API_KEY",
            "sk-test-key",
        )]));
        assert_eq!(config.openrouter_api_key, "sk-test-key");
    }

    // RED: Config must read OSE_APP_BE_OPENROUTER_MODEL, not OPENROUTER_MODEL.
    // Fails until ENV_OPENROUTER_MODEL is renamed to "OSE_APP_BE_OPENROUTER_MODEL".
    #[test]
    fn prefixed_openrouter_model_resolves_to_model_value() {
        let config = Config::from_env_fn(mock_env(&[("OSE_APP_BE_OPENROUTER_MODEL", "gpt-4o")]));
        assert_eq!(config.openrouter_model, "gpt-4o");
    }

    // RED: Config must read OSE_APP_BE_OPENROUTER_BASE_URL, not OPENROUTER_BASE_URL.
    // Fails until ENV_OPENROUTER_BASE_URL is renamed to "OSE_APP_BE_OPENROUTER_BASE_URL".
    #[test]
    fn prefixed_openrouter_base_url_resolves_to_url_value() {
        let config = Config::from_env_fn(mock_env(&[(
            "OSE_APP_BE_OPENROUTER_BASE_URL",
            "https://custom.openrouter.ai/api/v1",
        )]));
        assert_eq!(
            config.openrouter_base_url,
            "https://custom.openrouter.ai/api/v1"
        );
    }

    #[test]
    fn test_default_port() {
        let cfg = Config::from_env_with("", "", "", "", "", "");
        assert_eq!(cfg.port, 8302);
    }

    #[test]
    fn test_custom_port() {
        let cfg = Config::from_env_with("postgres://localhost/db", "9090", "*", "", "", "");
        assert_eq!(cfg.port, 9090);
    }

    #[test]
    fn test_default_database_url() {
        let cfg = Config::from_env_with("", "", "", "", "", "");
        assert_eq!(
            cfg.database_url,
            "postgres://ose_app:ose_app@localhost:5432/ose_app"
        );
    }

    #[test]
    fn test_custom_database_url() {
        let cfg = Config::from_env_with("postgres://user:pass@host/mydb", "8302", "*", "", "", "");
        assert_eq!(cfg.database_url, "postgres://user:pass@host/mydb");
    }

    #[test]
    fn test_default_cors_origins() {
        let cfg = Config::from_env_with("", "", "", "", "", "");
        assert_eq!(cfg.cors_origins, "*");
    }

    #[test]
    fn test_custom_cors_origins() {
        let cfg = Config::from_env_with("", "8302", "https://example.com", "", "", "");
        assert_eq!(cfg.cors_origins, "https://example.com");
    }

    #[test]
    fn test_default_openrouter_model() {
        let cfg = Config::from_env_with("", "", "", "", "", "");
        assert_eq!(cfg.openrouter_model, "openrouter/auto");
    }

    #[test]
    fn test_custom_openrouter_model() {
        let cfg = Config::from_env_with("", "", "", "key", "gpt-4", "https://api.example.com");
        assert_eq!(cfg.openrouter_model, "gpt-4");
        assert_eq!(cfg.openrouter_api_key, "key");
        assert_eq!(cfg.openrouter_base_url, "https://api.example.com");
    }

    #[test]
    fn test_from_env_returns_valid_config() {
        let cfg = Config::from_env();
        assert!(cfg.port > 0, "port must be non-zero");
        assert!(
            !cfg.database_url.is_empty(),
            "database_url must be non-empty"
        );
        assert!(
            !cfg.cors_origins.is_empty(),
            "cors_origins must be non-empty"
        );
    }

    #[test]
    fn test_from_env_with_invalid_port_defaults_to_8302() {
        let cfg = Config::from_env_with("", "not-a-number", "", "", "", "");
        assert_eq!(cfg.port, 8302);
    }
}

mod error_tests {
    use axum::response::IntoResponse;
    use ose_app_be::errors::AppError;

    #[test]
    fn test_internal_error_status() {
        let err = AppError::Internal("test error".to_string());
        let resp = err.into_response();
        assert_eq!(resp.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}

mod health_tests {
    use axum::http::StatusCode;
    use ose_app_be::contexts::health::api::http;

    #[tokio::test]
    async fn test_health_returns_healthy() {
        let resp = http::get_health_handler().await;
        assert_eq!(resp.0, StatusCode::OK);
        assert_eq!(resp.1.status, "healthy");
    }
}

mod router_tests {
    use ose_app_be::app;

    #[tokio::test]
    async fn test_app_router_compiles() {
        let _ = app::router();
    }
}
