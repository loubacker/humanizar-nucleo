#![forbid(unsafe_code)]

use async_trait::async_trait;
use axum::body::Body;
use axum::http::Request;
use http_body_util::BodyExt;
use humanizar_nucleo::application::service::{AbordagemService, NucleoService};
use humanizar_nucleo::domain::exception::NucleoException;
use humanizar_nucleo::domain::model::Nucleo;
use humanizar_nucleo::domain::model::enums::ReasonCode;
use humanizar_nucleo::domain::port::NucleoPort;
use humanizar_nucleo::infrastructure::controller::{ApplicationState, create_router_with_limits};
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;
use tracing::instrument::WithSubscriber;
use uuid::Uuid;

#[path = "../../support/in_memory_ports.rs"]
mod in_memory_ports;

static TEST_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
#[path = "../../support/security_fixture.rs"]
mod security_fixture;
#[path = "../../support/tracing_capture.rs"]
mod tracing_capture;

#[derive(Default)]
struct FailurePort {
    delay: std::time::Duration,
}

#[async_trait]
impl NucleoPort for FailurePort {
    async fn create(&self, _: Nucleo) -> Result<Nucleo, NucleoException> {
        Err(NucleoException::new(ReasonCode::UnexpectedError))
    }

    async fn update(&self, _: Nucleo) -> Result<Nucleo, NucleoException> {
        Err(NucleoException::new(ReasonCode::UnexpectedError))
    }

    async fn find_by_id(&self, _: Uuid) -> Result<Option<Nucleo>, NucleoException> {
        Ok(None)
    }

    async fn find_all(&self) -> Result<Vec<Nucleo>, NucleoException> {
        tokio::time::sleep(self.delay).await;
        Err(NucleoException::with_source(
            ReasonCode::DatabaseAccessFailed,
            std::io::Error::other(
                "password_sentinel Bearer_sentinel CPF_sentinel email_sentinel SQL_sentinel",
            ),
        ))
    }

    async fn delete(&self, _: Uuid) -> Result<(), NucleoException> {
        Ok(())
    }
}

#[tokio::test]
async fn dto_codes_correlation_and_phase_are_logged_without_sensitive_context() {
    let _guard = TEST_LOCK.lock().await;
    let capture = tracing_capture::TracingCapture::default();
    let subscriber = tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .with_writer(capture.clone())
        .finish();
    async {
        let server = security_fixture::JwksServer::start("key-1").await;
        let security = security_fixture::security_config(&server).await;
        let ports = Arc::new(in_memory_ports::InMemoryPorts::default());
        let router = create_router_with_limits(
            ports.state(),
            &security,
            1024,
            std::time::Duration::from_secs(5),
        );
        let token = security_fixture::rsa_token("key-1", &security_fixture::TestClaims::valid());
        let correlation = "929ad13a-2b78-490c-ab6d-247cd09e97e4";
        let request = Request::builder()
            .method("POST")
            .uri("/api/v1/nucleo/register?email=email_sentinel&password=password_sentinel")
            .header("authorization", format!("Bearer {token}"))
            .header("x-correlation-id", correlation)
            .header("content-type", "application/json")
            .body(Body::from(
                "{\"nome\":\"CPF_sentinel\",\"descricao\":\"body_sentinel\"}",
            ))
            .expect("request");
        let response = router.clone().oneshot(request).await.expect("response");
        assert_eq!(response.status().as_u16(), 201);
        let data: Value = serde_json::from_slice(
            &response
                .into_body()
                .collect()
                .await
                .expect("body")
                .to_bytes(),
        )
        .expect("JSON");
        let code = data["code"].as_str().expect("code");
        let log = capture.text();
        assert!(log.contains(code));
        assert!(log.contains(correlation));
        assert!(log.contains("http_complete"));
        assert!(log.contains("status=201"));
        let failures: Arc<dyn NucleoPort> = Arc::new(FailurePort::default());
        let state = ApplicationState::new(
            Arc::new(NucleoService::new(failures.clone(), ports.clone())),
            Arc::new(AbordagemService::new(ports, failures)),
        );
        let failing_router =
            create_router_with_limits(state, &security, 1024, std::time::Duration::from_secs(5));
        let request = Request::builder()
            .uri("/api/v1/nucleos?secret=SQL_sentinel")
            .header("authorization", format!("Bearer {token}"))
            .header("x-correlation-id", correlation)
            .body(Body::empty())
            .expect("request");
        let response = failing_router.oneshot(request).await.expect("response");
        assert_eq!(response.status().as_u16(), 500);
        let error: Value = serde_json::from_slice(
            &response
                .into_body()
                .collect()
                .await
                .expect("body")
                .to_bytes(),
        )
        .expect("JSON");
        assert_eq!(error["reasonCode"], "DATABASE_ACCESS_FAILED");
        let log = capture.text();
        assert!(log.contains("DATABASE_ACCESS_FAILED"));
        assert!(log.contains("source_present=true"));
        for sentinel in [
            "password_sentinel",
            "Bearer_sentinel",
            "CPF_sentinel",
            "email_sentinel",
            "SQL_sentinel",
            "body_sentinel",
            token.as_str(),
        ] {
            assert!(!log.contains(sentinel), "sentinel apareceu no tracing");
        }
        let before = capture.text().len();
        let response = router
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("health request"),
            )
            .await
            .expect("health");
        assert_eq!(response.status().as_u16(), 200);
        assert_eq!(capture.text().len(), before);
    }
    .with_subscriber(subscriber)
    .await;
}

#[tokio::test]
async fn timeout_has_controlled_error_correlation_and_cors() {
    let _guard = TEST_LOCK.lock().await;
    let server = security_fixture::JwksServer::start("key-1").await;
    let security = security_fixture::security_config(&server).await;
    let ports = Arc::new(in_memory_ports::InMemoryPorts::default());
    let failures: Arc<dyn NucleoPort> = Arc::new(FailurePort {
        delay: std::time::Duration::from_secs(1),
    });
    let state = ApplicationState::new(
        Arc::new(NucleoService::new(failures.clone(), ports.clone())),
        Arc::new(AbordagemService::new(ports, failures)),
    );
    let router =
        create_router_with_limits(state, &security, 1024, std::time::Duration::from_millis(50));
    let token = security_fixture::rsa_token("key-1", &security_fixture::TestClaims::valid());
    let response = router
        .oneshot(
            Request::builder()
                .uri("/api/v1/nucleos")
                .header("authorization", format!("Bearer {token}"))
                .header("origin", "http://localhost:3000")
                .header("x-correlation-id", "929ad13a-2b78-490c-ab6d-247cd09e97e4")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(response.status().as_u16(), 504);
    assert_eq!(
        response.headers()["access-control-allow-origin"],
        "http://localhost:3000"
    );
    let correlation = response.headers()["x-correlation-id"]
        .to_str()
        .expect("header")
        .to_owned();
    let error: Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes(),
    )
    .expect("JSON");
    assert_eq!(error["reasonCode"], "REQUEST_TIMEOUT");
    assert_eq!(error["correlationId"], correlation);
}
