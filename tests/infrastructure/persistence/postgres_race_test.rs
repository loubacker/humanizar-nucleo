#![forbid(unsafe_code)]

use axum::body::Body;
use axum::http::Request;
use http_body_util::BodyExt;
use humanizar_nucleo::domain::model::messaging::NucleoRetrieveCommand;
use humanizar_nucleo::infrastructure::controller::{ApplicationState, create_router};
use humanizar_nucleo::infrastructure::messaging::MessagingServices;
use postgres_fixture::PostgresFixture;
use serde_json::{Value, json};
use std::process::{Command, Stdio};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::{Duration, Instant};
use tower::ServiceExt;
use tracing::instrument::WithSubscriber;
use uuid::Uuid;

#[path = "../../support/postgres_fixture.rs"]
mod postgres_fixture;
#[path = "../../support/security_fixture.rs"]
mod security_fixture;
#[path = "../../support/tracing_capture.rs"]
mod tracing_capture;

static TEST_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

#[tokio::test]
async fn retention_preserves_active_intentions_and_schema_gate_detects_invalid_protection() {
    let _guard = TEST_LOCK.lock().await;
    let fixture = PostgresFixture::start().await;
    let services = MessagingServices::new(fixture.database.clone(), 5);
    let now = chrono::Local::now().naive_local();
    for index in 0..3 {
        let command = NucleoRetrieveCommand {
            event_id: Uuid::new_v4(),
            correlation_id: Uuid::new_v4(),
            source_event_id: Uuid::new_v4(),
            nucleo_id: Uuid::new_v4(),
            callback_exchange: "fixture.callbacks".to_owned(),
            actor_id: None,
            user_agent: None,
            origin_ip: None,
            occurred_at: now,
        };
        services
            .processed
            .process(command)
            .await
            .expect("persist intent");
        fixture
            .client
            .execute(
                "UPDATE outbox_event SET status = $1 WHERE id = $2",
                &[&["NEW", "PUBLISHED", "DEAD"][index], &(index as i64 + 1)],
            )
            .await
            .expect("states fixture");
    }
    fixture
        .client
        .execute(
            "UPDATE outbox_event SET created_at = $1",
            &[&(now - chrono::Duration::days(100))],
        )
        .await
        .expect("old intentions");
    fixture
        .client
        .execute(
            "UPDATE processed_event SET processed_at = $1",
            &[&(now - chrono::Duration::days(100))],
        )
        .await
        .expect("old receipts");
    assert_eq!(
        services
            .outbox
            .purge(
                now - chrono::Duration::hours(48),
                now - chrono::Duration::days(90)
            )
            .await
            .expect("purge"),
        4
    );
    assert_eq!(
        fixture
            .client
            .query_one(
                "SELECT COUNT(*) FROM outbox_event WHERE status = 'NEW'",
                &[]
            )
            .await
            .expect("active")
            .get::<_, i64>(0),
        1
    );
    assert_eq!(
        fixture
            .client
            .query_one("SELECT COUNT(*) FROM processed_event", &[])
            .await
            .expect("protected receipt")
            .get::<_, i64>(0),
        1
    );
    assert_startup_schema_result(&fixture, "Falha ao inicializar o cache JWKS").await;
    fixture.client.batch_execute("ALTER TABLE processed_event DROP CONSTRAINT uk_processed_consumer_event; CREATE INDEX uk_processed_consumer_event ON processed_event (consumer_name, event_id)").await.expect("invalid homonym fixture");
    assert_startup_schema_result(&fixture, "Constraints do schema incompativeis").await;
    fixture.client.batch_execute("DROP INDEX uk_processed_consumer_event; ALTER TABLE processed_event ADD CONSTRAINT uk_processed_consumer_event UNIQUE (consumer_name, event_id); ALTER TABLE abordagem DROP CONSTRAINT fk_abordagem_nucleo; ALTER TABLE abordagem ADD CONSTRAINT fk_abordagem_nucleo CHECK (true)").await.expect("invalid FK homonym");
    assert_startup_schema_result(&fixture, "Constraints do schema incompativeis").await;
    fixture.client.batch_execute("ALTER TABLE abordagem DROP CONSTRAINT fk_abordagem_nucleo; ALTER TABLE abordagem ADD CONSTRAINT fk_abordagem_nucleo FOREIGN KEY (nucleo_id) REFERENCES nucleo (id) ON DELETE RESTRICT; ALTER TABLE outbox_event DROP CONSTRAINT uk_outbox_event_id; CREATE INDEX uk_outbox_event_id ON outbox_event (event_id)").await.expect("invalid outbox homonym");
    assert_startup_schema_result(&fixture, "Constraints do schema incompativeis").await;
    fixture.close().await;
}

