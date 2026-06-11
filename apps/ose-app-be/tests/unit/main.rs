//! Unit tests for ose-app-be library.

mod config_tests {
    use ose_app_be::config::Config;

    fn from_pairs(pairs: &[(&str, &str)]) -> Result<Config, envy::Error> {
        envy::from_iter::<_, Config>(pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())))
    }

    /// Minimal valid config pairs (all required fields).
    fn min_valid() -> Vec<(&'static str, &'static str)> {
        vec![
            ("DATABASE_URL", "postgres://localhost/ose_app"),
            ("OSE_APP_BE_NATS_URL", "nats://localhost:4222"),
            ("OSE_APP_BE_CRANE_URL", "http://localhost:8300"),
        ]
    }

    // RED (item 68): Config must fail fast when DATABASE_URL is absent.
    #[test]
    fn load_fails_when_database_url_absent() {
        let result = from_pairs(&[
            ("OSE_APP_BE_PORT", "8302"),
            ("OSE_APP_BE_CORS_ORIGINS", "*"),
            ("OSE_APP_BE_NATS_URL", "nats://localhost:4222"),
            ("OSE_APP_BE_CRANE_URL", "http://localhost:8300"),
        ]);
        assert!(
            result.is_err(),
            "Config deserialization must fail when DATABASE_URL is absent"
        );
    }

    #[test]
    fn prefixed_port_key_resolves_to_port_value() {
        let mut pairs = min_valid();
        pairs.push(("OSE_APP_BE_PORT", "8399"));
        let config = from_pairs(&pairs).expect("valid config");
        assert_eq!(config.ose_app_be_port, 8399_u16);
    }

    #[test]
    fn prefixed_cors_origins_key_resolves_to_cors_value() {
        let mut pairs = min_valid();
        pairs.push(("OSE_APP_BE_CORS_ORIGINS", "https://app.oseplatform.com"));
        let config = from_pairs(&pairs).expect("valid config");
        assert_eq!(
            config.ose_app_be_cors_origins,
            "https://app.oseplatform.com"
        );
    }

    #[test]
    fn prefixed_openrouter_api_key_resolves_to_key_value() {
        let mut pairs = min_valid();
        pairs.push(("OSE_APP_BE_OPENROUTER_API_KEY", "sk-test-key"));
        let config = from_pairs(&pairs).expect("valid config");
        assert_eq!(config.ose_app_be_openrouter_api_key, "sk-test-key");
    }

    #[test]
    fn prefixed_openrouter_model_resolves_to_model_value() {
        let mut pairs = min_valid();
        pairs.push(("OSE_APP_BE_OPENROUTER_MODEL", "gpt-4o"));
        let config = from_pairs(&pairs).expect("valid config");
        assert_eq!(config.ose_app_be_openrouter_model, "gpt-4o");
    }

    #[test]
    fn prefixed_openrouter_base_url_resolves_to_url_value() {
        let mut pairs = min_valid();
        pairs.push((
            "OSE_APP_BE_OPENROUTER_BASE_URL",
            "https://custom.openrouter.ai/api/v1",
        ));
        let config = from_pairs(&pairs).expect("valid config");
        assert_eq!(
            config.ose_app_be_openrouter_base_url,
            "https://custom.openrouter.ai/api/v1"
        );
    }

    #[test]
    fn default_port_is_8302() {
        let config = from_pairs(&min_valid()).expect("valid config");
        assert_eq!(config.ose_app_be_port, 8302);
    }

    #[test]
    fn custom_port_resolves() {
        let mut pairs = min_valid();
        pairs.push(("OSE_APP_BE_PORT", "9090"));
        let config = from_pairs(&pairs).expect("valid config");
        assert_eq!(config.ose_app_be_port, 9090);
    }

    #[test]
    fn custom_database_url_resolves() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://user:pass@host/mydb"),
            ("OSE_APP_BE_NATS_URL", "nats://localhost:4222"),
            ("OSE_APP_BE_CRANE_URL", "http://localhost:8300"),
        ])
        .expect("valid config");
        assert_eq!(config.database_url, "postgres://user:pass@host/mydb");
    }

    #[test]
    fn default_cors_origins_is_wildcard() {
        let config = from_pairs(&min_valid()).expect("valid config");
        assert_eq!(config.ose_app_be_cors_origins, "*");
    }

    #[test]
    fn default_openrouter_model_is_auto() {
        let config = from_pairs(&min_valid()).expect("valid config");
        assert_eq!(config.ose_app_be_openrouter_model, "openrouter/auto");
    }

    #[test]
    fn custom_openrouter_fields_resolve() {
        let mut pairs = min_valid();
        pairs.extend_from_slice(&[
            ("OSE_APP_BE_OPENROUTER_API_KEY", "key"),
            ("OSE_APP_BE_OPENROUTER_MODEL", "gpt-4"),
            ("OSE_APP_BE_OPENROUTER_BASE_URL", "https://api.example.com"),
        ]);
        let config = from_pairs(&pairs).expect("valid config");
        assert_eq!(config.ose_app_be_openrouter_model, "gpt-4");
        assert_eq!(config.ose_app_be_openrouter_api_key, "key");
        assert_eq!(
            config.ose_app_be_openrouter_base_url,
            "https://api.example.com"
        );
    }
}

