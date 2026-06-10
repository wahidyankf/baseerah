//! Unit tests for organiclever-be library.

mod config_tests {
    use organiclever_be::config::Config;

    fn from_pairs(pairs: &[(&str, &str)]) -> Result<Config, envy::Error> {
        envy::from_iter::<_, Config>(pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())))
    }

    // RED (item 65): Config must fail fast when DATABASE_URL is absent.
    #[test]
    fn load_fails_when_database_url_absent() {
        let result = from_pairs(&[
            ("ORGANICLEVER_BE_PORT", "8202"),
            ("ORGANICLEVER_BE_CORS_ORIGINS", "*"),
        ]);
        assert!(
            result.is_err(),
            "Config deserialization must fail when DATABASE_URL is absent"
        );
    }

    #[test]
    fn prefixed_port_key_resolves_to_port_value() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/organiclever"),
            ("ORGANICLEVER_BE_PORT", "8299"),
        ])
        .expect("valid config");
        assert_eq!(config.organiclever_be_port, 8299_u16);
    }

    #[test]
    fn prefixed_cors_origins_key_resolves_to_cors_value() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/organiclever"),
            ("ORGANICLEVER_BE_CORS_ORIGINS", "https://example.com"),
        ])
        .expect("valid config");
        assert_eq!(config.organiclever_be_cors_origins, "https://example.com");
    }

    #[test]
    fn default_port_is_8202() {
        let config = from_pairs(&[("DATABASE_URL", "postgres://localhost/organiclever")])
            .expect("valid config");
        assert_eq!(config.organiclever_be_port, 8202);
    }

    #[test]
    fn custom_port_resolves() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/organiclever"),
            ("ORGANICLEVER_BE_PORT", "9090"),
        ])
        .expect("valid config");
        assert_eq!(config.organiclever_be_port, 9090);
    }

    #[test]
    fn custom_database_url_resolves() {
        let config = from_pairs(&[("DATABASE_URL", "postgres://user:pass@host/mydb")])
            .expect("valid config");
        assert_eq!(config.database_url, "postgres://user:pass@host/mydb");
    }

    #[test]
    fn default_cors_origins_is_wildcard() {
        let config = from_pairs(&[("DATABASE_URL", "postgres://localhost/organiclever")])
            .expect("valid config");
        assert_eq!(config.organiclever_be_cors_origins, "*");
    }

    #[test]
    fn custom_cors_origins_resolves() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/organiclever"),
            ("ORGANICLEVER_BE_CORS_ORIGINS", "https://example.com"),
        ])
        .expect("valid config");
        assert_eq!(config.organiclever_be_cors_origins, "https://example.com");
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