async fn assert_startup_schema_result(fixture: &PostgresFixture, expected_message: &str) {
    let schema: String = fixture
        .client
        .query_one("SELECT CURRENT_SCHEMA()", &[])
        .await
        .expect("fixture schema")
        .get(0);
    let mut database_url =
        url::Url::parse(&std::env::var("NUCLEO_TEST_DB_URL").expect("exclusive fixture URL"))
            .expect("fixture URL");
    database_url
        .query_pairs_mut()
        .append_pair("options", &format!("-c search_path={schema}"));
    let directory = std::env::temp_dir().join(format!(
        "nucleo-schema-startup-fixture-{}",
        Uuid::new_v4().simple()
    ));
    std::fs::create_dir(&directory).expect("empty fixture directory");
    let mut command = Command::new(env!("CARGO_BIN_EXE_humanizar-nucleo"));
    command
        .env_clear()
        .current_dir(&directory)
        .env("DB_URL", database_url.as_str().replace('+', "%20"))
        .env(
            "DB_USERNAME",
            std::env::var("NUCLEO_TEST_DB_USERNAME").expect("fixture username"),
        )
        .env(
            "DB_PASSWORD",
            std::env::var("NUCLEO_TEST_DB_PASSWORD").expect("fixture password"),
        )
        .env("DB_POOL_MIN_IDLE", "1")
        .env("DB_POOL_MAX_SIZE", "2")
        .env("DB_POOL_CONNECTION_TIMEOUT_SECONDS", "2")
        .env("KEYCLOAK_ISSUER", "http://127.0.0.1:1/jwks")
        .env("JWT_ISSUER", "https://issuer.fixture")
        .env("JWT_AUDIENCE", "nucleo-schema-fixture")
        .env("RABBITMQ_URL", "amqp://fixture:fixture@127.0.0.1:1/%2f")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for name in ["SystemRoot", "WINDIR"] {
        if let Some(value) = std::env::var_os(name) {
            command.env(name, value);
        }
    }
    let mut child = command.spawn().expect("service fixture process");
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut timed_out = false;
    while child.try_wait().expect("service fixture status").is_none() {
        if Instant::now() >= deadline {
            timed_out = true;
            child.kill().expect("stop timed out service fixture");
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    let output = child.wait_with_output().expect("service fixture output");
    std::fs::remove_dir(directory).expect("cleanup empty fixture directory");
    assert!(!timed_out, "service fixture exceeded its startup deadline");
    assert!(!output.status.success());
    let diagnostic = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(diagnostic.contains(expected_message), "{diagnostic}");
}

#[tokio::test]
async fn concurrent_delete_and_abordagem_creation_never_leave_an_orphan() {
    let _guard = TEST_LOCK.lock().await;
    let fixture = PostgresFixture::start().await;
    let server = security_fixture::JwksServer::start("key-1").await;
    let security = security_fixture::security_config(&server).await;
    let router = create_router(
        ApplicationState::from_database(fixture.database.clone(), Arc::new(AtomicBool::new(true))),
        &security,
    );
    let token = security_fixture::rsa_token("key-1", &security_fixture::TestClaims::valid());
    for _ in 0..10 {
        let id = Uuid::new_v4();
        fixture
            .client
            .execute(
                "INSERT INTO nucleo (id, nome, descricao) VALUES ($1, 'Parent', 'D')",
                &[&id],
            )
            .await
            .expect("parent");
        let delete = Request::builder()
            .method("DELETE")
            .uri(format!("/api/v1/nucleo/delete/{id}"))
            .header("authorization", format!("Bearer {token}"))
            .body(Body::empty())
            .expect("delete request");
        let create = Request::builder()
            .method("POST")
            .uri("/api/v1/abordagem/register")
            .header("authorization", format!("Bearer {token}"))
            .header("content-type", "application/json")
            .body(Body::from(
                json!({"nome":"Child", "descricao":"D", "nucleoId":id}).to_string(),
            ))
            .expect("create request");
        let (deleted, created) = tokio::join!(
            router.clone().oneshot(delete),
            router.clone().oneshot(create)
        );
        let statuses = (
            deleted.expect("delete").status().as_u16(),
            created.expect("create").status().as_u16(),
        );
        assert!(
            matches!(statuses, (409, 201) | (200, 404)),
            "statuses: {statuses:?}"
        );
        assert_eq!(fixture.client.query_one("SELECT COUNT(*) FROM abordagem AS a LEFT JOIN nucleo AS n ON n.id = a.nucleo_id WHERE n.id IS NULL", &[]).await.expect("integrity").get::<_, i64>(0), 0);
    }
    fixture.close().await;
}

#[tokio::test]
async fn health_failure_keeps_503_correlation_cors_and_visible_diagnostic() {
    let _guard = TEST_LOCK.lock().await;
    let fixture = PostgresFixture::start().await;
    let server = security_fixture::JwksServer::start("key-1").await;
    let security = security_fixture::security_config(&server).await;
    let capture = tracing_capture::TracingCapture::default();
    let subscriber = tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .with_writer(capture.clone())
        .finish();
    let readiness = Arc::new(AtomicBool::new(false));
    let router = create_router(
        ApplicationState::from_database(fixture.database.clone(), readiness.clone()),
        &security,
    );
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/health")
                .header("origin", "http://localhost:3000")
                .header("x-correlation-id", "929ad13a-2b78-490c-ab6d-247cd09e97e4")
                .body(Body::empty())
                .expect("request"),
        )
        .with_subscriber(subscriber)
        .await
        .expect("response");
    assert_eq!(response.status().as_u16(), 503);
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
    assert_eq!(error["reasonCode"], "MESSAGING_UNAVAILABLE");
    assert_eq!(error["correlationId"], correlation);
    assert!(capture.text().contains("status=503"));
    assert!(capture.text().contains("MESSAGING_UNAVAILABLE"));
    readiness.store(true, Ordering::Release);
    assert_eq!(
        router
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("request")
            )
            .await
            .expect("ready")
            .status()
            .as_u16(),
        200
    );
    fixture.close().await;
}