mod messaging_config_tests {
    use ose_app_be::config::Config;

    fn from_pairs(pairs: &[(&str, &str)]) -> Result<Config, envy::Error> {
        envy::from_iter::<_, Config>(pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())))
    }

    #[test]
    fn load_fails_when_nats_url_absent() {
        let result = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/ose"),
            ("OSE_APP_BE_CRANE_URL", "http://localhost:8300"),
        ]);
        assert!(
            result.is_err(),
            "Config must fail when OSE_APP_BE_NATS_URL is absent"
        );
    }

    #[test]
    fn load_fails_when_crane_url_absent() {
        let result = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/ose"),
            ("OSE_APP_BE_NATS_URL", "nats://localhost:4222"),
        ]);
        assert!(
            result.is_err(),
            "Config must fail when OSE_APP_BE_CRANE_URL is absent"
        );
    }

    #[test]
    fn nats_url_resolves() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/ose"),
            ("OSE_APP_BE_NATS_URL", "nats://nats:4222"),
            ("OSE_APP_BE_CRANE_URL", "http://crane:8300"),
        ])
        .expect("valid config");
        assert_eq!(config.ose_app_be_nats_url, "nats://nats:4222");
    }

    #[test]
    fn crane_url_resolves() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/ose"),
            ("OSE_APP_BE_NATS_URL", "nats://nats:4222"),
            ("OSE_APP_BE_CRANE_URL", "http://crane:8300"),
        ])
        .expect("valid config");
        assert_eq!(config.ose_app_be_crane_url, "http://crane:8300");
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
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use ose_app_be::app::{self, AppState};
    use ose_app_be::contexts::messaging::status;
    use tower::ServiceExt as _;

    fn make_state() -> AppState {
        AppState {
            nats: None,
            messaging_status: status::new_shared(),
        }
    }

    #[tokio::test]
    async fn test_app_router_compiles() {
        let _ = app::router(make_state());
    }

    #[tokio::test]
    async fn messaging_status_returns_pending_when_no_demo_run() {
        let router = app::router(make_state());
        let req = Request::builder()
            .uri("/api/v1/system/status/messaging")
            .body(Body::empty())
            .expect("build request");
        let resp = router.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), 1024)
            .await
            .expect("body bytes");
        let json: serde_json::Value = serde_json::from_slice(&body).expect("parse JSON");
        assert_eq!(json["jetstream_demo"], "pending");
    }

    #[tokio::test]
    async fn messaging_status_returns_stored_value() {
        let shared = status::new_shared();
        {
            let mut s = shared.lock().await;
            s.jetstream_demo = Some("delivered_and_acked".to_string());
        }
        let app_state = AppState {
            nats: None,
            messaging_status: shared,
        };
        let router = app::router(app_state);
        let req = Request::builder()
            .uri("/api/v1/system/status/messaging")
            .body(Body::empty())
            .expect("build request");
        let resp = router.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), 1024)
            .await
            .expect("body bytes");
        let json: serde_json::Value = serde_json::from_slice(&body).expect("parse JSON");
        assert_eq!(json["jetstream_demo"], "delivered_and_acked");
    }

    #[tokio::test]
    async fn media_convert_returns_503_when_nats_is_none() {
        let router = app::router(make_state());
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/media/convert")
            .body(Body::from(b"%PDF-1.4 fake".as_slice()))
            .expect("build request");
        let resp = router.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}

mod messaging_status_tests {
    use ose_app_be::contexts::messaging::status;

    #[tokio::test]
    async fn new_shared_has_none_jetstream_demo() {
        let shared = status::new_shared();
        let s = shared.lock().await;
        assert!(s.jetstream_demo.is_none());
    }

    #[tokio::test]
    async fn can_set_and_read_jetstream_demo() {
        let shared = status::new_shared();
        {
            let mut s = shared.lock().await;
            s.jetstream_demo = Some("delivered_and_acked".to_string());
        }
        let s = shared.lock().await;
        assert_eq!(s.jetstream_demo.as_deref(), Some("delivered_and_acked"));
    }

    #[tokio::test]
    async fn default_status_has_none_jetstream_demo() {
        let s = status::MessagingStatus::default();
        assert!(s.jetstream_demo.is_none());
    }
}
