#![forbid(unsafe_code)]

use futures_util::StreamExt;
use humanizar_nucleo::application::catalog::{ExchangeCatalog, RoutingKeyCatalog};
use humanizar_nucleo::domain::port::messaging::MessagePublisherPort;
use humanizar_nucleo::infrastructure::config::rabbit::RabbitSettings;
use humanizar_nucleo::infrastructure::messaging::MessagingServices;
use humanizar_nucleo::infrastructure::messaging::inbound::rabbit::RabbitRuntime;
use humanizar_nucleo::infrastructure::messaging::outbound::rabbit::RabbitOutboxPublisher;
use lapin::options::{
    BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, BasicRejectOptions,
    ConfirmSelectOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Connection, ConnectionProperties, ExchangeKind};
use postgres_fixture::PostgresFixture;
use serde_json::{Value, json};
use std::time::Duration;
use uuid::Uuid;

#[path = "../../support/postgres_fixture.rs"]
mod postgres_fixture;

#[tokio::test]
async fn real_rabbit_processing_replay_confirms_returns_and_shutdown() {
    let fixture = PostgresFixture::start().await;
    let url = std::env::var("NUCLEO_TEST_RABBIT_URL").expect("executar scripts/run_checks.py");
    assert_eq!(
        url::Url::parse(&url).expect("url").host_str(),
        Some("127.0.0.1")
    );
    let runtime = RabbitRuntime::start(
        fixture.database.clone(),
        RabbitSettings::new(url.clone()).expect("settings"),
    )
    .await
    .expect("runtime");
    let connection = Connection::connect(&url, ConnectionProperties::default())
        .await
        .expect("test connection");
    let channel = connection.create_channel().await.expect("test channel");
    let exchange = format!("test.callbacks.{}", Uuid::new_v4().simple());
    channel
        .exchange_declare(
            exchange.as_str().into(),
            ExchangeKind::Topic,
            ExchangeDeclareOptions {
                auto_delete: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .expect("exchange");
    let queue = channel
        .queue_declare(
            "".into(),
            QueueDeclareOptions {
                exclusive: true,
                auto_delete: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .expect("queue");
    channel
        .queue_bind(
            queue.name().clone(),
            exchange.as_str().into(),
            "ev.nucleo.*.v1".into(),
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("binding");
    let mut callbacks = channel
        .basic_consume(
            queue.name().clone(),
            "test.callbacks".into(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("consumer");
    let id = Uuid::new_v4();
    fixture
        .client
        .execute(
            "INSERT INTO nucleo (id, nome, descricao) VALUES ($1, 'Nucleo', 'D')",
            &[&id],
        )
        .await
        .expect("fixture row");
    let event_id = Uuid::new_v4();
    let correlation_id = Uuid::new_v4();
    let source_event_id = Uuid::new_v4();
    let notification_request_id = Uuid::new_v4();
    assert_ne!(notification_request_id, id);
    let command = json!({
        "eventId": event_id,
        "correlationId": correlation_id,
        "producerService": "humanizar-notification",
        "exchangeName": ExchangeCatalog::NUCLEO_COMMAND,
        "routingKey": RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
        "aggregateType": "NotificationRequest",
        "aggregateId": notification_request_id,
        "eventVersion": 1,
        "occurredAt": "2026-10-13T00:00:00",
        "actorId": null,
        "userAgent": null,
        "originIp": null,
        "payload": {
            "sourceEventId": source_event_id,
            "nucleoId": id,
            "callbackExchange": exchange,
            "callbackRoutingKey": RoutingKeyCatalog::NUCLEO_RETRIEVED_V1
        }
    });
    channel
        .confirm_select(ConfirmSelectOptions::default())
        .await
        .expect("confirm mode");
    let confirmation = channel
        .basic_publish(
            ExchangeCatalog::NUCLEO_COMMAND.into(),
            RoutingKeyCatalog::NUCLEO_RETRIEVE_V1.into(),
            BasicPublishOptions {
                mandatory: true,
                ..Default::default()
            },
            command.to_string().as_bytes(),
            BasicProperties::default().with_delivery_mode(2),
        )
        .await
        .expect("publish")
        .await
        .expect("confirm");
    assert!(confirmation.is_ack());
    let delivery = tokio::time::timeout(Duration::from_secs(15), callbacks.next())
        .await
        .expect("callback deadline")
        .expect("callback")
        .expect("delivery");
    let payload: Value = serde_json::from_slice(&delivery.data).expect("JSON callback");
    assert_eq!(payload["aggregateType"], "Nucleo");
    assert_eq!(payload["aggregateId"], id.to_string());
    assert_eq!(payload["payload"]["nucleoId"], id.to_string());
    assert_eq!(payload["payload"]["requestEventId"], event_id.to_string());
    assert_eq!(
        payload["payload"]["sourceEventId"],
        source_event_id.to_string()
    );
    assert_eq!(payload["payload"]["nome"], "Nucleo");
    assert_eq!(payload["correlationId"], correlation_id.to_string());
    assert_eq!(
        delivery
            .properties
            .correlation_id()
            .as_ref()
            .expect("AMQP correlation")
            .as_str(),
        correlation_id.to_string()
    );
    assert_eq!(
        delivery
            .properties
            .message_id()
            .as_ref()
            .expect("AMQP event")
            .as_str(),
        payload["eventId"].as_str().expect("event ID")
    );
    delivery
        .reject(BasicRejectOptions { requeue: true })
        .await
        .expect("simulate lost ACK/redelivery");
    let redelivery = tokio::time::timeout(Duration::from_secs(5), callbacks.next())
        .await
        .expect("redelivery deadline")
        .expect("redelivery")
        .expect("delivery");
    assert!(redelivery.redelivered);
    assert_eq!(redelivery.data, delivery.data);
    redelivery
        .ack(BasicAckOptions::default())
        .await
        .expect("ACK");
    channel
        .basic_publish(
            ExchangeCatalog::NUCLEO_COMMAND.into(),
            RoutingKeyCatalog::NUCLEO_RETRIEVE_V1.into(),
            BasicPublishOptions::default(),
            command.to_string().as_bytes(),
            BasicProperties::default(),
        )
        .await
        .expect("duplicate publish")
        .await
        .expect("duplicate confirm");
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_eq!(
        fixture
            .client
            .query_one(
                "SELECT COUNT(*) FROM processed_event WHERE event_id = $1",
                &[&event_id]
            )
            .await
            .expect("receipt")
            .get::<_, i64>(0),
        1
    );
    assert_eq!(
        fixture
            .client
            .query_one("SELECT COUNT(*) FROM outbox_event", &[])
            .await
            .expect("outbox")
            .get::<_, i64>(0),
        1
    );
    let invalid_event_id = Uuid::new_v4();
    let mut invalid = command.clone();
    invalid["eventId"] = json!(invalid_event_id);
    invalid["payload"]["callbackExchange"] = json!("é".repeat(128));
    channel
        .basic_publish(
            ExchangeCatalog::NUCLEO_COMMAND.into(),
            RoutingKeyCatalog::NUCLEO_RETRIEVE_V1.into(),
            BasicPublishOptions::default(),
            invalid.to_string().as_bytes(),
            BasicProperties::default(),
        )
        .await
        .expect("invalid publication")
        .await
        .expect("confirm");
    let mut recorded = false;
    for _ in 0..100 {
        if let Some(row) = fixture
            .client
            .query_opt(
                "SELECT result, reason_code FROM processed_event WHERE event_id = $1",
                &[&invalid_event_id],
            )
            .await
            .expect("receipt")
        {
            assert_eq!(row.get::<_, String>("result"), "FAILED");
            assert_eq!(row.get::<_, String>("reason_code"), "VALIDATION_ERROR");
            recorded = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert!(recorded);
    assert_eq!(
        fixture
            .client
            .query_one("SELECT COUNT(*) FROM outbox_event", &[])
            .await
            .expect("no invalid callback")
            .get::<_, i64>(0),
        1
    );
    runtime.shutdown().await.expect("drained shutdown");
    let services = MessagingServices::new(fixture.database.clone(), 5);
    fixture
        .client
        .batch_execute(
            "UPDATE outbox_event SET status = 'NEW', locked_by = NULL, next_retry_at = NULL",
        )
        .await
        .expect("uncertain local completion fixture");
    let now = chrono::Local::now().naive_local();
    let owner = Uuid::new_v4();
    let batch = services
        .outbox
        .claim(owner, 20, now, now + chrono::Duration::seconds(300))
        .await
        .expect("replay claim");
    let publisher = RabbitOutboxPublisher::new(channel.clone(), Duration::from_secs(5))
        .await
        .expect("publisher");
    publisher
        .publish(&batch[0])
        .await
        .expect("replay same persisted intention");
    let replay = tokio::time::timeout(Duration::from_secs(5), callbacks.next())
        .await
        .expect("replay deadline")
        .expect("replay")
        .expect("delivery");
    assert_eq!(replay.data, delivery.data);
    replay
        .ack(BasicAckOptions::default())
        .await
        .expect("replay ACK");
    let message = humanizar_nucleo::domain::model::messaging::OutboxMessage::new(
        999,
        Uuid::new_v4(),
        correlation_id,
        &exchange,
        "no.binding",
        "{}",
        0,
        5,
    );
    assert!(publisher.publish(&message).await.is_err());
    let oversized = humanizar_nucleo::domain::model::messaging::OutboxMessage::new(
        1000,
        Uuid::new_v4(),
        correlation_id,
        "é".repeat(128),
        "key",
        "{}",
        0,
        5,
    );
    assert!(publisher.publish(&oversized).await.is_err());
    assert!(publisher.is_available());
    channel
        .close(200, "closed fixture".into())
        .await
        .expect("close channel");
    assert!(!publisher.is_available());
    assert!(publisher.publish(&message).await.is_err());
    let restarted = RabbitRuntime::start(
        fixture.database.clone(),
        RabbitSettings::new(url).expect("settings"),
    )
    .await
    .expect("restart");
    assert!(
        restarted
            .readiness()
            .load(std::sync::atomic::Ordering::Acquire)
    );
    restarted.shutdown().await.expect("restart shutdown");
    connection
        .close(200, "fixture done".into())
        .await
        .expect("close fixture");
    fixture.close().await;
}