#[tokio::test]
async fn http_timeout_does_not_claim_rollback_or_repeat_an_uncertain_write() {
    let _guard = TEST_LOCK.lock().await;
    let fixture = PostgresFixture::start().await;
    let server = security_fixture::JwksServer::start("key-1").await;
    let security = security_fixture::security_config(&server).await;
    let router = humanizar_nucleo::infrastructure::controller::create_router_with_limits(
        ApplicationState::from_database(fixture.database.clone(), Arc::new(AtomicBool::new(true))),
        &security,
        1024,
        std::time::Duration::from_millis(250),
    );
    let token = security_fixture::rsa_token("key-1", &security_fixture::TestClaims::valid());
    let id = Uuid::new_v4();
    fixture
        .client
        .execute(
            "INSERT INTO nucleo (id, nome, descricao) VALUES ($1, 'Before', 'D')",
            &[&id],
        )
        .await
        .expect("parent");
    fixture.client.batch_execute("CREATE TABLE mutation_probe (executions INTEGER); CREATE FUNCTION record_mutation() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN INSERT INTO mutation_probe VALUES (1); RETURN NEW; END $$; CREATE TRIGGER mutation_probe AFTER UPDATE ON nucleo FOR EACH ROW EXECUTE FUNCTION record_mutation()").await.expect("probe");
    let mut locker = fixture.database.acquire().await.expect("locker");
    let locker_pid: i32 = locker
        .query_one("SELECT pg_backend_pid()", &[])
        .await
        .expect("pid")
        .get(0);
    let transaction = locker.transaction().await.expect("transaction");
    transaction
        .query_one("SELECT id FROM nucleo WHERE id = $1 FOR UPDATE", &[&id])
        .await
        .expect("hold lock");
    let writer = fixture
        .database
        .acquire()
        .await
        .expect("warm writer connection");
    writer
        .query_one("SELECT 1", &[])
        .await
        .expect("writer connection ready");
    drop(writer);
    let warmup = router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/nucleo/{id}"))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .expect("authenticated warmup"),
        )
        .await
        .expect("warmup response");
    assert_eq!(warmup.status().as_u16(), 200);
    let response = router
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/nucleo/update/{id}"))
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"nome":"After", "descricao":"D"}).to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(response.status().as_u16(), 504);
    assert!(
        fixture
            .client
            .query_one(
                "SELECT COUNT(*) FROM pg_stat_activity WHERE pg_blocking_pids(pid) @> ARRAY[$1::INTEGER]",
                &[&locker_pid]
            )
            .await
            .expect("blocked query")
            .get::<_, i64>(0)
            > 0
    );
    transaction.rollback().await.expect("release lock");
    for _ in 0..100 {
        if fixture
            .client
            .query_one("SELECT nome FROM nucleo WHERE id = $1", &[&id])
            .await
            .expect("eventual state")
            .get::<_, String>(0)
            == "After"
        {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
    assert_eq!(
        fixture
            .client
            .query_one("SELECT COUNT(*) FROM mutation_probe", &[])
            .await
            .expect("effects")
            .get::<_, i64>(0),
        1
    );
    drop(locker);
    fixture.close().await;
}
