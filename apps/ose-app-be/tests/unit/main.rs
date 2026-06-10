//! Unit tests for ose-app-be library.

mod config_tests {
    use ose_app_be::config::Config;

    fn from_pairs(pairs: &[(&str, &str)]) -> Result<Config, envy::Error> {
        envy::from_iter::<_, Config>(pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())))
    }

    // RED (item 68): Config must fail fast when DATABASE_URL is absent.
    #[test]
    fn load_fails_when_database_url_absent() {
        let result = from_pairs(&[
            ("OSE_APP_BE_PORT", "8302"),
            ("OSE_APP_BE_CORS_ORIGINS", "*"),
        ]);
        assert!(
            result.is_err(),
            "Config deserialization must fail when DATABASE_URL is absent"
        );
    }

    #[test]
    fn prefixed_port_key_resolves_to_port_value() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/ose_app"),
            ("OSE_APP_BE_PORT", "8399"),
        ])
        .expect("valid config");
        assert_eq!(config.ose_app_be_port, 8399_u16);
    }

    #[test]
    fn prefixed_cors_origins_key_resolves_to_cors_value() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/ose_app"),
            ("OSE_APP_BE_CORS_ORIGINS", "https://app.oseplatform.com"),
        ])
        .expect("valid config");
        assert_eq!(
            config.ose_app_be_cors_origins,
            "https://app.oseplatform.com"
        );
    }

    #[test]
    fn prefixed_openrouter_api_key_resolves_to_key_value() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/ose_app"),
            ("OSE_APP_BE_OPENROUTER_API_KEY", "sk-test-key"),
        ])
        .expect("valid config");
        assert_eq!(config.ose_app_be_openrouter_api_key, "sk-test-key");
    }

    #[test]
    fn prefixed_openrouter_model_resolves_to_model_value() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/ose_app"),
            ("OSE_APP_BE_OPENROUTER_MODEL", "gpt-4o"),
        ])
        .expect("valid config");
        assert_eq!(config.ose_app_be_openrouter_model, "gpt-4o");
    }

    #[test]
    fn prefixed_openrouter_base_url_resolves_to_url_value() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/ose_app"),
            (
                "OSE_APP_BE_OPENROUTER_BASE_URL",
                "https://custom.openrouter.ai/api/v1",
            ),
        ])
        .expect("valid config");
        assert_eq!(
            config.ose_app_be_openrouter_base_url,
            "https://custom.openrouter.ai/api/v1"
        );
    }

    #[test]
    fn default_port_is_8302() {
        let config =
            from_pairs(&[("DATABASE_URL", "postgres://localhost/ose_app")]).expect("valid config");
        assert_eq!(config.ose_app_be_port, 8302);
    }

    #[test]
    fn custom_port_resolves() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/ose_app"),
            ("OSE_APP_BE_PORT", "9090"),
        ])
        .expect("valid config");
        assert_eq!(config.ose_app_be_port, 9090);
    }

    #[test]
    fn custom_database_url_resolves() {
        let config = from_pairs(&[("DATABASE_URL", "postgres://user:pass@host/mydb")])
            .expect("valid config");
        assert_eq!(config.database_url, "postgres://user:pass@host/mydb");
    }

    #[test]
    fn default_cors_origins_is_wildcard() {
        let config =
            from_pairs(&[("DATABASE_URL", "postgres://localhost/ose_app")]).expect("valid config");
        assert_eq!(config.ose_app_be_cors_origins, "*");
    }

    #[test]
    fn default_openrouter_model_is_auto() {
        let config =
            from_pairs(&[("DATABASE_URL", "postgres://localhost/ose_app")]).expect("valid config");
        assert_eq!(config.ose_app_be_openrouter_model, "openrouter/auto");
    }

    #[test]
    fn custom_openrouter_fields_resolve() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/ose_app"),
            ("OSE_APP_BE_OPENROUTER_API_KEY", "key"),
            ("OSE_APP_BE_OPENROUTER_MODEL", "gpt-4"),
            ("OSE_APP_BE_OPENROUTER_BASE_URL", "https://api.example.com"),
        ])
        .expect("valid config");
        assert_eq!(config.ose_app_be_openrouter_model, "gpt-4");
        assert_eq!(config.ose_app_be_openrouter_api_key, "key");
        assert_eq!(
            config.ose_app_be_openrouter_base_url,
            "https://api.example.com"
        );
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
