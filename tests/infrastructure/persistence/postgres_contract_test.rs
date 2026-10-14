#![forbid(unsafe_code)]

use axum::body::Body;
use axum::http::Request;
use http_body_util::BodyExt;
use humanizar_nucleo::application::catalog::{ExchangeCatalog, RoutingKeyCatalog};
use humanizar_nucleo::application::messaging::inbound::mapper::NucleoCommandMapper;
use humanizar_nucleo::domain::model::messaging::{NucleoRetrieveCommand, ProcessingOutcome};
use humanizar_nucleo::infrastructure::controller::{ApplicationState, create_router};
use humanizar_nucleo::infrastructure::messaging::MessagingServices;
use postgres_fixture::PostgresFixture;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio_postgres::error::SqlState;
use tower::ServiceExt;
use uuid::Uuid;

#[path = "../../support/postgres_fixture.rs"]
mod postgres_fixture;
#[path = "../../support/security_fixture.rs"]
mod security_fixture;

#[tokio::test]
async fn real_postgres_preserves_crud_and_transactional_messaging_under_concurrency() {
    let fixture = PostgresFixture::start().await;
    let server = security_fixture::JwksServer::start("key-1").await;
    let security = security_fixture::security_config(&server).await;
    let router = create_router(
        ApplicationState::from_database(
            fixture.database.clone(),
            Arc::new(std::sync::atomic::AtomicBool::new(true)),
        ),
        &security,
    );
    let token = security_fixture::rsa_token("key-1", &security_fixture::TestClaims::valid());
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/nucleo/register")
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(
            json!({"nome":"Núcleo", "descricao":"Descrição"}).to_string(),
        ))
        .expect("request");
    let response = router.clone().oneshot(request).await.expect("response");
    assert_eq!(response.status().as_u16(), 201);
    let body: Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes(),
    )
    .expect("JSON");
    let id = Uuid::parse_str(body["data"]["id"].as_str().expect("id")).expect("uuid");
    let civil =
        chrono::NaiveDateTime::parse_from_str("2021-02-03T14:15:16.123456", "%Y-%m-%dT%H:%M:%S%.f")
            .expect("civil");
    fixture
        .client
        .execute(
            "UPDATE nucleo SET created_at = $2 WHERE id = $1",
            &[&id, &civil],
        )
        .await
        .expect("historic fixture");
    let update = Request::builder()
        .method("PUT")
        .uri(format!("/api/v1/nucleo/update/{id}"))
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(
            json!({"nome":"Updated", "descricao":"D"}).to_string(),
        ))
        .expect("request");
    let response = router.oneshot(update).await.expect("response");
    assert_eq!(response.status().as_u16(), 200);
    let body: Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes(),
    )
    .expect("JSON");
    assert_eq!(body["data"]["createdAt"], "2021-02-03T14:15:16.123456");
    let services = MessagingServices::new(fixture.database.clone(), 5);
    let notification_request_id = Uuid::new_v4();
    assert_ne!(notification_request_id, id);
    let envelope = json!({
        "eventId": Uuid::new_v4(),
        "correlationId": Uuid::new_v4(),
        "producerService": "humanizar-notification",
        "exchangeName": ExchangeCatalog::NUCLEO_COMMAND,
        "routingKey": RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
        "aggregateType": "NotificationRequest",
        "aggregateId": notification_request_id,
        "eventVersion": 1,
        "occurredAt": civil,
        "actorId": Uuid::new_v4(),
        "userAgent": "fixture-agent",
        "originIp": "127.0.0.1",
        "payload": {
            "sourceEventId": Uuid::new_v4(),
            "nucleoId": id,
            "callbackExchange": "test.callbacks",
            "callbackRoutingKey": RoutingKeyCatalog::NUCLEO_RETRIEVED_V1
        }
    });
    let command = NucleoCommandMapper::parse(
        &serde_json::to_vec(&envelope).expect("notification envelope"),
        ExchangeCatalog::NUCLEO_COMMAND,
        RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
    )
    .expect("notification command");
    let mut tasks = tokio::task::JoinSet::new();
    for _ in 0..12 {
        let port = Arc::clone(&services.processed);
        let command = command.clone();
        tasks.spawn(async move { port.process(command).await.expect("process") });
    }
    let mut processed = 0;
    while let Some(result) = tasks.join_next().await {
        if result.expect("task") == ProcessingOutcome::Processed {
            processed += 1;
        }
    }
    assert_eq!(processed, 1);
    let receipt = fixture
        .client
        .query_one(
            "SELECT response_event_id FROM processed_event WHERE event_id = $1",
            &[&command.event_id],
        )
        .await
        .expect("receipt");
    let callback_id: Uuid = receipt.get("response_event_id");
    let payload: String = fixture
        .client
        .query_one(
            "SELECT payload FROM outbox_event WHERE event_id = $1",
            &[&callback_id],
        )
        .await
        .expect("event")
        .get("payload");
    let callback: Value = serde_json::from_str(&payload).expect("callback");
    assert_eq!(
        callback["actorId"],
        command.actor_id.expect("actor").to_string()
    );
    assert_eq!(callback["userAgent"], "fixture-agent");
    assert_eq!(callback["originIp"], "127.0.0.1");
    assert_eq!(callback["aggregateType"], "Nucleo");
    assert_eq!(callback["aggregateId"], id.to_string());
    assert_eq!(callback["payload"]["nucleoId"], id.to_string());
    assert_eq!(
        callback["payload"]["sourceEventId"],
        command.source_event_id.to_string()
    );
    assert_eq!(
        callback["payload"]["requestEventId"],
        command.event_id.to_string()
    );
    assert_eq!(callback["payload"]["nome"], "Updated");
    assert_eq!(
        callback["correlationId"],
        command.correlation_id.to_string()
    );
    assert_eq!(
        services
            .processed
            .process(command.clone())
            .await
            .expect("lost ACK replay"),
        ProcessingOutcome::Duplicate
    );
    fixture
        .client
        .batch_execute(
            "ALTER TABLE processed_event ADD CONSTRAINT reject_fixture CHECK (false) NOT VALID",
        )
        .await
        .expect("fault injection");
    let mut failed = command.clone();
    failed.event_id = Uuid::new_v4();
    let count: i64 = fixture
        .client
        .query_one("SELECT COUNT(*) FROM outbox_event", &[])
        .await
        .expect("count")
        .get(0);
    assert!(services.processed.process(failed).await.is_err());
    assert_eq!(
        fixture
            .client
            .query_one("SELECT COUNT(*) FROM outbox_event", &[])
            .await
            .expect("count")
            .get::<_, i64>(0),
        count
    );
    fixture
        .client
        .batch_execute("ALTER TABLE processed_event DROP CONSTRAINT reject_fixture")
        .await
        .expect("remove fault");
    let now = chrono::Local::now().naive_local();
    let owner = Uuid::new_v4();
    let claimed = services
        .outbox
        .claim(owner, 20, now, now + chrono::Duration::seconds(300))
        .await
        .expect("claim");
    assert_eq!(claimed.len(), 1);
    let message = &claimed[0];
    let other = Uuid::new_v4();
    assert!(
        !services
            .outbox
            .complete(message, other, now)
            .await
            .expect("fencing")
    );
    fixture
        .client
        .execute(
            "UPDATE outbox_event SET next_retry_at = $1",
            &[&(now - chrono::Duration::seconds(1))],
        )
        .await
        .expect("expire lease");
    let reclaimed = services
        .outbox
        .claim(other, 20, now, now + chrono::Duration::seconds(300))
        .await
        .expect("reclaim");
    assert_eq!(reclaimed[0].event_id(), message.event_id());
    assert_eq!(reclaimed[0].payload(), message.payload());
    assert!(
        !services
            .outbox
            .complete(message, owner, now)
            .await
            .expect("old owner")
    );
    assert!(
        services
            .outbox
            .fail(&reclaimed[0], other, now)
            .await
            .expect("retry")
    );
    for _ in 0..4 {
        let batch = services
            .outbox
            .claim(other, 20, now, now + chrono::Duration::seconds(300))
            .await
            .expect("retry claim");
        assert_eq!(batch[0].event_id(), callback_id);
        services
            .outbox
            .fail(&batch[0], other, now)
            .await
            .expect("retry failure");
    }
    assert_eq!(
        fixture
            .client
            .query_one(
                "SELECT status FROM outbox_event WHERE event_id = $1",
                &[&callback_id]
            )
            .await
            .expect("dead")
            .get::<_, String>(0),
        "DEAD"
    );
    fixture.close().await;
}

