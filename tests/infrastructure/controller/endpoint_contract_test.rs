#![forbid(unsafe_code)]

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use humanizar_nucleo::infrastructure::controller::{create_router, create_router_with_limits};
use in_memory_ports::InMemoryPorts;
use security_fixture::{JwksServer, TestClaims, rsa_token, security_config};
use serde_json::{Value, json};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tower::ServiceExt;
use uuid::Uuid;

async fn request(
    router: &Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: Value,
) -> (StatusCode, Value, String) {
    let mut builder = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json")
        .header("x-correlation-id", "929ad13a-2b78-490c-ab6d-247cd09e97e4");
    if let Some(token) = token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    let response = router
        .clone()
        .oneshot(builder.body(Body::from(body.to_string())).expect("request"))
        .await
        .expect("response");
    let status = response.status();
    let correlation = response.headers()["x-correlation-id"]
        .to_str()
        .expect("header")
        .to_owned();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).expect("JSON contract"),
        correlation,
    )
}

#[path = "../../support/in_memory_ports.rs"]
mod in_memory_ports;
#[path = "../../support/security_fixture.rs"]
mod security_fixture;

#[tokio::test]
async fn crud_envelopes_preserve_identity_parent_and_created_at() {
    let server = JwksServer::start("key-1").await;
    let security = security_config(&server).await;
    let ports = Arc::new(InMemoryPorts::default());
    let router = create_router(ports.state(), &security);
    let token = rsa_token("key-1", &TestClaims::valid());
    let (status, created, correlation) = request(
        &router,
        "POST",
        "/api/v1/nucleo/register",
        Some(&token),
        json!({"nome":"Motor", "descricao":"Descrição", "id":Uuid::nil()}),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["code"], "NUCLEO_CREATED");
    assert_eq!(created["correlationId"], correlation);
    assert_eq!(created["success"], true);
    assert_eq!(created.as_object().expect("object").len(), 6);
    let id = created["data"]["id"].as_str().expect("id");
    assert_ne!(id, Uuid::nil().to_string());
    let created_at = created["data"]["createdAt"].clone();
    let (_, abordagem, _) = request(
        &router,
        "POST",
        "/api/v1/abordagem/register",
        Some(&token),
        json!({"nome":"ABA", "descricao":"D", "nucleoId":id}),
    )
    .await;
    let abordagem_id = abordagem["data"]["id"].as_str().expect("id");
    let (_, updated, _) = request(&router, "PUT", &format!("/api/v1/nucleo/update/{id}"), Some(&token), json!({"nome":"Atualizado", "descricao":"D", "id":Uuid::nil(), "createdAt":"2000-01-01T00:00:00"})).await;
    assert_eq!(updated["data"]["id"], id);
    assert_eq!(updated["data"]["createdAt"], created_at);
    assert_eq!(updated["data"]["abordagens"][0]["id"], abordagem_id);
    let (_, updated_abordagem, _) = request(
        &router,
        "PUT",
        &format!("/api/v1/abordagem/update/{abordagem_id}"),
        Some(&token),
        json!({"nome":"Atualizada", "descricao":"D", "nucleoId":Uuid::nil()}),
    )
    .await;
    assert_eq!(updated_abordagem["data"]["nucleoId"], id);
    for path in [
        format!("/api/v1/nucleo/{id}"),
        format!("/api/v1/nucleos/{id}"),
    ] {
        let (status, body, _) = request(&router, "GET", &path, Some(&token), Value::Null).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["data"]["id"], id);
    }
    let reads = ports.batch_reads.load(Ordering::SeqCst);
    let (_, listed, _) = request(
        &router,
        "GET",
        "/api/v1/nucleos/abordagens",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(
        listed["data"][0]["abordagens"]
            .as_array()
            .expect("children")
            .len(),
        1
    );
    assert_eq!(ports.batch_reads.load(Ordering::SeqCst), reads + 1);
    let (status, blocked, _) = request(
        &router,
        "DELETE",
        &format!("/api/v1/nucleo/delete/{id}"),
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(blocked["reasonCode"], "NUCLEO_HAS_ABORDAGENS");
    for path in [
        format!("/api/v1/abordagem/delete/{abordagem_id}"),
        format!("/api/v1/nucleo/delete/{id}"),
    ] {
        let (status, deleted, _) =
            request(&router, "DELETE", &path, Some(&token), Value::Null).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(deleted["data"], Value::Null);
        assert_eq!(deleted["success"], true);
    }
}

#[tokio::test]
async fn every_transport_failure_has_error_dto_and_same_correlation() {
    let server = JwksServer::start("key-1").await;
    let ports = Arc::new(InMemoryPorts::default());
    let security = security_config(&server).await;
    let router = create_router_with_limits(
        ports.state(),
        &security,
        256,
        std::time::Duration::from_secs(5),
    );
    let token = rsa_token("key-1", &TestClaims::valid());
    let cases = [
        (
            "GET",
            "/api/v1/nucleos",
            None,
            Value::Null,
            401,
            "AUTHENTICATION_REQUIRED",
        ),
        (
            "POST",
            "/api/v1/nucleo/register",
            Some(token.as_str()),
            Value::Null,
            400,
            "PAYLOAD_REQUIRED",
        ),
        (
            "POST",
            "/api/v1/nucleo/register",
            Some(token.as_str()),
            json!({}),
            400,
            "NOME_REQUIRED",
        ),
        (
            "POST",
            "/api/v1/nucleo/register",
            Some(token.as_str()),
            json!({"nome":"x"}),
            400,
            "DESCRICAO_REQUIRED",
        ),
        (
            "GET",
            "/api/v1/nucleo/bad-uuid",
            Some(token.as_str()),
            Value::Null,
            400,
            "REQUEST_PARAMETER_INVALID",
        ),
        (
            "GET",
            "/missing?password=sentinel",
            None,
            Value::Null,
            404,
            "ROUTE_NOT_FOUND",
        ),
        (
            "PATCH",
            "/api/v1/nucleos",
            Some(token.as_str()),
            Value::Null,
            405,
            "METHOD_NOT_ALLOWED",
        ),
        (
            "POST",
            "/api/v1/nucleo/register",
            Some(token.as_str()),
            json!({"nome":"x".repeat(1024)}),
            413,
            "PAYLOAD_TOO_LARGE",
        ),
    ];
    for (method, path, credential, body, status, reason) in cases {
        let (actual, error, correlation) = request(&router, method, path, credential, body).await;
        assert_eq!(actual.as_u16(), status, "{path}");
        assert_eq!(error["reasonCode"], reason);
        assert_eq!(error["correlationId"], correlation);
        assert_eq!(error.as_object().expect("error").len(), 7);
        assert!(!error["path"].as_str().expect("path").contains("password"));
    }
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/health")
                .header("x-correlation-id", "not-uuid")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        Uuid::parse_str(
            response.headers()["x-correlation-id"]
                .to_str()
                .expect("header")
        )
        .is_ok()
    );
    let (status, _, _) = request(&router, "GET", "/actuator/health", None, Value::Null).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(ports.effects.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn legacy_roles_are_the_only_source_of_admin_authority() {
    let server = JwksServer::start("key-1").await;
    let ports = Arc::new(InMemoryPorts::default());
    let router = create_router(ports.state(), &security_config(&server).await);
    for realm in [
        None,
        Some(json!(null)),
        Some(json!("ADMINISTRADOR")),
        Some(json!({"roles":"ADMINISTRADOR"})),
        Some(json!({"roles":["administrador", "ADMINISTRADOR,OTHER", 1]})),
    ] {
        let mut claims = TestClaims::valid();
        claims.realm_access = realm;
        claims.role = Some(json!("ADMINISTRADOR"));
        claims.roles = Some(json!(["ADMINISTRADOR"]));
        let token = rsa_token("key-1", &claims);
        let (status, _, _) =
            request(&router, "GET", "/api/v1/nucleos", Some(&token), Value::Null).await;
        assert_eq!(status, StatusCode::OK);
        let (status, error, _) = request(
            &router,
            "POST",
            "/api/v1/nucleo/register",
            Some(&token),
            json!({"nome":"x", "descricao":"D"}),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(error["reasonCode"], "AUTHORIZATION_DENIED");
    }
    assert_eq!(ports.effects.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn invalid_signature_algorithm_and_standard_claims_are_rejected() {
    let server = JwksServer::start("key-1").await;
    let ports = Arc::new(InMemoryPorts::default());
    let router = create_router(ports.state(), &security_config(&server).await);
    let mut cases = Vec::new();
    for invalid in 0..5 {
        let mut claims = TestClaims::valid();
        match invalid {
            0 => claims.iss = "wrong".to_owned(),
            1 => claims.aud = json!("wrong"),
            2 => claims.exp = 1,
            3 => claims.nbf = Some(security_fixture::now_for_tests() + 3600),
            4 => claims.sub = None,
            _ => unreachable!(),
        }
        cases.push(rsa_token("key-1", &claims));
    }
    cases.push(security_fixture::hmac_token("key-1", &TestClaims::valid()));
    cases.push("invalid.signature.token".to_owned());
    cases.push(rsa_token("unknown-key", &TestClaims::valid()));
    for token in cases {
        let (status, error, _) =
            request(&router, "GET", "/api/v1/nucleos", Some(&token), Value::Null).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(error["reasonCode"], "AUTHENTICATION_REQUIRED");
    }
}

#[tokio::test]
async fn cors_is_preserved_on_rejections_and_preflight_is_public() {
    let server = JwksServer::start("key-1").await;
    let ports = Arc::new(InMemoryPorts::default());
    let router = create_router(ports.state(), &security_config(&server).await);
    for path in ["/missing", "/api/v1/nucleos"] {
        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .header("origin", "http://localhost:3000")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(
            response.headers()["access-control-allow-origin"],
            "http://localhost:3000"
        );
        assert_eq!(
            response.headers()["access-control-allow-credentials"],
            "true"
        );
    }
    let response = router
        .oneshot(
            Request::builder()
                .method("OPTIONS")
                .uri("/api/v1/nucleos")
                .header("origin", "http://10.1.2.3:3000")
                .header("access-control-request-method", "GET")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(ports.effects.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn jwks_rotation_removes_old_keys_and_expired_cache_is_not_used_on_refresh_failure() {
    let server = JwksServer::start("key-1").await;
    let settings = humanizar_nucleo::infrastructure::config::SecuritySettings::new(
        server.jwks_url(),
        security_fixture::ISSUER,
        security_fixture::AUDIENCE,
    )
    .expect("settings")
    .with_cache_policy(
        std::time::Duration::from_millis(5),
        std::time::Duration::from_millis(1),
    );
    let security = humanizar_nucleo::infrastructure::config::SecurityConfig::initialize(settings)
        .await
        .expect("JWKS");
    let ports = Arc::new(InMemoryPorts::default());
    let router = create_router(ports.state(), &security);
    let first = rsa_token("key-1", &TestClaims::valid());
    assert_eq!(
        request(&router, "GET", "/api/v1/nucleos", Some(&first), Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    server.replace_key("key-2").await;
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    let second = rsa_token("key-2", &TestClaims::valid());
    assert_eq!(
        request(
            &router,
            "GET",
            "/api/v1/nucleos",
            Some(&second),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(&router, "GET", "/api/v1/nucleos", Some(&first), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    server.replace_document(json!({"keys":[]})).await;
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    assert_eq!(
        request(
            &router,
            "GET",
            "/api/v1/nucleos",
            Some(&second),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(ports.effects.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn jwks_redirect_and_oversized_document_fail_initialization() {
    let valid = JwksServer::start("key-1").await;
    let redirect = valid.jwks_url().to_owned();
    let app = axum::Router::new()
        .route(
            "/redirect",
            axum::routing::get(move || {
                let url = redirect.clone();
                async move { axum::response::Redirect::temporary(&url) }
            }),
        )
        .route(
            "/oversized",
            axum::routing::get(|| async { "x".repeat(1024 * 1024 + 1) }),
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener");
    let address = listener.local_addr().expect("address");
    let task =
        tokio::spawn(async move { axum::serve(listener, app).await.expect("fixture server") });
    for path in ["redirect", "oversized"] {
        let settings = humanizar_nucleo::infrastructure::config::SecuritySettings::new(
            format!("http://{address}/{path}"),
            security_fixture::ISSUER,
            security_fixture::AUDIENCE,
        )
        .expect("settings");
        assert!(
            humanizar_nucleo::infrastructure::config::SecurityConfig::initialize(settings)
                .await
                .is_err()
        );
    }
    task.abort();
}
