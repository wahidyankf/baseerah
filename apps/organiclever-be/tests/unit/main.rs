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
            ("ORGANICLEVER_BE_NATS_URL", "nats://localhost:4222"),
            ("ORGANICLEVER_BE_CRANE_URL", "http://localhost:8300"),
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
            ("ORGANICLEVER_BE_NATS_URL", "nats://localhost:4222"),
            ("ORGANICLEVER_BE_CRANE_URL", "http://localhost:8300"),
        ])
        .expect("valid config");
        assert_eq!(config.organiclever_be_port, 8299_u16);
    }

    #[test]
    fn prefixed_cors_origins_key_resolves_to_cors_value() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/organiclever"),
            ("ORGANICLEVER_BE_CORS_ORIGINS", "https://example.com"),
            ("ORGANICLEVER_BE_NATS_URL", "nats://localhost:4222"),
            ("ORGANICLEVER_BE_CRANE_URL", "http://localhost:8300"),
        ])
        .expect("valid config");
        assert_eq!(config.organiclever_be_cors_origins, "https://example.com");
    }

    #[test]
    fn default_port_is_8202() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/organiclever"),
            ("ORGANICLEVER_BE_NATS_URL", "nats://localhost:4222"),
            ("ORGANICLEVER_BE_CRANE_URL", "http://localhost:8300"),
        ])
        .expect("valid config");
        assert_eq!(config.organiclever_be_port, 8202);
    }

    #[test]
    fn custom_port_resolves() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/organiclever"),
            ("ORGANICLEVER_BE_PORT", "9090"),
            ("ORGANICLEVER_BE_NATS_URL", "nats://localhost:4222"),
            ("ORGANICLEVER_BE_CRANE_URL", "http://localhost:8300"),
        ])
        .expect("valid config");
        assert_eq!(config.organiclever_be_port, 9090);
    }

    #[test]
    fn custom_database_url_resolves() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://user:pass@host/mydb"),
            ("ORGANICLEVER_BE_NATS_URL", "nats://localhost:4222"),
            ("ORGANICLEVER_BE_CRANE_URL", "http://localhost:8300"),
        ])
        .expect("valid config");
        assert_eq!(config.database_url, "postgres://user:pass@host/mydb");
    }

    #[test]
    fn default_cors_origins_is_wildcard() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/organiclever"),
            ("ORGANICLEVER_BE_NATS_URL", "nats://localhost:4222"),
            ("ORGANICLEVER_BE_CRANE_URL", "http://localhost:8300"),
        ])
        .expect("valid config");
        assert_eq!(config.organiclever_be_cors_origins, "*");
    }

    #[test]
    fn custom_cors_origins_resolves() {
        let config = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/organiclever"),
            ("ORGANICLEVER_BE_CORS_ORIGINS", "https://example.com"),
            ("ORGANICLEVER_BE_NATS_URL", "nats://localhost:4222"),
            ("ORGANICLEVER_BE_CRANE_URL", "http://localhost:8300"),
        ])
        .expect("valid config");
        assert_eq!(config.organiclever_be_cors_origins, "https://example.com");
    }
}

mod messaging_config_tests {
    use organiclever_be::config::Config;

    fn from_pairs(pairs: &[(&str, &str)]) -> Result<Config, envy::Error> {
        envy::from_iter::<_, Config>(pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())))
    }

    #[test]
    fn load_fails_when_nats_url_absent() {
        let result = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/organiclever"),
            ("ORGANICLEVER_BE_CRANE_URL", "http://localhost:8300"),
        ]);
        assert!(
            result.is_err(),
            "Config must fail when ORGANICLEVER_BE_NATS_URL is absent"
        );
    }

    #[test]
    fn load_fails_when_crane_url_absent() {
        let result = from_pairs(&[
            ("DATABASE_URL", "postgres://localhost/organiclever"),
            ("ORGANICLEVER_BE_NATS_URL", "nats://localhost:4222"),
        ]);
        assert!(
            result.is_err(),
            "Config must fail when ORGANICLEVER_BE_CRANE_URL is absent"
        );
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
    use organiclever_be::app::router;

    #[tokio::test]
    async fn test_app_router_signature_compiles() {
        // Verify the router constructor signature is visible and callable.
        // Cannot instantiate AppState without a live NATS client; the full
        // runtime path is covered by e2e tests. This test confirms the
        // module compiles and the function symbol exists.
        let _ = std::hint::black_box(router as fn(_) -> _);
    }
}

mod messaging_status_tests {
    use organiclever_be::contexts::messaging::status;

    #[test]
    fn new_shared_creates_default_status() {
        let shared = status::new_shared();
        let locked = shared.try_lock().expect("should not be contended");
        assert!(
            locked.jetstream_demo.is_none(),
            "new status must have no demo outcome"
        );
    }

    #[tokio::test]
    async fn shared_status_can_be_updated() {
        let shared = status::new_shared();
        {
            let mut s = shared.lock().await;
            s.jetstream_demo = Some("delivered_and_acked".to_string());
        }
        let s = shared.lock().await;
        assert_eq!(
            s.jetstream_demo.as_deref(),
            Some("delivered_and_acked"),
            "status should reflect the update"
        );
    }
}

mod messaging_status_handler_tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use organiclever_be::app::{AppState, router};
    use organiclever_be::contexts::messaging::status as messaging_status;
    use tower::ServiceExt as _;

    /// Build a test `AppState` with no NATS client and the given demo outcome.
    fn make_app_state(demo: Option<&str>) -> AppState {
        let shared = messaging_status::new_shared();
        if let Some(outcome) = demo {
            // Blocking lock is safe in test context (single-threaded here).
            shared.try_lock().expect("unlocked").jetstream_demo = Some(outcome.to_string());
        }
        AppState {
            nats: None,
            messaging_status: shared,
        }
    }

    #[tokio::test]
    async fn messaging_status_returns_pending_when_unset() {
        let app = router(make_app_state(None));
        let req = Request::builder()
            .uri("/api/v1/system/status/messaging")
            .body(Body::empty())
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .expect("body");
        let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
        assert_eq!(json["jetstream_demo"], "pending");
    }

    #[tokio::test]
    async fn messaging_status_returns_delivered_and_acked() {
        let app = router(make_app_state(Some("delivered_and_acked")));
        let req = Request::builder()
            .uri("/api/v1/system/status/messaging")
            .body(Body::empty())
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .expect("body");
        let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
        assert_eq!(json["jetstream_demo"], "delivered_and_acked");
    }

    #[tokio::test]
    async fn media_convert_returns_503_without_nats() {
        let app = router(make_app_state(None));
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/media/convert")
            .header("Content-Type", "application/octet-stream")
            .body(Body::from(b"fake pdf bytes".as_ref()))
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