#[tokio::test]
async fn initial_schema_enforces_relationships_messaging_constraints_and_complete_indexes() {
    let fixture = PostgresFixture::start().await;
    let id = Uuid::new_v4();
    fixture
        .client
        .execute(
            "INSERT INTO nucleo (id, nome, descricao) VALUES ($1, 'Parent', 'D')",
            &[&id],
        )
        .await
        .expect("parent");
    let duplicate_parent = fixture
        .client
        .execute(
            "INSERT INTO nucleo (id, nome, descricao) VALUES ($1, 'Duplicate', 'D')",
            &[&id],
        )
        .await
        .expect_err("primary key rejects duplicates");
    assert_eq!(duplicate_parent.code(), Some(&SqlState::UNIQUE_VIOLATION));
    for _ in 0..2 {
        fixture
            .client
            .execute(
                "INSERT INTO abordagem (id, nome, descricao, nucleo_id) VALUES ($1, 'Child', 'D', $2)",
                &[&Uuid::new_v4(), &id],
            )
            .await
            .expect("one parent accepts multiple children");
    }
    let orphan = fixture
        .client
        .execute(
            "INSERT INTO abordagem (id, nome, descricao, nucleo_id) VALUES ($1, 'Orphan', 'D', $2)",
            &[&Uuid::new_v4(), &Uuid::new_v4()],
        )
        .await
        .expect_err("foreign key rejects orphans");
    assert_constraint(
        orphan,
        &SqlState::FOREIGN_KEY_VIOLATION,
        "fk_abordagem_nucleo",
    );
    let missing_parent = fixture
        .client
        .execute(
            "INSERT INTO abordagem (id, nome, descricao, nucleo_id) VALUES ($1, 'Missing', 'D', NULL)",
            &[&Uuid::new_v4()],
        )
        .await
        .expect_err("parent is mandatory");
    assert_eq!(missing_parent.code(), Some(&SqlState::NOT_NULL_VIOLATION));
    let deletion = fixture
        .client
        .execute("DELETE FROM nucleo WHERE id = $1", &[&id])
        .await
        .expect_err("parent with children cannot be deleted");
    assert!(matches!(
        deletion.code().map(SqlState::code),
        Some("23001" | "23503")
    ));
    assert_eq!(
        deletion
            .as_db_error()
            .expect("foreign key restriction")
            .constraint(),
        Some("fk_abordagem_nucleo")
    );
    assert_eq!(
        fixture
            .client
            .query_one(
                "SELECT COUNT(*) FROM abordagem WHERE nucleo_id = $1",
                &[&id]
            )
            .await
            .expect("children retained")
            .get::<_, i64>(0),
        2
    );
    let services = MessagingServices::new(fixture.database.clone(), 5);
    let mut source_events = Vec::new();
    for _ in 0..2 {
        let event_id = Uuid::new_v4();
        source_events.push(event_id);
        services
            .processed
            .process(NucleoRetrieveCommand {
                event_id,
                correlation_id: Uuid::new_v4(),
                source_event_id: Uuid::new_v4(),
                nucleo_id: id,
                callback_exchange: "fixture.callbacks".to_owned(),
                actor_id: None,
                user_agent: None,
                origin_ip: None,
                occurred_at: chrono::Local::now().naive_local(),
            })
            .await
            .expect("valid event and receipt");
    }
    let duplicate_outbox = fixture
        .client
        .execute(
            "UPDATE outbox_event SET event_id = (SELECT event_id FROM outbox_event WHERE id = 1) WHERE id = 2",
            &[],
        )
        .await
        .expect_err("outbox event must be unique");
    assert_constraint(
        duplicate_outbox,
        &SqlState::UNIQUE_VIOLATION,
        "uk_outbox_event_id",
    );
    let duplicate_receipt = fixture
        .client
        .execute(
            "UPDATE processed_event SET event_id = $1 WHERE event_id = $2",
            &[&source_events[0], &source_events[1]],
        )
        .await
        .expect_err("receipt is unique per consumer and event");
    assert_constraint(
        duplicate_receipt,
        &SqlState::UNIQUE_VIOLATION,
        "uk_processed_consumer_event",
    );
    fixture.client.execute("INSERT INTO processed_event (consumer_name, event_id, correlation_id, event_type, aggregate_type, aggregate_id, processed_at, result) SELECT 'fixture.other.consumer', event_id, correlation_id, event_type, aggregate_type, aggregate_id, processed_at, result FROM processed_event WHERE event_id = $1", &[&source_events[0]]).await.expect("another consumer accepts the same event");
    for (statement, constraint) in [
        (
            "UPDATE outbox_event SET event_version = 0",
            "ck_outbox_event_version",
        ),
        (
            "UPDATE outbox_event SET status = 'INVALID'",
            "ck_outbox_status",
        ),
        (
            "UPDATE outbox_event SET attempt_count = -1",
            "ck_outbox_attempt_count",
        ),
        (
            "UPDATE outbox_event SET max_attempts = 0",
            "ck_outbox_max_attempts",
        ),
        (
            "UPDATE processed_event SET result = 'INVALID'",
            "ck_processed_result",
        ),
    ] {
        let error = fixture
            .client
            .execute(statement, &[])
            .await
            .expect_err("check rejects invalid value");
        assert_constraint(error, &SqlState::CHECK_VIOLATION, constraint);
    }
    let primary_keys: i64 = fixture.client.query_one("SELECT COUNT(*) FROM pg_constraint WHERE contype = 'p' AND conrelid IN ('nucleo'::regclass, 'abordagem'::regclass, 'outbox_event'::regclass, 'processed_event'::regclass) AND conkey = ARRAY[(SELECT attnum FROM pg_attribute WHERE attrelid = conrelid AND attname = 'id')]::SMALLINT[]", &[]).await.expect("primary keys").get(0);
    assert_eq!(primary_keys, 4);
    let indexes = fixture.client.query(r#"
        SELECT index_relation.relname, ARRAY_AGG(attribute.attname::TEXT ORDER BY key.ordinality) AS columns
        FROM pg_index AS table_index
        JOIN pg_class AS index_relation ON index_relation.oid = table_index.indexrelid
        CROSS JOIN UNNEST(table_index.indkey) WITH ORDINALITY AS key(number, ordinality)
        JOIN pg_attribute AS attribute ON attribute.attrelid = table_index.indrelid AND attribute.attnum = key.number
        WHERE table_index.indrelid IN ('nucleo'::regclass, 'abordagem'::regclass, 'outbox_event'::regclass, 'processed_event'::regclass)
            AND NOT table_index.indisprimary AND NOT table_index.indisunique
            AND table_index.indisvalid AND table_index.indpred IS NULL AND table_index.indexprs IS NULL
        GROUP BY index_relation.relname
        ORDER BY index_relation.relname
    "#, &[]).await.expect("index definitions");
    let actual_indexes: Vec<(String, Vec<String>)> = indexes
        .into_iter()
        .map(|row| (row.get(0), row.get(1)))
        .collect();
    let expected_indexes: Vec<(String, Vec<String>)> = [
        ("idx_abordagem_nucleo", vec!["nucleo_id", "nome", "id"]),
        (
            "idx_outbox_aggregate",
            vec!["aggregate_type", "aggregate_id", "created_at"],
        ),
        ("idx_outbox_correlation", vec!["correlation_id"]),
        (
            "idx_outbox_dispatch",
            vec!["status", "next_retry_at", "created_at", "id"],
        ),
        ("idx_processed_correlation", vec!["correlation_id"]),
        ("idx_processed_event", vec!["event_id"]),
        ("idx_processed_reason_code", vec!["reason_code"]),
        ("idx_processed_time", vec!["processed_at", "id"]),
    ]
    .into_iter()
    .map(|(name, columns)| {
        (
            name.to_owned(),
            columns.into_iter().map(str::to_owned).collect(),
        )
    })
    .collect();
    assert_eq!(actual_indexes, expected_indexes);
    let server = security_fixture::JwksServer::start("key-1").await;
    let security = security_fixture::security_config(&server).await;
    let router = create_router(
        ApplicationState::from_database(
            fixture.database.clone(),
            Arc::new(std::sync::atomic::AtomicBool::new(true)),
        ),
        &security,
    );
    let token = security_fixture::rsa_token("key-1", &security_fixture::TestClaims::valid());
    let racing_parent = Uuid::new_v4();
    fixture
        .client
        .execute(
            "INSERT INTO nucleo (id, nome, descricao) VALUES ($1, 'Race', 'D')",
            &[&racing_parent],
        )
        .await
        .expect("parent without visible children");
    fixture.client.batch_execute("CREATE FUNCTION create_child_during_delete() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN INSERT INTO abordagem (id, nome, descricao, nucleo_id) VALUES (gen_random_uuid(), 'Race', 'D', OLD.id); RETURN OLD; END $$; CREATE TRIGGER create_child_during_delete BEFORE DELETE ON nucleo FOR EACH ROW EXECUTE FUNCTION create_child_during_delete()").await.expect("foreign key restriction after the application precheck");
    let response = router
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/nucleo/delete/{racing_parent}"))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .expect("delete request"),
        )
        .await
        .expect("controlled conflict response");
    assert_eq!(response.status().as_u16(), 409);
    let body: Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("error body")
            .to_bytes(),
    )
    .expect("error JSON");
    assert_eq!(body["reasonCode"], "NUCLEO_HAS_ABORDAGENS");
    assert_eq!(
        fixture
            .client
            .query_one(
                "SELECT COUNT(*) FROM nucleo WHERE id = $1",
                &[&racing_parent]
            )
            .await
            .expect("parent retained after restricted deletion")
            .get::<_, i64>(0),
        1
    );
    assert_eq!(
        fixture
            .client
            .query_one(
                "SELECT COUNT(*) FROM abordagem WHERE nucleo_id = $1",
                &[&racing_parent]
            )
            .await
            .expect("trigger insertion rolled back")
            .get::<_, i64>(0),
        0
    );
    let reinstall = fixture
        .client
        .batch_execute(include_str!("../../../migrations/0001_initial_schema.sql"))
        .await
        .expect_err("initial schema requires an empty database");
    assert_eq!(reinstall.code(), Some(&SqlState::DUPLICATE_TABLE));
    fixture
        .client
        .batch_execute("ROLLBACK")
        .await
        .expect("rollback");
    assert_eq!(
        fixture
            .client
            .query_one("SELECT nome FROM nucleo WHERE id = $1", &[&id])
            .await
            .expect("existing data retained")
            .get::<_, String>(0),
        "Parent"
    );
    fixture.close().await;
}

fn assert_constraint(error: tokio_postgres::Error, expected_state: &SqlState, expected_name: &str) {
    assert_eq!(error.code(), Some(expected_state));
    assert_eq!(
        error
            .as_db_error()
            .expect("database constraint error")
            .constraint(),
        Some(expected_name)
    );
}
