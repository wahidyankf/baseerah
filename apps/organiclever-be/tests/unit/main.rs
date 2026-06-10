//! Unit tests for organiclever-be library.

mod config_tests {
    use std::env;

    use organiclever_be::config::Config;

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

    // RED: Config must read ORGANICLEVER_BE_PORT, not PORT.
    // Fails until the ENV_PORT constant is renamed to "ORGANICLEVER_BE_PORT".
    #[test]
    fn prefixed_port_key_resolves_to_port_value() {
        let config = Config::from_env_fn(mock_env(&[("ORGANICLEVER_BE_PORT", "8299")]));
        assert_eq!(config.port, 8299_u16);
    }

    // RED: Config must read ORGANICLEVER_BE_CORS_ORIGINS, not CORS_ORIGINS.
    // Fails until the ENV_CORS_ORIGINS constant is renamed to "ORGANICLEVER_BE_CORS_ORIGINS".
    #[test]
    fn prefixed_cors_origins_key_resolves_to_cors_value() {
        let config = Config::from_env_fn(mock_env(&[(
            "ORGANICLEVER_BE_CORS_ORIGINS",
            "https://example.com",
        )]));
        assert_eq!(config.cors_origins, "https://example.com");
    }

    #[test]
    fn test_default_port() {
        // PORT is not expected to be set in a clean test environment;
        // Config::from_env falls back to 8202.
        let cfg = Config::from_env_with("", "", "");
        assert_eq!(cfg.port, 8202);
    }

    #[test]
    fn test_custom_port() {
        let cfg = Config::from_env_with("postgres://localhost/db", "9090", "*");
        assert_eq!(cfg.port, 9090);
    }

    #[test]
    fn test_default_database_url() {
        let cfg = Config::from_env_with("", "", "");
        assert_eq!(
            cfg.database_url,
            "postgres://postgres:postgres@localhost:5432/organiclever"
        );
    }

    #[test]
    fn test_custom_database_url() {
        let cfg = Config::from_env_with("postgres://user:pass@host/mydb", "8202", "*");
        assert_eq!(cfg.database_url, "postgres://user:pass@host/mydb");
    }

    #[test]
    fn test_default_cors_origins() {
        let cfg = Config::from_env_with("", "", "");
        assert_eq!(cfg.cors_origins, "*");
    }

    #[test]
    fn test_custom_cors_origins() {
        let cfg = Config::from_env_with("", "8202", "https://example.com");
        assert_eq!(cfg.cors_origins, "https://example.com");
    }

    #[test]
    fn test_from_env_returns_valid_config() {
        // Exercises Config::from_env() code path; env vars may or may not be set,
        // but the function must always return a structurally valid Config.
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
    fn test_from_env_with_invalid_port_defaults_to_8202() {
        let cfg = Config::from_env_with("", "not-a-number", "");
        assert_eq!(cfg.port, 8202);
    }
}

mod error_tests {
    use axum::response::IntoResponse;
    use organiclever_be::errors::AppError;

    #[test]
    fn test_internal_error_status() {
        let err = AppError::Internal("test error".to_string());
        let resp = err.into_response();
        assert_eq!(resp.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}

mod health_tests {
    use axum::http::StatusCode;
    use organiclever_be::contexts::health::api::http;

    #[tokio::test]
    async fn test_health_returns_ok() {
        let resp = http::get_health_handler().await;
        assert_eq!(resp.0, StatusCode::OK);
    }
}

mod router_tests {
    use organiclever_be::app;

    #[tokio::test]
    async fn test_app_router_compiles() {
        let _ = app::router();
    }
}
