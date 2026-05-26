//! Unit tests for organiclever-be library.

mod config_tests {
    use organiclever_be::config::Config;

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
